use app_interface::system::IConfigAppService;
use domain_common::error;
use domain_common::error::DomainError;
use domain_workspace::facade::IGlobalConfigFacade;
use domain_workspace::model::setting::global::{GlobalConfig, GLOBAL_KEY};
use domain_workspace::model::setting::template::{Template, TEMPLATE_KEY};
use std::sync::Arc;

pub struct GlobalConfigFacade {
    global_config_app_service: Arc<dyn IConfigAppService>,
}

impl GlobalConfigFacade {
    pub fn new(global_config_app_service: Arc<dyn IConfigAppService>) -> Self {
        GlobalConfigFacade {
            global_config_app_service,
        }
    }
}

#[async_trait::async_trait]
impl IGlobalConfigFacade for GlobalConfigFacade {
    async fn find_global_config_by_org(
        &self,
        organization: &String,
    ) -> error::Result<Option<GlobalConfig>> {
        let config = self
            .global_config_app_service
            .get_config_list(&format!("{}/{}", GLOBAL_KEY, organization))
            .await
            .map_err(|e| DomainError::CallClientError(e.into()))?;
        if config.is_empty() {
            return Ok(None);
        }
        let config = config.into_iter().last().unwrap();
        let result = serde_json::from_value(config.value)
            .map_err(|e| DomainError::CallClientError(e.into()))?;
        Ok(Some(result))
    }

    async fn find_template_by_identifier(
        &self,
        organization: &String,
        identifier: &String,
    ) -> error::Result<Option<Template>> {
        let config = self
            .global_config_app_service
            .get_config_list(&format!("{}/{}/{}", TEMPLATE_KEY, organization, identifier))
            .await
            .map_err(|e| DomainError::CallClientError(e.into()))?;
        if config.is_empty() {
            return Ok(None);
        }
        let config = config.into_iter().last().unwrap();
        let result = serde_json::from_value(config.value)
            .map_err(|e| DomainError::CallClientError(e.into()))?;
        Ok(Some(result))
    }

    async fn find_all_template(&self, organization: &String) -> error::Result<Vec<Template>> {
        let config = self
            .global_config_app_service
            .get_config_list(&format!("{}/{}", TEMPLATE_KEY, organization))
            .await
            .map_err(|e| DomainError::CallClientError(e.into()))?;

        let mut ret = vec![];
        for config in config {
            let result = serde_json::from_value(config.value)
                .map_err(|e| DomainError::CallClientError(e.into()))?;
            ret.push(result);
        }
        Ok(ret)
    }

    async fn save_global_config(&self, global_config: &mut GlobalConfig) -> error::Result<()> {
        let config = serde_json::to_string(global_config)
            .map_err(|e| DomainError::CallClientError(e.into()))?;
        self.global_config_app_service
            .save(
                &format!("{}/{}", GLOBAL_KEY, &global_config.organization),
                &config,
            )
            .await
            .map_err(|e| DomainError::CallClientError(e.into()))?;
        Ok(())
    }

    async fn save_templates(&self, templates: &mut Vec<Template>) -> error::Result<()> {
        for v in templates {
            self.save_template(v).await?;
        }
        Ok(())
    }

    async fn save_template(&self, template: &mut Template) -> error::Result<()> {
        let config =
            serde_json::to_string(template).map_err(|e| DomainError::CallClientError(e.into()))?;
        self.global_config_app_service
            .save(
                &format!(
                    "{}/{}/{}",
                    TEMPLATE_KEY, &template.organization, &template.identifier
                ),
                &config,
            )
            .await
            .map_err(|e| DomainError::CallClientError(e.into()))?;
        Ok(())
    }
}
