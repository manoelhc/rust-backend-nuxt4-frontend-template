# Copilot Instructions for rust-backend-nuxt4-frontend-template

## Project Overview
This is a monorepo template containing:
- **Backend**: Rust with Axum framework, PostgreSQL database, JWT authentication
- **Frontend**: Nuxt 4 with Vue 3, TailwindCSS, Flowbite, and i18n support
- **Infrastructure**: Docker Compose with PostgreSQL, Redis, Nginx gateway, and observability stack

## Code Quality Requirements

**CRITICAL: Before completing any task, you MUST:**

1. **Run Linters and Fix Issues**
   - Backend: `cargo fmt` and `cargo clippy -- -D warnings`
   - Frontend: Check with `npm run build` (ensures TypeScript compiles)
   - Fix all linting errors and warnings

2. **Check Dependencies**
   - Backend: Run `cargo outdated` and `cargo audit`
   - Frontend: Run `npm outdated` and `npm audit`
   - Update dependencies if safe to do so
   - Fix security vulnerabilities

3. **Verify Translations**
   - **EVERY SINGLE TEXT** in the frontend MUST be translated
   - Check all three language files: `en.json`, `pt.json`, `es.json`
   - No hardcoded strings in `.vue` files
   - Use `$t('key.path')` for all user-facing text
   - Verify all translation keys exist in all language files

## Code Style and Standards

### Rust Backend
- Use Rust 2021 edition
- Follow Rust naming conventions (snake_case for functions/variables, PascalCase for types)
- Use `rustfmt` for formatting
- Use `clippy` for linting
- Always handle errors explicitly (avoid unwrap in production code)
- Use structured logging with `tracing`
- Document public APIs with doc comments
- Keep functions focused and small

### TypeScript/Vue Frontend
- Use TypeScript for all new code
- Follow Vue 3 Composition API patterns
- Use `<script setup>` syntax
- Follow TailwindCSS utility-first approach
- Use Flowbite components where applicable
- Ensure all UI text is internationalized (i18n)
- Keep components small and reusable

### Database
- Use PostgreSQL with SQLx for type-safe queries
- Store migrations in `migrations/` directory with sequential numbering
- Always use transactions for multi-statement operations
- Index foreign keys and frequently queried columns
- Always validate and sanitize inputs to prevent SQL injection
- Always use `CREATE <DATABASE|TABLE|INDEX|VIEW|FUNCTION> IF NOT EXISTS` for idempotent migrations

### Docker
- Use multi-stage builds for optimization
- Never hardcode secrets in Dockerfiles
- Use environment variables for configuration
- Document all required environment variables

## Project Structure

```
.
├── src/                    # Rust backend source
│   └── main.rs            # Main application entry point
├── migrations/            # Database migrations
├── frontend/              # Nuxt 4 frontend
│   ├── app/              # App entry point
│   ├── components/       # Vue components
│   ├── layouts/          # Layout components
│   ├── pages/            # Page components (auto-routed)
│   ├── locales/          # i18n translation files
│   └── assets/           # Static assets
├── .github/              # CI/CD workflows
│   └── workflows/        # GitHub Actions
├── docker-compose.yml    # Main services orchestration
├── docker-compose.observability.yml  # Observability stack
├── nginx.conf            # Nginx gateway configuration
└── Cargo.toml            # Rust dependencies
```

## Environment Variables

### Backend
- `JWT_SECRET`: Secret key for JWT signing (required in production)
- `PORT`: Server port (default: 3000)
- `DATABASE_URL`: PostgreSQL connection string
- `OTEL_EXPORTER_OTLP_ENDPOINT`: OpenTelemetry collector endpoint
- `OTEL_SERVICE_NAME`: Service name for observability

### Frontend
- `NUXT_PUBLIC_API_URL`: Backend API URL
- `NUXT_PUBLIC_PROJECT_NAME`: Application name displayed in UI
- `AI_FRONTEND_DEV`: Enable mock API mode for frontend development without backend (set to 'true')

## AI Frontend Development Mode

