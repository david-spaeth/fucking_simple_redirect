FROM rust:alpine as builder
WORKDIR /usr/src/fucking_simple_redirect
COPY . src  
RUN cd src/ && cargo build -r  --target-dir .

FROM scratch
WORKDIR /usr/src/fucking_simple_redirect
COPY passwd.minimal /etc/passwd
USER fucking_user
ENV FUCKING_CONFIG=/usr/src/fucking_simple_redirect/domains.config
COPY --from=builder /usr/src/fucking_simple_redirect/src/release/fucking_simple_redirect /usr/src/fucking_simple_redirect/fucking_simple_redirect
CMD ["/usr/src/fucking_simple_redirect/fucking_simple_redirect"]
EXPOSE 8080/tcp
