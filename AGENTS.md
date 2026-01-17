# AI Agents Development Guide

This document provides guidelines for AI agents working on this project, including automated tools, UI builders, and code assistants.

## Quick Start for AI Agents

### Frontend Development Without Backend

For UI development, testing, and design work, you can run the frontend without needing the backend by enabling the **AI_FRONTEND_DEV** mode:

```bash
# Set the environment variable
export AI_FRONTEND_DEV=true

# Or in your .env file
AI_FRONTEND_DEV=true

# Then run the frontend
cd frontend
npm install
npm run dev
```

**What this enables:**
- All API calls return mocked data automatically
- No backend or database required
- Works seamlessly with UI builders like Builder.io, Figma imports, etc.
- Realistic network delays simulated for authentic UX testing
- Console logs show when mock data is being used

### Mock Data Configuration

Mock data is defined in `frontend/utils/mockData.ts`. You can customize this for your testing needs:

```typescript
export const mockData = {
  version: { version: '0.1.0' },
  health: { status: 'ok', message: 'Service is healthy' },
  profile: { user: { /* user data */ } },
  // Add more as needed
}
```

### Using the API in Components

Always use the `useApi()` composable instead of direct fetch calls:

```vue
<script setup lang="ts">
const { get, post, authenticatedRequest } = useApi()

// Simple GET request
const data = await get<ResponseType>('/endpoint', 'mockKey')

// POST request
const result = await post<ResponseType>('/endpoint', { body }, 'mockKey')

// Authenticated request
const profile = await authenticatedRequest<ProfileType>(
  '/profile',
  token,
  'GET',
  null,
  'profile'
)
</script>
```

## Backend Code Organization

The Rust backend follows a modular structure for maintainability and clarity:

### File Structure

```
src/
├── main.rs              # Application entry point and route configuration
├── models.rs            # Data structures and type definitions
├── middleware.rs        # Authentication and authorization middleware
├── migrations.rs        # SQL migration parser with PostgreSQL support
└── handlers/            # Request handlers organized by domain
    ├── mod.rs           # Module exports
    ├── system.rs        # System endpoints (health, version, uptime, profile)
    └── admin.rs         # Admin endpoints (roles and users management)
```

### Module Responsibilities

**`models.rs`**
- All structs and data models
- Request/response types
- Database models (User, Role, Permission, etc.)
- Application state (AppState)

**`middleware.rs`**
- JWT token validation
- Authentication middleware (`auth_middleware`)
- Admin authorization middleware (`admin_middleware`)
- Claims extractor implementation

**`migrations.rs`**
- SQL statement parser respecting PostgreSQL syntax
- Handles DO $$ ... END $$; blocks correctly
- Comprehensive test suite for parser

**`handlers/system.rs`**
- Public endpoints: `health_check`, `system_version`, `validate_token`
- Protected endpoints: `system_uptime`, `system_onboarding`, `get_profile`
- Basic system operations

**`handlers/admin.rs`**
- Role management: CRUD operations for roles
- Permission management: Setting page-level permissions
- User management: Listing users, assigning/removing roles
- Admin-only operations

**`main.rs`**
- Application initialization
- Database connection and migration execution
- Route configuration and middleware setup
- Server startup

### Adding New Functionality

**For system-level features:**
1. Add models to `models.rs`
2. Add handler to `handlers/system.rs`
3. Register route in `main.rs`

**For admin features:**
1. Add models to `models.rs`
2. Add handler to `handlers/admin.rs`
3. Register route in `main.rs` under admin routes

**For new domains (e.g., reports, analytics):**
1. Create `handlers/domain_name.rs`
2. Add models to `models.rs`
3. Export in `handlers/mod.rs`
4. Register routes in `main.rs`

### Example: Adding a New Admin Endpoint

```rust
// 1. Add request/response models to models.rs
#[derive(Deserialize)]
pub struct MyRequest {
    pub field: String,
}

#[derive(Serialize)]
pub struct MyResponse {
    pub result: String,
}

// 2. Add handler to handlers/admin.rs
pub async fn my_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<MyRequest>,
) -> Result<Json<MyResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Implementation
    Ok(Json(MyResponse { result: "success".to_string() }))
}

// 3. Register in main.rs
let admin_routes = Router::new()
    .route("/admin/my-endpoint", post(admin::my_handler))
    // ... other routes
```

## Development Workflow

### 1. Code Quality Checks

**Before committing code, always run:**

```bash
# Backend (Rust)
cd /path/to/project
cargo fmt --check          # Check formatting
cargo fmt                  # Fix formatting
cargo clippy -- -D warnings # Lint with strict warnings
cargo test                 # Run tests
cargo outdated            # Check for outdated dependencies
cargo audit               # Security audit

# Frontend (Nuxt/TypeScript)
cd frontend
npm run build             # Ensure TypeScript compiles
npx prettier --check .    # Check formatting (if configured)
npx eslint .             # Lint code (if configured)
npm outdated             # Check for outdated dependencies
npm audit                # Security audit
npm audit fix            # Fix security issues automatically
```

### 2. Dependency Management

**Check for outdated dependencies regularly:**

```bash
# Backend
cargo outdated
cargo update  # Update within semver constraints

# Frontend
npm outdated
npm update   # Update within semver constraints
```

