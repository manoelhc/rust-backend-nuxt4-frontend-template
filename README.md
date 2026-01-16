# Rust Backend + Nuxt 4 Frontend Template

A production-ready monorepo template featuring a Rust Axum backend with PostgreSQL, JWT authentication, and a modern Nuxt 4 frontend with TailwindCSS, Flowbite, and internationalization support.

## 🚀 Features

### Backend (Rust + Axum)
- **RESTful API** with Axum framework
- **PostgreSQL** database with SQLx for type-safe queries
- **JWT Authentication** with email verification and MFA checks
- **OpenTelemetry** integration for observability
- **Auto-registration** system via `/system/onboarding` endpoint
- **User profiles** with customizable properties (JSON storage)
- **Health checks** and version endpoints

#### API Endpoints

**Public:**
- `GET /health` - Service health check
- `GET /system/version` - Application version
- `POST /validate-token` - JWT validation (checks email_verified and mfa_enabled)

**Protected (requires valid JWT):**
- `GET /system/uptime` - System uptime with formatted duration
- `POST /system/onboarding` - Auto-register user from JWT claims
- `GET /profile` - Get user profile information

### Frontend (Nuxt 4 + Vue 3)
- **Modern UI** with TailwindCSS and Flowbite components
- **Responsive layout** with navbar, sidebar, and content areas
- **Dark mode toggle** with system preference detection
- **Internationalization (i18n)** supporting English, Portuguese, and Spanish
- **Language selector** in navbar
- **User profile section** with avatar, preferences, and logout
- **Dashboard** and **Support** pages included
- **Customizable branding** via environment variables

### Infrastructure
- **Docker & Docker Compose** for easy deployment
- **PostgreSQL 18** for data persistence
- **Redis** for caching and sessions
- **Nginx** as API gateway (routes `/api` to backend, rest to frontend)
- **Observability Stack**: Grafana, Loki, Tempo, Mimir for metrics, logs, and traces
- **CI/CD workflows** for automated testing and deployment

## 📋 Prerequisites

- Rust 1.83+ and Cargo
- Node.js 20+ and npm
- Docker and Docker Compose (for containerized deployment)
- PostgreSQL 18+ (if running locally without Docker)

## 🛠️ Quick Start

### Using Docker Compose (Recommended)

```bash
# Clone the repository
git clone <repository-url>
cd rust-backend-nuxt4-frontend-template

# Copy and configure environment variables
cp .env.example .env
# Edit .env with your settings

# Start all services
docker-compose up -d

# Access the application
# Frontend: http://localhost:80
# Backend API: http://localhost:80/api
# Grafana (observability): http://localhost:3030 (admin/admin)
```

### Running Locally

#### Backend

```bash
# Install dependencies and run migrations
cargo build

# Set environment variables
export DATABASE_URL="postgres://postgres:postgres@localhost:5432/app_db"
export JWT_SECRET="your-secret-key"

# Run the backend
cargo run

# The API will be available at http://localhost:3000
```

#### Frontend

```bash
cd frontend

# Install dependencies
npm install

# Set environment variables
export NUXT_PUBLIC_API_URL="http://localhost:3000"
export NUXT_PUBLIC_PROJECT_NAME="My Application"

# Run development server
npm run dev

# The frontend will be available at http://localhost:3000
```

## 🔧 Configuration

### Environment Variables

#### Backend (.env)
```env
JWT_SECRET=your-secure-secret-key-here
PORT=3000
DATABASE_URL=postgres://postgres:postgres@localhost:5432/app_db
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
OTEL_SERVICE_NAME=rust-backend-template
```

#### Frontend
```env
NUXT_PUBLIC_API_URL=http://localhost:3000
NUXT_PUBLIC_PROJECT_NAME=My Application
AI_FRONTEND_DEV=false
NUXT_PUBLIC_ENABLE_URL_AUTH=false
```

### URL-Based Authentication

The frontend supports receiving JWT tokens via URL query parameters for seamless integration with external authentication systems.

