use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use dotenv::dotenv;

pub struct ServerConfig {
    pub host: String,
    pub port: i32,
    pub url: String,
    pub secret_key: String,
    pub database_url: String,
}

pub type Connection = ConnectionManager<PgConnection>;
pub type Pool = r2d2::Pool<Connection>;

impl ServerConfig {
    pub fn from_env() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("set DATABASE_URL");
        let secret_key = std::env::var("SECRET_KEY").expect("set SECRET_KEY");
        let url = std::env::var("URL").expect("set URL");
        let host = std::env::var("HOST").unwrap_or("0.0.0.0".to_string());
        let port = std::env::var("PORT")
            .unwrap_or("8080".to_string())
            .parse::<i32>()
            .unwrap();

        Self {
            url,
            database_url,
            secret_key,
            host,
            port,
        }
    }

    pub fn configured_pool(&self) -> Pool {
        let manager = ConnectionManager::<PgConnection>::new(self.database_url.clone());
        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.")
    }
}
