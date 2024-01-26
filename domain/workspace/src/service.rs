use std::sync::Arc;

use crate::facade::{IGlobalConfigFacade, IUserFacade};
use crate::model::project::{CreateProjectParam, CreateProjectSetParam, Project, ProjectSet};
use crate::model::role::Role;
use crate::model::setting::base::FlowItem;
use crate::model::setting::field::Field;
use crate::model::setting::global::GlobalConfig;
use crate::model::setting::space_work_item_set::SpaceWorkItemSet;
use crate::model::setting::status::Status;
use crate::model::setting::template::Template;
use crate::model::setting::work_time_type::WorkTimeType;
use crate::model::tag::Tag;
use crate::model::user::User;
use crate::model::value::CreateTemplateParam;
use crate::repository::ISpaceRepository;
use async_trait::async_trait;
use chrono::Utc;
use domain_common::error;

/// The `IWorkspaceService` trait defines the interface for workspace services.
/// It is an async trait, meaning that its methods return futures.
/// It is also Send + Sync, meaning it can be used across threads.
#[async_trait]
pub trait IWorkspaceService<T>: Send + Sync {
    /// Initializes the system.
    async fn init_global_config(&self, tx: &mut T, org_id: &str) -> error::Result<()>;

    /// Creates a new template.
    /// The `param` parameter contains the parameters for the new template.
    /// The `creator` parameter is the name of the user creating the template.
    async fn create_template(
        &self,
        tx: &mut T,
        param: CreateTemplateParam,
        creator: &str,
    ) -> error::Result<String>;

    /// Creates a new project.
    /// The `param` parameter contains the parameters for the new project.
    /// The `creator` parameter is the name of the user creating the project.
    async fn create_project(
        &self,
        tx: &mut T,
        param: CreateProjectParam,
        creator: &str,
    ) -> error::Result<Project>;

    /// Creates a new project set.
    /// The `param` parameter contains the parameters for the new project set.
    /// The `creator` parameter is the name of the user creating the project set.
    async fn create_project_set(
        &self,
        tx: &mut T,
        param: CreateProjectSetParam,
        creator: &str,
    ) -> error::Result<ProjectSet>;

    async fn find_space_members(&self, tx: &mut T, space_id: &String) -> error::Result<Vec<User>>;

    async fn find_space_tags(&self, tx: &mut T, space_id: &String) -> error::Result<Vec<Tag>>;
}

/// The `WorkspaceService` struct is an implementation of the `IWorkspaceService` trait.
pub struct WorkspaceService<T> {
    user_facade: Arc<dyn IUserFacade>,
    global_config_facade: Arc<dyn IGlobalConfigFacade>,
    space_repo: Arc<dyn ISpaceRepository<Transaction = T>>,
}

impl<T> WorkspaceService<T> {
    pub fn new(
        user_facade: Arc<dyn IUserFacade>,
        global_config_facade: Arc<dyn IGlobalConfigFacade>,
        space_repo: Arc<dyn ISpaceRepository<Transaction = T>>,
    ) -> Self {
        Self {
            user_facade,
            global_config_facade,
            space_repo,
        }
    }
}