**Security audits:**

```bash
# Backend
cargo audit
cargo audit fix  # If fixes available

# Frontend
npm audit
npm audit fix
```

### 3. Translation Verification

**Every single piece of UI text must be translated into all supported languages:**
- English (en)
- Portuguese (pt)
- Spanish (es)

**Translation files location:** `frontend/locales/`

**Verification checklist:**
- [ ] Check `frontend/locales/en.json`
- [ ] Check `frontend/locales/pt.json`
- [ ] Check `frontend/locales/es.json`
- [ ] All keys exist in all three files
- [ ] No hardcoded strings in `.vue` files
- [ ] Use `$t('key.path')` for all user-facing text
- [ ] Date/time formats respect locale
- [ ] Number formats respect locale

**Example translation pattern:**

```vue
<!-- BAD: Hardcoded text -->
<button>Save</button>

<!-- GOOD: Internationalized -->
<button>{{ $t('common.save') }}</button>
```

**Adding new translations:**

1. Add to `en.json`:
```json
{
  "myFeature": {
    "title": "My Feature",
    "description": "This is my feature"
  }
}
```

2. Add corresponding translations to `pt.json` and `es.json`

3. Use in components:
```vue
<h1>{{ $t('myFeature.title') }}</h1>
<p>{{ $t('myFeature.description') }}</p>
```

### 4. Testing Changes

**Backend testing:**

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Check coverage (if configured)
cargo tarpaulin
```

**Frontend testing:**

```bash
# Build test
npm run build

# Development server
npm run dev

# With AI_FRONTEND_DEV enabled
AI_FRONTEND_DEV=true npm run dev
```

## Common Tasks

### Adding a New API Endpoint (Frontend)

1. Add mock data to `frontend/utils/mockData.ts`:
```typescript
export const mockData = {
  // ... existing data
  myNewEndpoint: {
    data: 'mock response'
  }
}
```

2. Use in component:
```typescript
const { get } = useApi()
const data = await get('/my/endpoint', 'myNewEndpoint')
```

### Adding a New Page

1. Create page in `frontend/pages/mypage.vue`
2. Add translations to all locale files
3. Add navigation link in sidebar if needed
4. Test with `AI_FRONTEND_DEV=true`

### Updating Styles

1. Use TailwindCSS utility classes
2. Use Flowbite components where possible
3. Support dark mode with `dark:` prefixes
4. Test in both light and dark modes
5. Ensure responsive design (mobile-first)

## CI/CD Integration

### Continuous Integration

The CI workflow automatically:
- Runs `rustfmt --check` on Rust code
- Runs `clippy` with strict warnings
- Builds backend and frontend
- Runs all tests
- Checks Docker builds

**Ensure your code passes all checks before pushing.**

### Continuous Deployment

Triggered on releases:
- Auto-updates version numbers
- Builds Docker images
- Pushes to GitHub Container Registry
- Creates release notes

## Troubleshooting

### Frontend won't start

1. Check Node.js version (requires 20+)
2. Delete `node_modules` and `package-lock.json`, then `npm install`
3. Clear Nuxt cache: `rm -rf .nuxt .output`

### Backend won't compile

1. Check Rust version (requires 1.83+)
2. Update dependencies: `cargo update`
3. Clean build: `cargo clean && cargo build`

### Translations missing

1. Run grep to find hardcoded strings:
```bash
cd frontend
grep -r ">" . --include="*.vue" | grep -v "$t(" | grep -v "node_modules"
```

2. Check for missing keys:
```bash
# Compare locale files
diff <(jq -r 'keys[]' locales/en.json | sort) <(jq -r 'keys[]' locales/pt.json | sort)
```

### Mock data not working

1. Verify `AI_FRONTEND_DEV=true` is set
2. Check console for `[AI_FRONTEND_DEV]` log messages
3. Verify mock key matches in `mockData.ts`
4. Check that `useApi()` is being used instead of direct fetch

## Best Practices

### For AI Code Assistants

1. **Always run linters after generating code**
2. **Check for security vulnerabilities in dependencies**
3. **Ensure all text is internationalized**
4. **Test with mock data enabled**
5. **Follow existing code patterns**
6. **Document complex logic**
7. **Use TypeScript for type safety**

### For UI Builders

1. **Enable `AI_FRONTEND_DEV=true`**
2. **Use Flowbite components for consistency**
3. **Follow TailwindCSS conventions**
4. **Support dark mode**
5. **Test responsive layouts**
6. **Use translation keys, not hardcoded text**

### For Testing Tools

1. **Mock data is realistic and comprehensive**
2. **Network delays simulate real conditions**
3. **Error cases are covered in mock responses**
4. **Authentication flows work with mock tokens**

## Resources

- [Nuxt 4 Documentation](https://nuxt.com/)
- [TailwindCSS](https://tailwindcss.com/)
- [Flowbite Components](https://flowbite.com/)
- [Rust Axum Framework](https://github.com/tokio-rs/axum)
- [SQLx Database Toolkit](https://github.com/launchbadge/sqlx)

## Support

For questions or issues:
1. Check this document first
2. Review `.github/copilot-instructions.md`
3. Check the main `README.md`
4. Review existing code patterns
5. Consult the project documentation
