use tantivy::schema::{Schema, STORED, TEXT};
use tantivy::Index;
use tokio::fs;

use crate::APP_STATE;

pub async fn init_index(config: &crate::config::AppConf) {
    let index_path = config.full_text_search.index_path.as_str();
    let mut schema_builder = Schema::builder();
    schema_builder.add_u64_field("id", STORED);
    schema_builder.add_text_field("title", TEXT);
    schema_builder.add_text_field("body", TEXT);
    schema_builder.add_date_field("date", STORED);
    let schema = schema_builder.build();
    let exists = fs::try_exists(format!("{}/{}", index_path, "meta.json"))
        .await
        .unwrap_or(false);
    let index = if exists {
        Index::open_in_dir(&index_path).expect("open index error")
    } else {
        Index::create_in_dir(&index_path, schema).expect("create index error")
    };
    APP_STATE.index.store(
        Box::into_raw(Box::new(index)),
        std::sync::atomic::Ordering::Relaxed,
    );
}
