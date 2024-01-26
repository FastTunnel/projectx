use crate::adapter::repository::po::{space_member, status, tag};

mod po;
pub(crate) mod space;

pub async fn init_table() {
    tag::init_table().await;
    status::init_table().await;
    space_member::init_table().await;
}
