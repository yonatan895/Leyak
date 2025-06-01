# Leyak

This repository contains a simple microservice playground. The stack is orchestrated with Docker Compose and uses PostgreSQL and RabbitMQ.

## Getting started

1. Copy `.env.sample` to `.env` and adjust the values if necessary.
2. Create a directory called `.secrets` in the project root with the following files:
   - `postgres_password` – the password for the PostgreSQL service.
   - `jwt_secret` – secret used for signing JWT tokens.
3. Set OAuth credentials in `.env` for GitHub login (`GITHUB_CLIENT_ID` and `GITHUB_CLIENT_SECRET`).
3. Run the services with `docker compose up --build`.

Environment variables for non‑sensitive values can be tweaked in `.env`. Secrets are loaded from the files above.

When the stack is running, open `http://localhost:8000` to see the login page. After authenticating with GitHub, you'll be redirected to `/profile` which displays the stored user information from PostgreSQL.
