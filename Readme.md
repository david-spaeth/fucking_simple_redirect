# About

It's a personal project for a personal need but that I would like to find so I share.

It's a simple server in Rust that redirects according to a configuration file.

## Config

The file is expected under `/usr/src/fucking_simple_redirect/domains.config`. You can change the location with the environment variable `FUCKING_CONFIG`

The config file is simple. One redirect per line.
The line must with the following pattern otherwise it will be ignored:

`redirect [HOST] to [URL]`

If the line ends with `temp`, the redirect will be temporary.

`redirect 127.0.0.1:8080 to https://google.com temp`

## Server

The server listens on `127.0.0.1:8080` by default, you can change with `FUCKING_HOST` and `FUCKING_PORT`.
