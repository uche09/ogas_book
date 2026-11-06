# Oga's Book API

The Oga's Book API is a Rust service built with Axum and PostgreSQL. This guide is for API consumers and teammates who want to run the service locally, especially frontend developers integrating the current authentication endpoints.

## Current API

Base URL for local development: `http://localhost:3000`

| Method | Path           | Purpose                                   |
| ------ | -------------- | ----------------------------------------- |
| `GET`  | `/health`      | Check that the API is responding.         |
| `POST` | `/auth/signup` | Create a non-admin user account.          |
| `POST` | `/auth/login`  | Authenticate a user and create a session. |

There is no refresh endpoint or protected business API currently mounted. The login response contains a short-lived user token, but the API does not yet define an `Authorization` header contract for protected routes.

## Prerequisites

- Rust stable and Cargo
- Docker with the Compose plugin, or a PostgreSQL 16 database
- `curl` or Postman for making requests
- `sqlx-cli` 0.9.0 for applying migrations before compiling against a fresh database

## Run Locally

Run these commands from the `api/` directory:

```sh
cp .env.example .env
docker compose up -d postgres
cargo install sqlx-cli --version 0.9.0 --no-default-features --features postgres,rustls --locked
sqlx migrate run
cargo run
```

The API also applies migrations on startup. Running `sqlx migrate run` first is still necessary when the database is fresh: SQLx validates the API's database queries during compilation, before `main` can run.

Check that the server is ready:

```sh
curl http://localhost:3000/health
```

Expected response:

```json
{
  "status": "ok",
  "message": "Server is running"
}
```

Stop PostgreSQL with `docker compose down`. The named volume preserves local data. To delete the database volume as well, run `docker compose down -v`.

### Configuration

The API reads `DATABASE_URL` as its PostgreSQL connection setting. Docker Compose reads `POSTGRES_USER`, `POSTGRES_PASSWORD`, and `POSTGRES_PORT` to initialize and expose the local database. Copy `.env.example` to `.env` before starting the database. Keep real credentials in the ignored `.env` file, not in `docker-compose.yml`.

| Variable            | Required    | Description                                                                                          |
| ------------------- | ----------- | ---------------------------------------------------------------------------------------------------- |
| `API_PORT`          | No          | HTTP listen port. Defaults to `3000`.                                                                |
| `DATABASE_URL`      | Yes         | PostgreSQL connection string used by the API and SQLx.                                               |
| `POSTGRES_USER`     | For Compose | PostgreSQL user used when initializing the local container.                                          |
| `POSTGRES_PASSWORD` | For Compose | PostgreSQL password used when initializing the local container. Use a private value when needed.     |
| `POSTGRES_PORT`     | For Compose | Host port mapped to PostgreSQL's container port `5432`.                                              |
| `JWT_SECRET`        | Recommended | Secret used to sign user tokens. Set a private, sufficiently random value outside local development. |
| `RUST_LOG`          | No          | Tracing filter; defaults to `info`.                                                                  |

`DATABASE_URL` must match the Compose settings: `postgresql://<POSTGRES_USER>:<POSTGRES_PASSWORD>@localhost:<POSTGRES_PORT>/ogas_book`. The example values are for local development only. Do not commit `.env` or production secrets.

## Authentication

All request and response bodies use JSON. Send `Content-Type: application/json` with POST requests.

### Sign Up

`POST /auth/signup`

Request:

```json
{
  "first_name": "Ada",
  "last_name": "Lovelace",
  "other_name": null,
  "email": "ada@example.com",
  "password": "Strong!Pass1"
}
```

`other_name` is optional and may be omitted or set to `null`. Do not send `is_admin`; the endpoint always creates a regular user, even if a value is supplied.

On success, the API returns `201 Created`:

```json
{
  "first_name": "ada",
  "email": "ada@example.com"
}
```

Names and email are trimmed and lowercased before being stored. First and last names must be 3-70 characters. If provided, `other_name` must also be 3-70 characters. Passwords must be at least 8 characters long and include an uppercase letter, a lowercase letter, a digit, and an ASCII punctuation character.

An existing email returns `422 Unprocessable Entity`.

### Log In

`POST /auth/login`

Request:

```json
{
  "email": "ada@example.com",
  "password": "Strong!Pass1"
}
```

On success, the API returns `200 OK` with a user token and the user's business memberships:

```json
{
  "user_token": "<signed-jwt>",
  "memberships": []
}
```

The user token expires after 5 minutes. The response also sets a `refresh` cookie with `HttpOnly`, `Secure`, `SameSite=Lax`, and `Path=/auth` attributes. Frontend JavaScript cannot read this cookie. The cookie is marked `Secure`; use HTTPS for environments where the browser requires secure transport.

The email is trimmed and lowercased before lookup. Invalid credentials return `401 Unauthorized`.

### Errors

API errors use this JSON shape:

```json
{
  "error": "Unauthorized"
}
```

Common statuses:

| Status | Meaning                                                      |
| ------ | ------------------------------------------------------------ |
| `401`  | Login credentials are invalid.                               |
| `422`  | Request validation failed or signup email is already in use. |
| `500`  | An unexpected server or database error occurred.             |

## Example Requests

Check health:

```sh
curl -i http://localhost:3000/health
```

Create an account:

```sh
curl -i http://localhost:3000/auth/signup \
	-H 'Content-Type: application/json' \
	-d '{"first_name":"Ada","last_name":"Lovelace","email":"ada@example.com","password":"Strong!Pass1"}'
```

Log in and save the refresh cookie:

```sh
curl -i -c cookies.txt http://localhost:3000/auth/login \
	-H 'Content-Type: application/json' \
	-d '{"email":"ada@example.com","password":"Strong!Pass1"}'
```

## Frontend Integration Notes

- The API currently has no CORS middleware configured. A frontend served from another origin may need a development proxy or same-origin setup; adding `credentials: "include"` alone does not configure server-side CORS.
- The refresh cookie is `HttpOnly`, so browser code should not attempt to read it. The browser manages it as a cookie.
- No refresh route or protected route is currently available. Coordinate with the API team before depending on token refresh or a bearer-token authorization convention.

## Tests

Unit tests do not require a database. The HTTP authentication integration test uses `API_TEST_DATABASE_URL`, applies migrations to that database, and creates a test user/session. Use a dedicated test database, not a database with data you need to preserve.

After starting PostgreSQL and applying the development migrations as shown above, run from `api/`. These commands create a separate database for the integration test so it does not add test users or sessions to the development database:

```sh
set -a
. ./.env
set +a
docker compose exec -T postgres createdb -U "$POSTGRES_USER" ogas_book_test
export API_TEST_DATABASE_URL="postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:${POSTGRES_PORT}/ogas_book_test"
cargo test
```
