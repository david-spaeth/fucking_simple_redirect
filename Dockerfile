FROM rust:alpine
RUN adduser -S fucking_user
WORKDIR /usr/src/fucking_simple_redirect
COPY . src  
RUN cd src/ && cargo install --path .  && cd ../  && rm -fr src/
USER fucking_user
ENV FUCKING_CONFIG=/usr/src/fucking_simple_redirect/domains.config
CMD ["fucking_simple_redirect"]
expose 8080/tcp
