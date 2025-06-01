# Leyak

This repository contains a simple microservice playground. The stack is orchestrated with Docker Compose and uses PostgreSQL and RabbitMQ.

## Getting started

1. Copy `.env.sample` to `.env` and adjust the values if necessary. The file contains
   defaults such as `POSTGRES_VERSION` and `RABBITMQ_VERSION` which control the container
   images used by Docker Compose.
2. Create a directory called `.secrets` in the project root with the following files:
   - `postgres_password` – the password for the PostgreSQL service.
   - `jwt_secret` – secret used for signing JWT tokens.
3. Run the services with `docker compose up --build`.

Environment variables for non‑sensitive values can be tweaked in `.env`. Secrets are loaded from the files above.