#[async_trait]
impl<T> IWorkspaceService<T> for WorkspaceService<T>
where
    T: Send + Sync,
{
    /// Initializes the system.
    async fn init_global_config(&self, _tx: &mut T, org_id: &str) -> error::Result<()> {
        // Initializes the system config
        let project_fields = Field::init_project_fields(org_id.into());
        let global_work_item_fields = Field::init_work_item_fields(org_id.into());
        let project_status = Status::init_project_status(org_id.into());
        let project_set_status = Status::init_project_set_status(org_id.into());
        let work_item_status = Status::init_work_item_status(org_id.into());
        let work_time_type = WorkTimeType::init_work_time_type(org_id.into());
        let project_work_item_set =
            SpaceWorkItemSet::init_work_item_set(org_id, &global_work_item_fields);
        let project_flow = FlowItem::init_project_flow_item();
        let mut global_config = GlobalConfig {
            organization: org_id.into(),
            project_set_status_flow: project_flow.clone(),
            project_fields,
            project_status_flow: project_flow,
            project_work_item_set,
            work_item_fields: global_work_item_fields,
            work_item_status,
            project_set_status,
            project_status,
            global_work_item_work_time_type: work_time_type,
        };
        self.global_config_facade
            .save_global_config(&mut global_config)
            .await?;

        // Initializes the system roles & templates
        let global_roles = self.user_facade.query_global_roles(org_id).await?;
        let mut templates = Template::init_template(org_id.into(), &global_config);
        let mut clone_roles = templates
            .iter()
            .map(|v| {
                global_roles
                    .iter()
                    .filter(|role| !role.is_project_set_role)
                    .cloned()
                    .into_iter()
                    .map(|mut role| {
                        role.id = 0;
                        role.identifier = uuid::Uuid::new_v4().to_string();
                        role.own = Some(v.identifier.clone());
                        role
                    })
                    .collect::<Vec<Role>>()
            })
            .flatten()
            .collect::<Vec<Role>>();
        self.user_facade.create_role(&mut clone_roles).await?;
        self.global_config_facade
            .save_templates(&mut templates)
            .await?;

        Ok(())
    }

    /// Creates a new template.
    /// The `param` parameter contains the parameters for the new template.
    /// The `creator` parameter is the name of the user creating the template.
    async fn create_template(
        &self,
        _tx: &mut T,
        param: CreateTemplateParam,
        creator: &str,
    ) -> error::Result<String> {
        let global_config = self
            .global_config_facade
            .find_global_config_by_org(&param.organization)
            .await?
            .ok_or(error::DomainError::AppNotInitialized)?;
        let global_roles = self
            .user_facade
            .query_global_roles(&param.organization)
            .await?;
        let uuid = uuid::Uuid::new_v4().to_string();
        let mut clone_roles = global_roles
            .iter()
            .filter(|role| !role.is_project_set_role)
            .cloned()
            .into_iter()
            .map(|mut role| {
                role.id = 0;
                role.identifier = uuid::Uuid::new_v4().to_string();
                role.own = Some(uuid.clone());
                role
            })
            .collect::<Vec<Role>>();
        self.user_facade.create_role(&mut clone_roles).await?;
        let mut template = Template {
            gmt_create: Utc::now(),
            gmt_modified: None,
            creator: creator.to_string(),
            modifier: None,
            identifier: uuid,
            name: param.name.clone(),
            display_name: param.name,
            description: param.description,
            icon: None,
            enable: true,
            organization: param.organization,
            project_fields: global_config.project_fields.clone(),
            project_roles: vec![],
            project_status_flow: global_config.project_status_flow.clone(),
            project_work_item_set: global_config.project_work_item_set.clone(),
        };
        self.global_config_facade
            .save_template(&mut template)
            .await?;
        Ok(template.identifier)
    }

    /// Creates a new project.
    /// The `param` parameter contains the parameters for the new project.
    /// The `creator` parameter is the name of the user creating the project.
    async fn create_project(
        &self,
        tx: &mut T,
        param: CreateProjectParam,
        creator: &str,
    ) -> error::Result<Project> {
        let template = self
            .global_config_facade
            .find_template_by_identifier(&param.organization, &param.project_template)
            .await?
            .ok_or(error::DomainError::AppNotInitialized)?;
        let mut project = Project::new_project(param, &template, creator)?;
        self.space_repo.save_project(tx, &mut project).await?;
        Ok(project)
    }

    /// Creates a new project set.
    /// The `param` parameter contains the parameters for the new project set.
    /// The `creator` parameter is the name of the user creating the project set.
    async fn create_project_set(
        &self,
        tx: &mut T,
        param: CreateProjectSetParam,
        creator: &str,
    ) -> error::Result<ProjectSet> {
        let global_config = self
            .global_config_facade
            .find_global_config_by_org(&param.organization)
            .await?
            .ok_or(error::DomainError::AppNotInitialized)?;
        let mut project_set = ProjectSet::new_project_set(param, &global_config, creator)?;
        self.space_repo
            .save_project_set(tx, &mut project_set)
            .await?;
        Ok(project_set)
    }

    async fn find_space_members(&self, tx: &mut T, space_id: &String) -> error::Result<Vec<User>> {
        let member_ids = self.space_repo.find_space_member_ids(tx, space_id).await?;
        let members = self.user_facade.query_users_by_ids(&member_ids).await?;
        Ok(members)
    }

    async fn find_space_tags(&self, tx: &mut T, space_id: &String) -> error::Result<Vec<Tag>> {
        let tags = self.space_repo.find_space_tags(tx, space_id).await?;
        Ok(tags)
    }
}
