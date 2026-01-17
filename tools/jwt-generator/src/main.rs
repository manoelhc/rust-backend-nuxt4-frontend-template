use chrono::{Duration, Utc};
use clap::Parser;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;

/// JWT Claims structure (matching the main project)
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Claims {
    sub: String,
    exp: usize,
    email_verified: Option<bool>,
    mfa_enabled: Option<bool>,
    email: Option<String>,
    name: Option<String>,
    admin: Option<bool>,
    organization: Option<String>,
}

/// JWT Token Generator
/// 
/// Generates JWT tokens for testing and development purposes.
/// Uses the JWT_SECRET from the .env file in the project root.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Subject/User ID
    #[arg(short, long, default_value = "user123")]
    sub: String,

    /// User email
    #[arg(short, long)]
    email: Option<String>,

    /// User full name
    #[arg(short, long)]
    name: Option<String>,

    /// Mark email as verified
    #[arg(long, default_value = "true")]
    email_verified: bool,

    /// Mark MFA as enabled
    #[arg(long, default_value = "true")]
    mfa_enabled: bool,

    /// Mark user as admin
    #[arg(short, long, default_value = "false")]
    admin: bool,

    /// Organization ID
    #[arg(short, long)]
    organization: Option<String>,

    /// Token expiration in hours
    #[arg(long, default_value = "24")]
    expires_in: i64,

    /// JWT secret (overrides .env)
    #[arg(long)]
    secret: Option<String>,
}

fn main() {
    let args = Args::parse();

    // Load .env file from project root (two directories up from tools/jwt-generator)
    let env_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.join(".env"));

    if let Some(path) = env_path {
        if path.exists() {
            dotenv::from_path(&path).ok();
            println!("Loaded .env from: {}", path.display());
        }
    }

    // Get JWT secret from args, env var, or use default
    let jwt_secret = args.secret
        .or_else(|| env::var("JWT_SECRET").ok())
        .unwrap_or_else(|| {
            eprintln!("Warning: JWT_SECRET not found in .env or --secret flag. Using default (NOT SECURE).");
            "my-secret-key".to_string()
        });

    // Calculate expiration time
    let exp = (Utc::now() + Duration::hours(args.expires_in))
        .timestamp() as usize;

    // Build claims
    let claims = Claims {
        sub: args.sub.clone(),
        exp,
        email_verified: Some(args.email_verified),
        mfa_enabled: Some(args.mfa_enabled),
        email: args.email.clone(),
        name: args.name.clone(),
        admin: Some(args.admin),
        organization: args.organization.clone(),
    };

    // Generate token
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .expect("Failed to encode JWT token");

    // Print token and claims info
    println!("\n=== JWT Token Generated ===");
    println!("\nToken:");
    println!("{}", token);
    
    println!("\n=== Claims ===");
    println!("Subject (sub):     {}", claims.sub);
    println!("Email:             {}", claims.email.as_deref().unwrap_or("(not set)"));
    println!("Name:              {}", claims.name.as_deref().unwrap_or("(not set)"));
    println!("Email Verified:    {}", claims.email_verified.unwrap_or(false));
    println!("MFA Enabled:       {}", claims.mfa_enabled.unwrap_or(false));
    println!("Admin:             {}", claims.admin.unwrap_or(false));
    println!("Organization:      {}", claims.organization.as_deref().unwrap_or("(not set)"));
    println!("Expires:           {} (in {} hours)", 
        chrono::DateTime::from_timestamp(claims.exp as i64, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
            .unwrap_or_else(|| "Invalid timestamp".to_string()),
        args.expires_in
    );

    println!("\n=== Usage ===");
    println!("You can use this token in:");
    println!("1. Authorization header: Bearer {}", token);
    println!("2. URL parameter (if NUXT_PUBLIC_ENABLE_URL_AUTH=true): http://localhost:3000/?auth-token={}", token);
    println!();
}