**Enable the feature:**
```bash
export NUXT_PUBLIC_ENABLE_URL_AUTH=true
```

**Usage:**
```
http://localhost:3000/?auth-token=your-jwt-token-here
```

**How it works:**
1. When a user accesses the URL with `auth-token` query parameter, the plugin intercepts it
2. The token is stored in a secure cookie named `auth-token` (7-day expiration)
3. All subsequent API calls automatically include the token in the Authorization header
4. The query parameter is removed from the URL for security (optional)

**Security considerations:**
- Only enable this feature when integrating with trusted authentication systems
- Use HTTPS in production to prevent token interception
- Tokens are stored as HttpOnly cookies when served over HTTPS
- The feature is disabled by default

### AI Frontend Development Mode

For UI development, design work, or AI agent integration **without needing the backend**, enable the mock API mode:

```bash
# Set the environment variable
export AI_FRONTEND_DEV=true

# Or add to .env file
AI_FRONTEND_DEV=true

# Run the frontend
cd frontend
npm install
npm run dev
```

**Benefits:**
- ✅ No backend or database required
- ✅ All API calls return realistic mock data
- ✅ Works seamlessly with UI builders (Builder.io, etc.)
- ✅ Simulated network delays for realistic testing
- ✅ Perfect for rapid prototyping and design iteration

**For AI Agents and Developers:**
See [AGENTS.md](AGENTS.md) for comprehensive development guidelines, including:
- Code quality checks and linting requirements
- Dependency management
- Translation verification procedures
- Mock data customization
- Testing procedures

### JWT Token Requirements

The backend expects JWT tokens with the following claims:
- `sub`: Subject/user ID (required)
- `exp`: Expiration timestamp (required)
- `email_verified`: Must be `true` (required for protected endpoints)
- `mfa_enabled`: Must be `true` (required for protected endpoints)
- `admin`: Must be `true` (required for admin endpoints like `/admin/roles`, `/admin/users`)
- `email`: User email (optional, used in onboarding)
- `name`: User full name (optional, used in onboarding)

#### Example JWT Token Generation (Python)

```python
import jwt
import time

# Regular user token
payload = {
    "sub": "user123",
    "exp": int(time.time()) + 3600,
    "email_verified": True,
    "mfa_enabled": True,
    "email": "user@example.com",
    "name": "John Doe"
}

# Admin user token (required for /admin/* endpoints)
admin_payload = {
    "sub": "admin123",
    "exp": int(time.time()) + 3600,
    "email_verified": True,
    "mfa_enabled": True,
    "admin": True,  # Required for admin access
    "email": "admin@example.com",
    "name": "Admin User"
}

token = jwt.encode(payload, "your-secret-key", algorithm="HS256")
admin_token = jwt.encode(admin_payload, "your-secret-key", algorithm="HS256")
print(token)
print(admin_token)
```

## 🏗️ Project Structure

```
.
├── src/                          # Rust backend source
│   └── main.rs                  # Main application
├── migrations/                   # Database migrations
│   └── 001_create_users_table.sql
├── frontend/                     # Nuxt 4 frontend
│   ├── app/                     # App entry point
│   ├── components/              # Vue components
│   │   ├── Navbar.vue
│   │   └── Sidebar.vue
│   ├── layouts/                 # Layout components
│   │   └── default.vue
│   ├── pages/                   # Auto-routed pages
│   │   ├── index.vue           # Dashboard
│   │   └── support.vue
│   ├── locales/                 # i18n translations
│   │   ├── en.json
│   │   ├── pt.json
│   │   └── es.json
│   └── assets/css/              # Styles
├── .github/
│   ├── workflows/               # CI/CD pipelines
│   │   ├── ci.yml              # Continuous Integration
│   │   └── cd.yml              # Continuous Deployment
│   └── copilot-instructions.md  # AI coding guidelines
├── docker-compose.yml           # Main services
├── docker-compose.observability.yml  # Observability stack
├── nginx.conf                   # Nginx gateway config
├── Dockerfile                   # Backend container
└── Cargo.toml                   # Rust dependencies
```

