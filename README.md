# RenameBot \[[Add to Server](https://discord.com/api/oauth2/authorize?client_id=912495511574048799&permissions=275079302144&scope=bot%20applications.commands)\]
[![Build](https://github.com/homie-pile/rename-bot/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/rename-bot/actions/workflows/build.yml)
[![Clippy](https://github.com/homie-pile/rename-bot/actions/workflows/clippy.yml/badge.svg)](https://github.com/Ewpratten/rename-bot/actions/workflows/clippy.yml)

RenameBot is a utility chat bot for the *Homie Pile* Discord server. In this server, all members are administrators which poses some interesting permissions issues when editing eachother's nicknames. As a solution, RenameBot is running with higher permissions than any of the server members, and handles brokering nickname edits between the server members.

## Building

This bot is designed to be run in Docker on a server.

```sh
cargo build --release
docker build -t renamebot .
```

