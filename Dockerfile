FROM debian:latest

COPY ./target/release/rename-bot /app/rename-bot
WORKDIR /app

ENTRYPOINT ["/app/rename-bot"]