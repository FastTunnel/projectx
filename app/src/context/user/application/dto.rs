use chrono::{DateTime, Utc};
use serde::Serialize;

pub mod command {
    use serde::Deserialize;
    use validator::{Validate, ValidationError};

    #[derive(Debug, Deserialize, Validate)]
    #[validate(schema(function = "valid_user_create_fn", skip_on_field_errors = false))]
    pub struct UserCreateCommand {
        pub username: Option<String>,
        pub phone: Option<String>,
        pub email: Option<String>,
        pub password: String,
        pub display_name: Option<String>,
    }

    fn valid_user_create_fn(param: &UserCreateCommand) -> Result<(), ValidationError> {
        if param.username.is_some() || param.phone.is_some() || param.email.is_some() {
            Ok(())
        } else {
            Err(ValidationError::new(
                "username, phone, email Can not be empty",
            ))
        }
    }

    #[derive(Debug, Deserialize, Validate)]
    pub struct UserLoginCommand {
        #[validate(length(min = 5, max = 30, message = "Can not be empty"))]
        pub username: String,
        #[validate(length(min = 8, max = 30, message = "Can not be empty"))]
        pub password: String,
    }
    #[derive(Debug, Deserialize, Validate)]
    pub struct InitOrganizationCommand {
        #[validate(length(min = 5, max = 100, message = "Can not be empty"))]
        pub name: String,
        pub description: Option<String>,
        pub icon: Option<String>,
    }

    #[derive(Debug, Deserialize, Validate)]
    pub struct CreateTeamCommand {
        #[validate(length(min = 5, max = 100, message = "Can not be empty"))]
        pub name: String,
        pub description: Option<String>,
        pub organization: String,
        pub public: bool,
        pub icon: Option<String>,
        pub parent: Option<String>,
        pub leader: Option<String>,
    }

    #[derive(Debug, Deserialize, Validate)]
    pub struct TeamMemberCommand {
        pub team: String,
        pub members: Vec<String>,
    }

    #[derive(Debug, Deserialize, Validate)]
    pub struct RoleCreateCommand {
        #[validate(length(min = 5, max = 100, message = "Can not be empty"))]
        pub name: String,
        pub description: Option<String>,
        pub organization: String,
        pub parent: Option<String>,
    }

    #[derive(Debug, Deserialize, Validate)]
    pub struct RoleBindCommand {
        #[validate(length(min = 5, max = 100, message = "Can not be empty"))]
        pub user: String,
        #[validate(length(min = 1, message = "Can not be empty"))]
        pub roles: Vec<String>,
    }
}

pub mod query {}

#[derive(Debug, Serialize)]
pub struct UserDto {
    pub id: u64,
    pub identifier: String,
    pub display_name: Option<String>,
    pub gmt_entry: Option<DateTime<Utc>>,
    pub gmt_leave: Option<DateTime<Utc>>,
    pub leave: bool,
    pub email: Option<String>,
    pub email_verified: bool,
    pub phone: Option<String>,
    pub birthday: Option<DateTime<Utc>>,
    pub country: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub address: Option<String>,
    pub pinyin: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OrganizationDto {
    pub id: u64,
    pub identifier: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub public: bool,
    pub pinyin: String,
}
