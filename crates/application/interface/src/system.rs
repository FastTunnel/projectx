use crate::error;
use async_trait::async_trait;
use domain_system::model::Config;

pub mod dto {
    pub mod command {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        pub struct InitSystemCommand {
            pub config: String,
            pub value: String,
        }
    }
}

#[async_trait]
pub trait IConfigAppService: Send + Sync {
    async fn init_system(&self) -> error::Result<()>;
    async fn sys_is_init(&self) -> error::Result<bool>;
    async fn save(&self, key: &str, value: &str) -> error::Result<()>;
    async fn get_config(&self, key: &str) -> error::Result<Option<Config>>;
    async fn get_config_list(&self, key_prefix: &str) -> error::Result<Vec<Config>>;
}
