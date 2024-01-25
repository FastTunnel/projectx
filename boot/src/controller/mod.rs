use axum::routing::{get, post};
use axum::Router;

pub mod global;
pub mod organization;
pub mod template;
pub mod user;

pub fn init_user_router() -> Router {
    Router::new()
        .route("/register", post(user::register_user))
        .route("/detail", get(user::user_detail))
        .route("/create_role", post(user::create_role))
        .route("/bind_role", post(user::user_bind_roles))
        .route("/unbind_role", post(user::user_unbind_roles))
        .route(
            "/query_roles_by_org_and_own",
            post(user::query_roles_by_org_and_own),
        )
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

pub fn init_workspace_router() -> Router {
    Router::new()
        .route("/template", post(template::find_all_template))
        .route("/template_detail", post(template::template_detail))
        .route("/create_template", post(template::create_template))
}
