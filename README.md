# PepperoniBot (rewritten in Rust)
## A personal Discord bot for managing my Discord server.
### Only includes an order system for now.
## Dependencies:
- [Serenity](https://github.com/serenity-rs/serenity) (Discord API)
- [Wither](https://github.com/thedodd/wither) (ODM for MongoDB)
## Deployment with Docker:
### Simply run the following command:
```bash
docker compose up --build -d
```
### This will build the project and run it in a Docker container with a MongoDB instance.
