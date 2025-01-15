# Rust Resume Builder API

A RESTful API built with Rust for managing resume data including experiences, skills, and responsibilities.

## Table of Contents

- [Rust Resume Builder API](#rust-resume-builder-api)
  - [Table of Contents](#table-of-contents)
  - [Prerequisites](#prerequisites)
  - [API Documentation](#api-documentation)
    - [Experiences](#experiences)
    - [Skills](#skills)
    - [Categories](#categories)
  - [Development](#development)
  - [Testing](#testing)

## Prerequisites

Before you begin, ensure you have the following installed:

- Docker
- Docker Compose
- Rust (optional, for local development)

The API will be available at `http://localhost:8000`

## API Documentation

### Experiences

- `GET /experiences` - List all experiences
- `POST /experiences` - Create new experience
- `GET /experiences/{id}` - Get specific experience
- `PUT /experiences/{id}` - Update experience
- `DELETE /experiences/{id}` - Delete experience

### Skills

- `GET /skills` - List all skills
- `POST /skills` - Create new skill

### Categories

- `GET /categories` - List all categories

## Development

- To run the application using Docker:

```bash
docker build -t rust-server .
docker-compose up
```

- To run locally without Docker:

1. Install Rust using [rustup](https://rustup.rs/)
2. Clone the repository
3. Run the development server

```bash
cargo run
```

## Testing

To run tests:

```bash
cargo test
```

For integration tests:

```bash
cargo test -- --test-threads=1