**For UI development without backend dependencies**, enable the `AI_FRONTEND_DEV` flag:

```bash
export AI_FRONTEND_DEV=true
cd frontend
npm run dev
```

**What this enables:**
- All API calls automatically return mock data
- No backend or database required
- Perfect for UI builders like Builder.io, Figma imports, etc.
- Realistic network delays for authentic UX testing
- Console logs show when mock data is being used

**Implementation:**
- Use `useApi()` composable instead of direct fetch calls
- Mock data defined in `frontend/utils/mockData.ts`
- Automatic fallback to mock data on API failures
- See `AGENTS.md` for detailed AI agent development guide

**Example usage in components:**
```vue
<script setup lang="ts">
const { get } = useApi()
const data = await get<ResponseType>('/endpoint', 'mockKey')
</script>
```

## Testing

### Backend
```bash
cargo test
cargo clippy
cargo fmt --check
```

### Frontend
```bash
cd frontend
npm run build  # Ensures TypeScript compiles correctly
```

## Deployment

### Docker Compose
```bash
# Main application
docker-compose up -d

# With observability
docker-compose -f docker-compose.yml -f docker-compose.observability.yml up -d
```

### Production Considerations
1. Always use strong, unique JWT secrets
2. Configure CORS to specific origins (not permissive)
3. Use TLS/SSL in production
4. Enable database connection pooling
5. Set up proper logging and monitoring
6. Use health checks for all services
7. Implement rate limiting
8. Regular security updates

## API Endpoints

### Public
- `GET /health` - Health check
- `GET /system/version` - Get application version
- `POST /validate-token` - Validate JWT token (checks email_verified and mfa_enabled)

### Protected (requires valid JWT)
- `GET /system/uptime` - Get system uptime
- `POST /system/onboarding` - Auto-register user from JWT claims
- `GET /profile` - Get user profile

### JWT Requirements
- Must include `sub` (subject/user ID)
- Must include `exp` (expiration)
- Must have `email_verified: true`
- Must have `mfa_enabled: true`
- Optional: `email`, `name` for user info

## i18n

All user-facing text must be internationalized. Supported languages:
- English (en)
- Portuguese (pt)
- Spanish (es)

Add new translations to `frontend/locales/{lang}.json`

## Observability

The project includes a complete observability stack:
- **Grafana**: Visualization dashboard (port 3030)
- **Loki**: Log aggregation
- **Tempo**: Distributed tracing
- **Mimir**: Metrics storage
- **OpenTelemetry Collector**: Telemetry collection

Access Grafana at http://localhost:3030 (admin/admin)

## Common Tasks

### Add a new endpoint
1. Define response/request structs in `src/main.rs`
2. Implement handler function
3. Add route to router (protected or public)
4. Update API documentation
5. Add tests
6. Update frontend API calls if needed
7. Update frontend mock data if needed

### Add a new frontend page
1. Create file in `frontend/pages/` (auto-routed)
2. Add translations to all locale files
3. Add navigation link if needed
4. Use `NuxtLayout` wrapper

### Add a database table
1. Create migration in `migrations/XXX_description.sql`
2. Add corresponding Rust struct with `sqlx::FromRow`
3. Test with `cargo test`

## Security Guidelines

1. Never commit secrets to Git
2. Use `.env` files for local development (add to .gitignore)
3. Validate all user inputs
4. Use parameterized queries (SQLx handles this)
5. Implement rate limiting for public endpoints
6. Keep dependencies updated
7. Run security audits regularly (`cargo audit`)
8. Use HTTPS in production
9. Implement proper CORS policies
10. Log security events

## Code Review Checklist

- [ ] Code follows project style guidelines
- [ ] All tests pass
- [ ] No hardcoded secrets or credentials
- [ ] Error handling is appropriate
- [ ] Security considerations addressed
- [ ] Performance implications considered
- [ ] Documentation updated if needed
- [ ] i18n translations added for new UI text
- [ ] Database migrations are reversible if possible
- [ ] Environment variables documented
