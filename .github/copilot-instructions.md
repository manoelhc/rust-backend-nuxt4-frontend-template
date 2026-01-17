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
│   ├── main.rs            # Application entry point and route configuration
│   ├── models.rs          # Data structures and type definitions
│   ├── middleware.rs      # Authentication and authorization middleware
│   ├── migrations.rs      # SQL migration parser with PostgreSQL support
│   └── handlers/          # Request handlers by domain
│       ├── mod.rs         # Module exports
│       ├── system.rs      # System endpoints (health, version, uptime, profile)
│       └── admin.rs       # Admin endpoints (roles and users management)
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

## Backend Code Organization

The backend follows a modular architecture for better maintainability:

### Module Structure

**`models.rs`** - All data structures
- Database models (User, Role, Permission, etc.)
- Request/response types
- Application state

**`middleware.rs`** - Authentication & Authorization
- JWT token validation
- `auth_middleware` - Protected routes
- `admin_middleware` - Admin-only routes
- Claims extractor

**`migrations.rs`** - Database migrations
- SQL parser supporting PostgreSQL syntax
- Handles DO $$ ... END $$; blocks
- Comprehensive test coverage

**`handlers/system.rs`** - System endpoints
- Public: health, version, validate_token
- Protected: uptime, onboarding, profile

**`handlers/admin.rs`** - Admin endpoints
- Role management (CRUD)
- Permission management
- User role assignment

**`main.rs`** - Application bootstrap
- Server initialization
- Route registration
- Middleware configuration

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

### JWT Token Generation for Testing

The project includes a JWT token generator tool for development and testing:

```bash
# Generate a regular user token
make token

# Generate an admin token (for /admin/* endpoints)
make token ARGS="--admin"

# Generate a custom token
make token ARGS="--sub testuser --email test@example.com --name 'Test User'"
```

All available options:
- `--sub <ID>` - Subject/User ID
- `--email <EMAIL>` - User email
- `--name <NAME>` - User full name
- `--email-verified <true|false>` - Email verification status
- `--mfa-enabled <true|false>` - MFA status
- `--admin` - Mark user as admin
- `--expires-in <HOURS>` - Token expiration in hours
- `--secret <SECRET>` - JWT secret (overrides .env)

The tool automatically uses the `JWT_SECRET` from your `.env` file and outputs:
- The generated JWT token
- All claims in human-readable format
- Expiration time
- Usage instructions

See `tools/jwt-generator/README.md` for complete documentation.

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

**For system endpoints:**
1. Add models to `src/models.rs`
2. Add handler to `src/handlers/system.rs`
3. Register route in `src/main.rs` (public or protected routes)
4. Update API documentation
5. Add tests
6. Update frontend API calls if needed
7. Update frontend mock data if needed

**For admin endpoints:**
1. Add models to `src/models.rs`
2. Add handler to `src/handlers/admin.rs`
3. Register route in `src/main.rs` (admin routes)
4. Update API documentation
5. Add tests
6. Update frontend API calls if needed
7. Update frontend mock data if needed

**For new domain (e.g., reports):**
1. Create `src/handlers/domain.rs`
2. Add models to `src/models.rs`
3. Export in `src/handlers/mod.rs`
4. Add handlers to new module
5. Register routes in `src/main.rs`

**Example - Adding an admin endpoint:**
```rust
// 1. In src/models.rs
#[derive(Deserialize)]
pub struct MyRequest {
    pub field: String,
}

#[derive(Serialize)]
pub struct MyResponse {
    pub result: String,
}

// 2. In src/handlers/admin.rs
pub async fn my_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<MyRequest>,
) -> Result<Json<MyResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Implementation
    Ok(Json(MyResponse { result: "success".to_string() }))
}

// 3. In src/main.rs
let admin_routes = Router::new()
    .route("/admin/my-endpoint", post(admin::my_handler))
    // ... other routes
```

## Multi-Tenancy (Organization Isolation)

**CRITICAL:** The application implements strict multi-tenancy. All data MUST be isolated by organization.

### Database Table Requirements

**ALL tables MUST include `organization_id`:**

```sql
CREATE TABLE IF NOT EXISTS my_table (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID REFERENCES organizations(id) ON DELETE CASCADE,  -- REQUIRED
    -- ... other columns ...
);

CREATE INDEX IF NOT EXISTS idx_my_table_organization_id ON my_table(organization_id);
```

### Query Requirements

**ALL queries MUST filter by `organization_id`:**

```rust
// ❌ WRONG
SELECT * FROM users;

// ✅ CORRECT
SELECT * FROM users WHERE organization_id = $1;
```

### JWT Claims

Extract organization from JWT claims:

```rust
pub struct Claims {
    pub organization: Option<String>,  // Organization name
    // ... other fields
}
```

### Handler Pattern

```rust
pub async fn handler(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> Result<Json<Response>, (StatusCode, Json<ErrorResponse>)> {
    // Get organization_id from claims
    let org_id: Option<Uuid> = if let Some(org) = &claims.organization {
        sqlx::query_scalar("SELECT id FROM organizations WHERE name = $1")
            .bind(org)
            .fetch_optional(&state.db_pool)
            .await.ok().flatten()
    } else {
        None
    };

    // Query with organization filter
    let data = sqlx::query_as::<_, Model>(
        "SELECT * FROM table WHERE organization_id = $1"
    )
    .bind(org_id)
    .fetch_all(&state.db_pool)
    .await?;

    Ok(Json(data))
}
```

### Testing

Generate tokens with organization:

```bash
make token ARGS="--organization Acme --email user@acme.com"
```

### Add a new frontend page
1. Create file in `frontend/pages/` (auto-routed)
2. Add translations to all locale files
3. Add navigation link if needed
4. Use `NuxtLayout` wrapper

### Add a database table
1. Create migration in `migrations/XXX_description.sql`
2. **MUST include `organization_id UUID REFERENCES organizations(id) ON DELETE CASCADE`**
3. **MUST create index on `organization_id`**
4. **MUST include organization_id in unique constraints**
5. Add corresponding Rust struct with `sqlx::FromRow` and `organization_id` field
6. **MUST filter all queries by `organization_id`**
7. Test with `cargo test`

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
