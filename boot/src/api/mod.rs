use axum::routing::{get, post};
use axum::Router;

pub mod global;
pub mod organization;
pub mod space;
pub mod template;
pub mod user;

pub fn init_user_router() -> Router {
    Router::new()
        .route("/register", post(user::register_user))
        .route("/detail", get(user::user_detail))
        .route("/own_roles/:organization/roles/:own", get(user::own_roles))
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
        .route("/create_template", post(template::create_template))
        .route("/find_all_template", post(template::find_all_template))
        .route("/create_project", post(space::create_project))
        .route("/create_project_set", post(space::create_project_set))
        .route("/space/:space_id/members", get(space::query_space_member))
        .route("/space/:space_id/tags", get(space::query_space_tag))
        .route(
            "/space/:space_type/:space_id/status_flow",
            get(space::query_space_status_flow),
        )
        .route(
            "/space/:space_id/work_item_set",
            get(space::query_space_work_item_set),
        )
        .route("/space/:space_id/member/add", post(space::space_member_add))
        .route(
            "/space/:space_id/member/delete",
            post(space::space_member_remove),
        )
}
