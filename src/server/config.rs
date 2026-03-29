use anyhow::{Context, Result};

#[derive(Clone, Debug)]
pub struct Config {
    pub s3_bucket:            String,
    pub s3_region:            String,
    pub jwt_secret:           String,
    pub admin_password_hash:  String,
    pub port:                 u16,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            s3_bucket: std::env::var("S3_BUCKET_NAME")
                .context("S3_BUCKET_NAME manquant")?,
            s3_region: std::env::var("AWS_REGION")
                .unwrap_or_else(|_| "eu-west-3".to_string()),
            jwt_secret: std::env::var("JWT_SECRET")
                .context("JWT_SECRET manquant")?,
            admin_password_hash: std::env::var("ADMIN_PASSWORD_HASH")
                .context("ADMIN_PASSWORD_HASH manquant")?,
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .context("PORT invalide")?,
        })
    }
}
