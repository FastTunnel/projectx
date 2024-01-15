use chrono::prelude::*;
use itertools::Itertools;
use pinyin::ToPinyin;
use std::collections::HashSet;
use std::mem;

use crate::error;

pub struct CreateUserParam {
    pub username: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub password: String,
    pub display_name: Option<String>,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct UserProfile {
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

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct User {
    pub id: u64,
    pub identifier: String,
    pub username: String,
    pub password: String,
    pub salt: String,
    pub disabled: bool,
    pub gmt_create: DateTime<Utc>,
    pub creator: String,
    pub gmt_modified: Option<DateTime<Utc>>,
    pub modifier: Option<String>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub token: Option<String>,
    pub user_profile: Option<UserProfile>,
    pub roles: Vec<Role>,
}

impl User {
    pub fn create_user(
        CreateUserParam {
            username,
            phone,
            email,
            password,
            display_name,
        }: CreateUserParam,
        creator: &String,
    ) -> error::Result<Self> {
        let salt = uuid::Uuid::new_v4().to_string();
        let digest = md5::compute(format!("{}{}", password, salt));
        let id = uuid::Uuid::new_v4().to_string();
        Ok(Self {
            id: 0,
            identifier: id.clone(),
            username: username.or(phone.clone()).or(email.clone()).ok_or(
                error::DomainError::IllegalArgument(
                    "username, phone and email are all empty".to_string(),
                ),
            )?,
            password: format!("{:x}", digest),
            salt,
            disabled: false,
            gmt_create: Utc::now(),
            creator: creator.clone(),
            gmt_modified: None,
            modifier: None,
            user_profile: Some(UserProfile {
                id: 0,
                identifier: id.clone(),
                display_name: display_name.clone(),
                gmt_entry: None,
                email,
                email_verified: false,
                phone,
                birthday: None,
                country: None,
                province: None,
                city: None,
                address: None,
                pinyin: display_name.map(|name| {
                    name.as_str()
                        .to_pinyin()
                        .filter(|v| v.is_some())
                        .map(|x| x.unwrap().plain())
                        .collect::<Vec<_>>()
                        .join(" ")
                }),
                avatar: None,
                gmt_leave: None,
                leave: false,
            }),
            last_login_at: None,
            roles: Vec::new(),
            token: None,
        })
    }

    pub fn change_password(&mut self, password: String) {
        let digest = md5::compute(format!("{}{}", password, self.salt));
        self.password = format!("{:x}", digest);
    }

    pub fn verify_user_info(&self) -> error::Result<()> {
        if self.password.is_empty() {
            return Err(error::DomainError::IllegalArgument(
                "user password is empty".to_string(),
            ));
        }
        if self.password.len() > 50 {
            return Err(error::DomainError::IllegalArgument(
                "user password is too long".to_string(),
            ));
        }
        if self.password.len() < 8 {
            return Err(error::DomainError::IllegalArgument(
                "user password is too short".to_string(),
            ));
        }
        Ok(())
    }

    pub fn change_display_name(&mut self, display_name: String) {
        if let Some(user_profile) = self.user_profile.as_mut() {
            user_profile.display_name = Some(display_name);
        } else {
            tracing::error!("user profile is empty");
        }
    }

    pub fn change_avatar(&mut self, avatar: String) {
        if let Some(user_profile) = self.user_profile.as_mut() {
            user_profile.avatar = Some(avatar);
        } else {
            tracing::error!("user profile is empty");
        }
    }

    pub fn login(&mut self, password: &String, new_token: &String) -> error::Result<()> {
        let digest = md5::compute(format!("{}{}", password, self.salt));
        if self.password != format!("{:x}", digest) {
            return Err(error::DomainError::PasswordNotMatch);
        }
        self.last_login_at = Some(Utc::now());
        self.token = Some(new_token.clone());
        Ok(())
    }

    pub fn disable_user(&mut self) {
        self.disabled = true;
    }

    pub fn enable_user(&mut self) {
        self.disabled = false;
    }

    pub fn leave(&mut self) {
        if let Some(user_profile) = self.user_profile.as_mut() {
            user_profile.leave = true;
            user_profile.gmt_leave = Some(Utc::now());
        } else {
            tracing::error!("user profile is empty");
        }
    }

    pub fn bind_roles(&mut self, roles: Vec<Role>) {
        let mut old_roles = mem::replace(&mut self.roles, vec![]);
        old_roles.extend(roles.into_iter());
        self.roles = old_roles
            .into_iter()
            .unique_by(|role| role.id)
            .collect::<Vec<_>>();
    }
    pub fn unbind_roles(&mut self, roles: Vec<Role>) {
        let old_roles = mem::replace(&mut self.roles, vec![]);
        let role_map = roles.into_iter().collect::<HashSet<_>>();
        self.roles = old_roles
            .into_iter()
            .take_while(|role| {
                if role_map.contains(role) {
                    return false;
                }
                true
            })
            .collect::<Vec<_>>();
    }

    pub(crate) fn refresh_token(&mut self, token: &String) {
        self.token = Some(token.clone());
    }
}

pub struct CreateRoleParam {
    pub name: String,
    pub description: Option<String>,
    pub organization: String,
    pub parent: Option<String>,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Role {
    pub id: u64,
    pub identifier: String,
    pub name: String,
    pub description: Option<String>,
    pub organization: String,
    pub parent: Option<String>,
    pub default_role: bool,
    pub gmt_create: DateTime<Utc>,
    pub creator: String,
    pub gmt_modified: Option<DateTime<Utc>>,
    pub modifier: Option<String>,
    pub permissions: Vec<Permissions>,
}

impl Role {
    pub fn create_role(
        CreateRoleParam {
            name,
            description,
            organization,
            parent,
        }: CreateRoleParam,
        creator: &String,
    ) -> error::Result<Self> {
        let id = uuid::Uuid::new_v4().to_string();
        Ok(Self {
            id: 0,
            identifier: id.clone(),
            name,
            description,
            organization,
            parent,
            default_role: false,
            gmt_create: Utc::now(),
            creator: creator.clone(),
            gmt_modified: None,
            modifier: None,
            permissions: Vec::new(),
        })
    }

    pub fn bind_permission(&mut self, permission: Permissions) {
        self.permissions.push(permission);
    }

    pub fn unbind_permission(&mut self, permission: Permissions) {
        self.permissions.retain(|p| p.id != permission.id);
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Permissions {
    pub id: u64,
    pub key: String,
    pub name: String,
    pub group_permission: bool,
}

pub struct Organization {
    pub id: u64,
    pub identifier: String,
    pub name: String,
    pub description: Option<String>,
    pub pinyin: String,
    pub public: bool,
    pub icon: Option<String>,
    pub gmt_create: DateTime<Utc>,
    pub gmt_modified: Option<DateTime<Utc>>,
}

pub struct CreateOrganizationParam {
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
}

impl Organization {
    pub fn create_organization(
        CreateOrganizationParam {
            name,
            description,
            icon,
        }: CreateOrganizationParam,
    ) -> error::Result<Self> {
        let id = uuid::Uuid::new_v4().to_string();
        Ok(Self {
            id: 0,
            identifier: id.clone(),
            name: name.clone(),
            description,
            pinyin: name
                .as_str()
                .to_pinyin()
                .filter(|v| v.is_some())
                .map(|x| x.unwrap().plain())
                .collect::<Vec<_>>()
                .join(" "),
            public: false,
            icon,
            gmt_create: Utc::now(),
            gmt_modified: None,
        })
    }
}

pub struct CreateTeamParam {
    pub name: String,
    pub description: Option<String>,
    pub public: bool,
    pub icon: Option<String>,
    pub parent: Option<String>,
    pub leader: Option<String>,
}

pub struct Team {
    pub id: u64,
    pub identifier: String,
    pub name: String,
    pub description: Option<String>,
    pub public: bool,
    pub icon: Option<String>,
    pub parent: Option<String>,
    pub leader: Option<String>,
    pub gmt_create: DateTime<Utc>,
    pub creator: String,
    pub gmt_modified: Option<DateTime<Utc>>,
    pub modifier: Option<String>,
    pub organization: Option<Organization>,
    pub members: Vec<User>,
}

impl Team {
    pub fn create_team(
        organization: Organization,
        CreateTeamParam {
            name,
            description,
            public,
            icon,
            parent,
            leader,
        }: CreateTeamParam,
        creator: &String,
    ) -> error::Result<Self> {
        let id = uuid::Uuid::new_v4().to_string();
        Ok(Self {
            id: 0,
            identifier: id.clone(),
            name: name.clone(),
            description,
            organization: Some(organization),
            public,
            icon,
            parent,
            leader,
            gmt_create: Utc::now(),
            creator: creator.clone(),
            gmt_modified: None,
            modifier: None,
            members: vec![],
        })
    }

    pub fn add_member(&mut self, users: Vec<User>) {
        let mut old_members = mem::replace(&mut self.members, vec![]);
        old_members.extend(users.into_iter());
        self.members = old_members
            .into_iter()
            .unique_by(|user| user.id)
            .collect::<Vec<_>>();
    }

    pub fn remove_member(&mut self, users: Vec<User>) {
        let old_members = mem::replace(&mut self.members, vec![]);
        let user_map = users.into_iter().collect::<HashSet<_>>();
        self.members = old_members
            .into_iter()
            .take_while(|user| {
                if user_map.contains(user) {
                    return false;
                }
                true
            })
            .collect::<Vec<_>>();
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn generate_password() {
        let password = "12345678";
        let id = uuid::Uuid::new_v4().to_string();
        println!("{}", id);
        let salt = uuid::Uuid::new_v4().to_string();
        println!("{}", salt);
        let digest = md5::compute(format!("{}{}", password, salt));
        println!("{:x}", digest);
    }
}
