use app_interface::error;
use app_interface::workspace::dto::GlobalConfigDTO;
use axum::Json;

#[tracing::instrument()]
pub async fn global_config_detail() -> error::Result<Json<Option<GlobalConfigDTO>>> {
    todo!()
}
