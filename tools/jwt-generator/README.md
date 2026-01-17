# JWT Token Generator

A command-line utility for generating JWT tokens for testing and development.

## Features

- Generates JWT tokens compatible with the main application
- Reads `JWT_SECRET` from the project's `.env` file
- Supports all JWT claims used in the application
- Configurable expiration time
- Interactive mode with sensible defaults

## Usage

### Basic Token (Regular User)

```bash
make token
```

This generates a token with:
- Default user ID: `user123`
- Email verified: `true`
- MFA enabled: `true`
- Admin: `false`
- Expires in: 24 hours

### Admin Token

```bash
make token ARGS="--admin"
```

### Custom User

```bash
make token ARGS="--sub admin123 --email admin@example.com --name 'Admin User' --admin"
```

### Custom Expiration

```bash
make token ARGS="--expires-in 48"  # Token valid for 48 hours
```

### All Options

```bash
cd tools/jwt-generator
cargo run -- --help
```

Options:
- `-s, --sub <SUB>` - Subject/User ID (default: user123)
- `-e, --email <EMAIL>` - User email
- `-n, --name <NAME>` - User full name
- `--email-verified <true|false>` - Mark email as verified (default: true)
- `--mfa-enabled <true|false>` - Mark MFA as enabled (default: true)
- `-a, --admin` - Mark user as admin (default: false)
- `--expires-in <HOURS>` - Token expiration in hours (default: 24)
- `--secret <SECRET>` - JWT secret (overrides .env)

## Examples

### Generate Admin Token with Full Details

```bash
make token ARGS="--sub admin001 --email admin@company.com --name 'John Admin' --admin --expires-in 168"
```

### Generate Regular User Token

```bash
make token ARGS="--sub user456 --email user@company.com --name 'Jane Doe'"
```

### Quick Test Token (No Verification Required)

```bash
make token ARGS="--email-verified false --mfa-enabled false"
```

## Output

The tool outputs:
1. The generated JWT token
2. All claims in the token
3. Expiration time
4. Usage instructions for both Bearer auth and URL parameter

## Integration

The generated tokens work with:
- **Authorization Header**: `Authorization: Bearer <token>`
- **URL Parameter** (when `NUXT_PUBLIC_ENABLE_URL_AUTH=true`): `http://localhost:3000/?auth-token=<token>`
