FROM rust:latest

COPY . .

RUN cargo build --release

EXPOSE 80

ENTRYPOINT ["./target/release/nginx", "/etc/httpd.conf"]