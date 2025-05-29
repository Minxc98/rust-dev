## project readme
# Axum Base Project

A web service template built with Axum framework, featuring user authentication, Swagger documentation, and database integration.

## Features

- REST API endpoints using Axum web framework
- PostgreSQL database integration with SQLx
- User authentication with JWT
- API documentation using Swagger UI
- Structured logging with tracing
- Docker compose setup for development

## Prerequisites

- Rust toolchain
- Docker and Docker Compose
- PostgreSQL client (optional)

## Getting Started

需要安装protoc编译

1. Clone the repository
2. Start the database services:
    1. Start Docker services:
       ```bash
       docker-compose up -d
       ```
    2. Run database migrations:
       ```bash
       cargo sqlx migrate run
       sqlx database create
       sqlx migrate run
       ```
