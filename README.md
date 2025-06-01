# Leyak

This repository contains a simple microservice playground. The stack is orchestrated with Docker Compose and uses PostgreSQL and RabbitMQ.

## Getting started

1. Copy `.env.sample` to `.env` and adjust the values if necessary.
2. Create a directory called `.secrets` in the project root with the following files:
   - `postgres_password` – the password for the PostgreSQL service.
   - `jwt_secret` – secret used for signing JWT tokens.
   - `google_client_id` – OAuth client ID for Google login.
   - `google_client_secret` – OAuth client secret for Google login.
3. Run the services with `docker compose up --build`.

After the stack is running, you can initiate authentication at `http://localhost:8000/auth/google`.