## 🎨 Frontend Features

### Language Switching
Click the language selector in the navbar to switch between English, Portuguese, and Spanish. The selection is persisted in a cookie.

### Dark Mode
Toggle between light and dark modes using the button in the navbar. The preference is saved to localStorage and respects system preferences by default.

### Responsive Design
The UI adapts to mobile, tablet, and desktop screen sizes with a collapsible sidebar on mobile devices.

### Customization
- Project name is configurable via `NUXT_PUBLIC_PROJECT_NAME` environment variable
- Brand colors can be customized in `tailwind.config.js`
- All UI text is translatable via locale files

## 📊 Observability

The template includes a complete observability stack:

```bash
# Start the observability stack
docker-compose -f docker-compose.observability.yml up -d

# Access Grafana
# URL: http://localhost:3030
# Default credentials: admin/admin
```

**Components:**
- **Grafana**: Unified dashboard for metrics, logs, and traces
- **Loki**: Log aggregation and querying
- **Tempo**: Distributed tracing
- **Mimir**: Long-term metrics storage
- **OpenTelemetry Collector**: Telemetry data collection and forwarding

## 🔒 Security

- JWT tokens must have `email_verified` and `mfa_enabled` set to `true`
- CORS is configured as permissive for development (restrict for production)
- No secrets are hardcoded in Docker images
- Database uses parameterized queries (SQL injection protection)
- All environment variables should be properly secured in production

### Production Recommendations

1. Use strong, randomly generated JWT secrets
2. Configure CORS to specific origins
3. Enable HTTPS/TLS
4. Use Docker secrets or a secrets manager
5. Regular security updates
6. Implement rate limiting
7. Enable database connection pooling
8. Set up monitoring and alerting

## 🧪 Testing

### Backend
```bash
# Run tests
cargo test

# Lint with clippy
cargo clippy

# Format code
cargo fmt
```

### Frontend
```bash
cd frontend

# Build (ensures TypeScript compiles)
npm run build

# Run in preview mode
npm run preview
```

## 🚢 CI/CD

### Continuous Integration
Automatically runs on pull requests and pushes to main/develop:
- Backend: rustfmt, clippy, build, test
- Frontend: build checks
- Docker: image build verification

### Continuous Deployment
Triggered on release publication:
1. Updates version in `Cargo.toml` and `frontend/package.json`
2. Builds Docker images for backend and frontend
3. Pushes images to GitHub Container Registry with version tags
4. Creates release summary with pull commands

#### Creating a Release

```bash
# Create and push a new tag
git tag -a v1.0.0 -m "Release version 1.0.0"
git push origin v1.0.0

# Create a release on GitHub (manually or via CLI)
gh release create v1.0.0 --title "v1.0.0" --notes "Release notes here"
```

## 📖 API Examples

### Health Check
```bash
curl http://localhost:3000/health
```

### Get Version
```bash
curl http://localhost:3000/system/version
```

### Validate Token
```bash
curl -X POST http://localhost:3000/validate-token \
  -H "Content-Type: application/json" \
  -d '{"token":"your-jwt-token"}'
```

### Protected Endpoint (with JWT)
```bash
curl -H "Authorization: Bearer your-jwt-token" \
  http://localhost:3000/system/uptime
```

### Onboarding (auto-register user)
```bash
curl -X POST -H "Authorization: Bearer your-jwt-token" \
  http://localhost:3000/system/onboarding
```

### Get Profile
```bash
curl -H "Authorization: Bearer your-jwt-token" \
  http://localhost:3000/profile
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and linting
5. Submit a pull request

## 📄 License

See the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [Nuxt 4](https://nuxt.com/) - Vue.js framework
- [TailwindCSS](https://tailwindcss.com/) - Utility-first CSS
- [Flowbite](https://flowbite.com/) - UI components
- [SQLx](https://github.com/launchbadge/sqlx) - Database toolkit
- [OpenTelemetry](https://opentelemetry.io/) - Observability framework
