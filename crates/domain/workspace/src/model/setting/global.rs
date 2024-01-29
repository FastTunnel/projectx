use crate::model::setting::base::FlowItem;
use crate::model::setting::field::Field;
use crate::model::setting::space_work_item_set::SpaceWorkItemSet;
use crate::model::setting::status::Status;
use crate::model::setting::work_time_type::WorkTimeType;
use serde::{Deserialize, Serialize};

pub static GLOBAL_KEY: &'static str = "/global/v1";
/// Global struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub organization: String,

    pub(crate) project_set_status_flow: Vec<FlowItem>,

    /// 项目字段配置
    pub(crate) project_fields: Vec<Field>,

    /// 项目状态配置
    pub(crate) project_status_flow: Vec<FlowItem>,
    /// 项目工作项配置
    pub(crate) project_work_item_set: Vec<SpaceWorkItemSet>,

    /// 全局配置项目工作项字段配置
    pub(crate) work_item_fields: Vec<Field>,
    /// 全局配置项目工作项状态配置
    pub(crate) work_item_status: Vec<Status>,
    pub(crate) project_set_status: Vec<Status>,
    pub(crate) project_status: Vec<Status>,
    /// 全局配置项目工作项状态流转配置
    pub(crate) global_work_item_work_time_type: Vec<WorkTimeType>,
}
