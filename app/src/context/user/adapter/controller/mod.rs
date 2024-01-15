use axum::routing::{get, post};
use axum::Router;

pub mod organization;
pub mod user;

pub fn init_user_router() -> Router {
    Router::new()
        .route("/register", post(user::register_user))
        .route("/detail", get(user::user_detail))
        .route("/create_role", post(user::create_role))
        .route("/bind_role", post(user::user_bind_roles))
        .route("/unbind_role", post(user::user_unbind_roles))
}

pub fn init_org_router() -> Router {
    Router::new()
        .route(
            "/current_organization",
            get(organization::current_organization),
        )
        .route("/create_team", post(organization::create_team))
        .route("/team_add_member", post(organization::team_add_member))
        .route(
            "/team_remove_member",
            post(organization::team_remove_member),
        )
}
