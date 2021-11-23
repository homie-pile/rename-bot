FROM debian:latest

COPY ./target/release/rename-bot /app/rename-bot
WORKDIR /app

ENTRYPOINT ["RUST_LOG=info", "/app/rename-bot"]