pub(crate) mod organization;
pub(crate) mod po;
pub(crate) mod role;
pub(crate) mod team;
pub(crate) mod user;

pub async fn init_table() {
    po::organization::init_table().await;
    po::permission::init_table().await;
    po::role::init_table().await;
    po::role_permission::init_table().await;
    po::team::init_table().await;
    po::team_member::init_table().await;
    po::user::init_table().await;
    po::user_profile::init_table().await;
    po::user_role::init_table().await;
}
