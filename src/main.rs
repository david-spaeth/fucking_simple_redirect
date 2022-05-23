use std::collections::HashMap;
use std::fs::File;
use std::net::{TcpStream, TcpListener};
use std::io::{Write, BufReader, BufRead};
use std::process::exit;
use std::{thread, env};

#[derive(Clone)]
struct DomainConfig {
    url: String,
    temporarily: bool
}

impl DomainConfig {
    
    pub fn new(url: String, temporarily: bool ) -> Self {
        Self {
            url,
            temporarily,
        }
    }
     fn code(&self) -> &str {
         match self.temporarily {
            true => "302 Found",
            false =>  "301 Moved Permanently"
        }
    }
}

fn get_host(stream: &TcpStream) -> Result<String, std::io::Error> {
    
    let mut reader = BufReader::new(stream);
    let mut buffer = String::new();
    loop {
        let line_result = reader.read_line(&mut buffer);
        if let Ok(size) = line_result {
            if 0 == size {
                break;
            }
            let string_line = buffer.to_string();
            if string_line.to_lowercase().starts_with("host:") {
              let split_result = string_line.split(":").skip(1);
               let host = split_result.map(|s| s.to_string()).reduce(|accum, item| {
                   accum + ":" + &item
                });
                if let Some(url) = host {
                    return Ok(url.trim().to_string());
                }
            }
        } else if let Err(error) = line_result {
            return Err(error);
        } 
        buffer.clear();
    }

    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Host header not found"))
}

fn handle_error(mut stream: &TcpStream) {
    let response = b"HTTP/1.1 418 I'm a teapot";
    let _ =  stream.write(response);
}

fn handle_redirect(mut stream: &TcpStream, url: &String, domains_config: &HashMap<String, DomainConfig>) {
    
    println!("geting url {}", url);    
    if let Some(domain_config) = domains_config.get(url) {
           
       let code = domain_config.code();
        println!("goto {} with {}", domain_config.url, code); 
        let response_string = format!("HTTP/1.1 {}\nServer: fucking_simple_redirect\nLocation: {}", code, domain_config.url);
        let response = response_string.as_bytes();
        match stream.write_all(response).and_then(|_|stream.flush()) {
            Ok(_) =>  {
                println!("Redirect send")
            },
            Err(e) => eprintln!("Failed sending response: {}", e),
        }

    } else {
        handle_error(&stream)
    }
}

fn handle_client(stream: TcpStream, domain_config: HashMap<String, DomainConfig>) {
   match get_host(&stream) {
       Ok(url) => {
        handle_redirect(&stream, &url, &domain_config);
       }
       Err(error) => {
        handle_error(&stream);
        eprintln!("Unable handle request: {}", error);
       }
    }
} 

fn handle_listener(listener: TcpListener, domain_config: HashMap<String, DomainConfig>) {
   
    for stream in listener.incoming() {
        let domain_config_clone = domain_config.clone();
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream, domain_config_clone)
                });
            }
            Err(e) => {
                eprintln!("Unable to connect: {}", e);
            }
        }
    }
}

fn parse_config_line(line: String) -> Option<(String, DomainConfig)> {
    if !line.starts_with("redirect") {
         return Option::None;
    }
    let mut values = line.split(" ");
    values.next();
    let domain_option = values.next();
    let to_check_option = values.next();
    let to_option = values.next();
    let mode_option = values.next();
    if let (Some(domain),Some(to_check), Some(to)) = (domain_option, to_check_option, to_option) {
        if !to_check.trim().eq_ignore_ascii_case("to") {
            return Option::None  
        }
        let temporarily = mode_option.unwrap_or_default().trim().eq_ignore_ascii_case("temp");

        return Option::Some((domain.trim().to_string(), DomainConfig::new(to.trim().to_string(), temporarily)));
    } 
    Option::None

}

fn handle_read_config_file(path: &str) -> Result<HashMap<String, DomainConfig>, std::io::Error> {
    
   let buffer_reader = File::open(path).and_then(|file| Ok(BufReader::new(file)));
   match  buffer_reader {
       Ok (reader) => {
        let mut domains_config: HashMap<String, DomainConfig> = HashMap::new();
        for line_result in reader.lines()  {
            if let Ok(line) = line_result {
                if let Some(infos) = parse_config_line(line) {
                    domains_config.insert(infos.0, infos.1);
                }
            }
        }
         Ok(domains_config)
       } 
       Err(error) => {
        Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Unabled to read config file at {}. {}", path, error.to_string())))
       }
   }
} 

fn main() {
    let file = env::var("FUCKING_CONFIG").unwrap_or("./domains.config".to_string());
    let domain_config = match handle_read_config_file(&file) {
        Ok(domain_config) => domain_config,
        Err(error) => {
            eprintln!("{}", error.to_string());
            exit(1);
        }
    };
    let host = env::var("FUCKING_HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("FUCKING_PORT").unwrap_or("8080".to_string());
    let listen = format!("{}:{}", host, port );
    let listener = TcpListener::bind(listen).unwrap_or_else(|error| {
        eprintln!("Unable to start server: {}", error);
        exit(2);
    });

    if let Ok(local_adrr) = listener.local_addr() {
        println!("Server start on: {} with port: {}", local_adrr.ip(), local_adrr.port());
        handle_listener(listener,domain_config);
    }
}