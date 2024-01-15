use crate::error;
use crate::sys::model::Config;
use crate::sys::repository::IConfigRepository;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait IConfigService<T>: Send + Sync {
    async fn sys_is_init(&self, tx: &mut T) -> error::Result<bool>;
    async fn sys_init(&self, tx: &mut T) -> error::Result<()>;
}

pub struct ConfigService<T> {
    config_repo: Arc<dyn IConfigRepository<Transaction = T>>,
}

impl<T> ConfigService<T> {
    pub fn new(config_repo: Arc<dyn IConfigRepository<Transaction = T>>) -> Self {
        Self { config_repo }
    }
}

#[async_trait]
impl<T> IConfigService<T> for ConfigService<T>
where
    T: Send + Sync,
{
    async fn sys_is_init(&self, tx: &mut T) -> error::Result<bool> {
        let option = self
            .config_repo
            .get_config(tx, &"sys_info".to_string())
            .await?;
        match option {
            None => Ok(false),
            Some(config) => match config {
                Config::SysInfo { is_init } => Ok(is_init),
                #[allow(unreachable_patterns)]
                _ => Err(error::DomainError::AppInitialized),
            },
        }
    }

    async fn sys_init(&self, tx: &mut T) -> error::Result<()> {
        let mut config = Config::SysInfo { is_init: true };
        self.config_repo.save(tx, &mut config).await?;
        Ok(())
    }
}
