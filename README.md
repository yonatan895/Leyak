# Leyak

This repository contains a simple microservice playground. The stack is orchestrated with Docker Compose and uses PostgreSQL and RabbitMQ.

## Getting started

1. Review the `.env` file (generated from `.env.sample`) and adjust any values if necessary. It already contains defaults for all non‑secret settings.
2. Create a directory called `.secrets` in the project root with the following files:
   - `postgres_password` – the password for the PostgreSQL service.
   - `jwt_secret` – secret used for signing JWT tokens.
3. Run the services with `docker compose up --build`.

Environment variables for non‑sensitive values are already provided in `.env`. Only the secrets need to be added manually in `.secrets`.
