FROM rust:alpine
RUN adduser -S fucking_user
USER fucking_user
WORKDIR /usr/src/fucking_simple_redirect
COPY . .

RUN cargo install --path .

CMD ["fucking_simple_redirect"]