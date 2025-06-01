# Leyak

This repository contains a simple microservice playground. The stack is orchestrated with Docker Compose and uses PostgreSQL and RabbitMQ.

## Getting started

1. Copy `.env.sample` to `.env` and adjust the values if necessary.
2. Create a directory called `.secrets` in the project root with the following files:
   - `postgres_password` – the password for the PostgreSQL service.
   - `jwt_secret` – secret used for signing JWT tokens.
3. Run the services with `docker compose up --build`.

Environment variables for non‑sensitive values can be tweaked in `.env`. Secrets are loaded from the files above.

Once the containers are running you can confirm each service responds by visiting:

- `http://localhost:8000/` – gateway root
- `http://localhost:8000/health` – gateway health check
- `http://localhost:8001/` – user service root
- `http://localhost:8001/health` – user service health check
- `http://localhost:8002/` – tweet service root
- `http://localhost:8002/health` – tweet service health check
