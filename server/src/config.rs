#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
}

impl Config {
    pub fn init() -> Self {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be configured in the .env file");
        let jwt_secret =
            std::env::var("JWT_SECRET").expect("JWT_SECRET must be configured in the .env file");
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .expect("PORT must be a number");

        Self {
            database_url,
            jwt_secret,
            port,
        }
    }
}
