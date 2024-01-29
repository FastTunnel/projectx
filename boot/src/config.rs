use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppConf {
    pub jwt_secret: String,
    pub database: DatabaseConf,
    pub server: ServerConf,
    pub full_text_search: FullTextSearchConf,
}

#[derive(Deserialize)]
pub struct DatabaseConf {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Deserialize)]
pub struct ServerConf {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct FullTextSearchConf {
    pub index_path: String,
}
