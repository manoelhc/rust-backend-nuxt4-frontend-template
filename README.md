# rust-backend-nuxt4-frontend-template

A monorepo template with a Rust Axum backend and Nuxt 4 frontend, complete with Docker and Docker Compose configurations.

## Features

### Backend (Rust + Axum)
- **Health Check Endpoint**: `GET /health` - Returns service health status
- **JWT Validation Endpoint**: `POST /validate-token` - Validates JWT tokens
- **Protected System Uptime Endpoint**: `GET /system/uptime` - Returns application uptime (requires valid JWT token)
- JWT authentication middleware for protected routes
- CORS enabled for cross-origin requests

### Frontend (Nuxt 4)
- Minimal Nuxt 4 setup ready for development
- Vue 3 with Composition API
- TypeScript support

## Quick Start

### Prerequisites
- Rust 1.83+ and Cargo
- Node.js 20+ and npm
- Docker and Docker Compose (for containerized deployment)

### Running Locally

#### Backend
```bash
# Navigate to project root
cd rust-backend-nuxt4-frontend-template

# Run the backend (default port: 3000)
cargo run

# Or build for production
cargo build --release
./target/release/rust-backend-template
```

#### Frontend
```bash
# Navigate to frontend directory
cd frontend

# Install dependencies
npm install

# Run development server (default port: 3000)
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview
```

### Running with Docker Compose

```bash
# Build and start both services
docker-compose up --build

# Backend will be available at: http://localhost:3000
# Frontend will be available at: http://localhost:3001
```

## API Endpoints

### Health Check
```bash
curl http://localhost:3000/health
```
Response:
```json
{
  "status": "ok",
  "message": "Service is healthy"
}
```

### Validate JWT Token
```bash
curl -X POST http://localhost:3000/validate-token \
  -H "Content-Type: application/json" \
  -d '{"token":"your-jwt-token"}'
```
Response:
```json
{
  "valid": true,
  "message": "Token is valid"
}
```

### System Uptime (Protected)
Requires a valid JWT token in the Authorization header:
```bash
curl -H "Authorization: Bearer your-jwt-token" \
  http://localhost:3000/system/uptime
```
Response:
```json
{
  "uptime_seconds": 3661,
  "uptime_formatted": "1h 1m 1s"
}
```

## Authentication

The backend uses JWT (JSON Web Tokens) for authentication. By default, the JWT secret is `my-secret-key`, but you can override it using the `JWT_SECRET` environment variable.

### Generating a Test JWT Token

You can use any JWT library to generate tokens. Here's an example using Python:

```python
import jwt
import time

secret = "my-secret-key"
payload = {
    "sub": "test-user",
    "exp": int(time.time()) + 3600  # Expires in 1 hour
}
token = jwt.encode(payload, secret, algorithm="HS256")
print(token)
```

Or using an online tool like [jwt.io](https://jwt.io) with:
- Algorithm: HS256
- Secret: `my-secret-key`
- Payload: `{"sub": "test-user", "exp": <future_timestamp>}`

## Environment Variables

### Backend
- `JWT_SECRET`: Secret key for JWT signing and validation (default: `my-secret-key` - **NOT SECURE FOR PRODUCTION**)
- `PORT`: Port to run the backend server on (default: `3000`)

**Important**: Always set a strong, unique `JWT_SECRET` in production environments. Never use the default secret key.

### Frontend
- `NUXT_PUBLIC_API_URL`: Backend API URL (default: `http://backend:3000` in Docker)

### Setting Up Environment Variables

1. Copy the example environment file:
   ```bash
   cp .env.example .env
   ```

2. Edit `.env` with your values:
   ```bash
   JWT_SECRET=your-secure-secret-key-here
   PORT=3000
   ```

3. When using Docker Compose, create a `.env` file in the project root with your variables, and Docker Compose will automatically load them.

## Project Structure

```
.
├── src/                    # Rust backend source code
│   └── main.rs            # Main application with Axum server
├── frontend/              # Nuxt 4 frontend
│   ├── app/              # Application files
│   ├── public/           # Static assets
│   ├── nuxt.config.ts    # Nuxt configuration
│   └── package.json      # Frontend dependencies
├── Cargo.toml            # Rust dependencies
├── Dockerfile            # Backend Docker configuration
├── docker-compose.yml    # Multi-container orchestration
└── README.md            # This file
```

## Development

### Backend Development
The backend automatically reloads when you make changes if you use `cargo watch`:
```bash
cargo install cargo-watch
cargo watch -x run
```

### Frontend Development
The frontend has hot module replacement (HMR) enabled by default:
```bash
cd frontend
npm run dev
```

## Building for Production

### Backend
```bash
cargo build --release
```
The binary will be in `target/release/rust-backend-template`

### Frontend
```bash
cd frontend
npm run build
```
The build output will be in `frontend/.output/`

## License

This project is licensed under the terms specified in the LICENSE file.
