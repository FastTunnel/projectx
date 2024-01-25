use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ResourceType {
    ProjectSet,
    Project,
    WorkItem,
    WorkItemSet,
}

impl Serialize for ResourceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ResourceType {
    fn deserialize<D>(deserializer: D) -> Result<ResourceType, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ResourceType::from_string(&s).ok_or(serde::de::Error::custom(format!(
            "unknown ResourceType: {}",
            s
        )))
    }
}

#[allow(dead_code)]
impl ResourceType {
    pub fn to_string(&self) -> String {
        match self {
            ResourceType::ProjectSet => "project_set".to_string(),
            ResourceType::Project => "project".to_string(),
            ResourceType::WorkItem => "work_item".to_string(),
            ResourceType::WorkItemSet => "work_item_set".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "project_set" => Option::from(ResourceType::ProjectSet),
            "project" => Option::from(ResourceType::Project),
            "work_item" => Option::from(ResourceType::WorkItem),
            "work_item_set" => Option::from(ResourceType::WorkItemSet),
            _ => None,
        }
    }

    pub fn name(&self) -> String {
        match self {
            ResourceType::ProjectSet => "项目集".to_string(),
            ResourceType::Project => "项目".to_string(),
            ResourceType::WorkItem => "工作项".to_string(),
            ResourceType::WorkItemSet => "工作项集".to_string(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SpaceStage {
    NotStarted,
    InProgress,
    Completed,
}

#[allow(dead_code)]
impl SpaceStage {
    pub fn to_string(&self) -> String {
        match self {
            SpaceStage::NotStarted => "not_started".to_string(),
            SpaceStage::InProgress => "in_progress".to_string(),
            SpaceStage::Completed => "completed".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "not_started" => Option::from(SpaceStage::NotStarted),
            "in_progress" => Option::from(SpaceStage::InProgress),
            "completed" => Option::from(SpaceStage::Completed),
            _ => None,
        }
    }

    pub fn name(&self) -> String {
        match self {
            SpaceStage::NotStarted => "未开始".to_string(),
            SpaceStage::InProgress => "进行中".to_string(),
            SpaceStage::Completed => "已完成".to_string(),
        }
    }

    pub fn order(&self) -> i32 {
        match self {
            SpaceStage::NotStarted => 1,
            SpaceStage::InProgress => 2,
            SpaceStage::Completed => 3,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum SpaceType {
    ProjectSet,
    Project,
}

#[allow(dead_code)]
impl SpaceType {
    pub fn to_string(&self) -> String {
        match self {
            SpaceType::ProjectSet => "project_set".to_string(),
            SpaceType::Project => "project".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "project_set" => Option::from(SpaceType::ProjectSet),
            "project" => Option::from(SpaceType::Project),
            _ => None,
        }
    }

    pub fn name(&self) -> String {
        match self {
            SpaceType::ProjectSet => "项目集".to_string(),
            SpaceType::Project => "项目".to_string(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum WorkItemCategory {
    OriginalDemand,
    Theme,
    Demand,
    Task,
    Bug,
    Risk,
}

impl WorkItemCategory {
    pub fn to_string(&self) -> String {
        match self {
            WorkItemCategory::OriginalDemand => "OriginalDemand".to_string(),
            WorkItemCategory::Theme => "Theme".to_string(),
            WorkItemCategory::Demand => "Demand".to_string(),
            WorkItemCategory::Task => "Task".to_string(),
            WorkItemCategory::Bug => "Bug".to_string(),
            WorkItemCategory::Risk => "Risk".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "OriginalDemand" => Option::from(WorkItemCategory::OriginalDemand),
            "Theme" => Option::from(WorkItemCategory::Theme),
            "Demand" => Option::from(WorkItemCategory::Demand),
            "Task" => Option::from(WorkItemCategory::Task),
            "Bug" => Option::from(WorkItemCategory::Bug),
            "Risk" => Option::from(WorkItemCategory::Risk),
            _ => None,
        }
    }

    pub fn name(&self) -> String {
        match self {
            WorkItemCategory::OriginalDemand => "原始需求".to_string(),
            WorkItemCategory::Theme => "主题".to_string(),
            WorkItemCategory::Demand => "需求".to_string(),
            WorkItemCategory::Task => "任务".to_string(),
            WorkItemCategory::Bug => "缺陷".to_string(),
            WorkItemCategory::Risk => "风险".to_string(),
        }
    }
}

pub enum WorkItemStage {
    Ack,
    Analyze,
    Handle,
    Design,
    Develop,
    Test,
    Verify,
    Release,
    NormalEnd,
    AbnormalEnd,
}

#[allow(dead_code)]
impl WorkItemStage {
    pub fn to_string(&self) -> String {
        match self {
            WorkItemStage::Ack => "Ack".to_string(),
            WorkItemStage::Analyze => "Analyze".to_string(),
            WorkItemStage::Handle => "Handle".to_string(),
            WorkItemStage::Design => "Design".to_string(),
            WorkItemStage::Develop => "Develop".to_string(),
            WorkItemStage::Test => "Test".to_string(),
            WorkItemStage::Verify => "Verify".to_string(),
            WorkItemStage::Release => "Release".to_string(),
            WorkItemStage::NormalEnd => "NormalEnd".to_string(),
            WorkItemStage::AbnormalEnd => "AbnormalEnd".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "Ack" => Option::from(WorkItemStage::Ack),
            "Analyze" => Option::from(WorkItemStage::Analyze),
            "Handle" => Option::from(WorkItemStage::Handle),
            "Design" => Option::from(WorkItemStage::Design),
            "Develop" => Option::from(WorkItemStage::Develop),
            "Test" => Option::from(WorkItemStage::Test),
            "Verify" => Option::from(WorkItemStage::Verify),
            "Release" => Option::from(WorkItemStage::Release),
            "NormalEnd" => Option::from(WorkItemStage::NormalEnd),
            "AbnormalEnd" => Option::from(WorkItemStage::AbnormalEnd),
            _ => None,
        }
    }

    pub fn name(&self) -> String {
        match self {
            WorkItemStage::Ack => "确认阶段".to_string(),
            WorkItemStage::Analyze => "分析阶段".to_string(),
            WorkItemStage::Handle => "处理阶段".to_string(),
            WorkItemStage::Design => "设计阶段".to_string(),
            WorkItemStage::Develop => "开发阶段".to_string(),
            WorkItemStage::Test => "测试阶段".to_string(),
            WorkItemStage::Verify => "验证阶段".to_string(),
            WorkItemStage::Release => "发布阶段".to_string(),
            WorkItemStage::NormalEnd => "正常结束".to_string(),
            WorkItemStage::AbnormalEnd => "异常结束".to_string(),
        }
    }

    pub fn order(&self) -> i32 {
        match self {
            WorkItemStage::Ack => 1,
            WorkItemStage::Analyze => 2,
            WorkItemStage::Handle => 3,
            WorkItemStage::Design => 4,
            WorkItemStage::Develop => 5,
            WorkItemStage::Test => 6,
            WorkItemStage::Verify => 7,
            WorkItemStage::Release => 8,
            WorkItemStage::NormalEnd => 9,
            WorkItemStage::AbnormalEnd => 10,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum FieldType {
    Intput,
    Textarea,
    Select,
    SelectUser,
    MultiSelect,
    MultiSelectUser,
    Switch,
    DateTime,
    Star,
}

impl Serialize for FieldType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for FieldType {
    fn deserialize<D>(deserializer: D) -> Result<FieldType, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FieldType::from_string(&s).ok_or(serde::de::Error::custom(format!(
            "unknown FieldType: {}",
            s
        )))
    }
}

#[allow(dead_code)]
impl FieldType {
    pub fn to_string(&self) -> String {
        match self {
            FieldType::Intput => "input".to_string(),
            FieldType::Textarea => "textarea".to_string(),
            FieldType::Select => "select".to_string(),
            FieldType::SelectUser => "select_user".to_string(),
            FieldType::MultiSelect => "multi_select".to_string(),
            FieldType::Switch => "switch".to_string(),
            FieldType::DateTime => "date_time".to_string(),
            FieldType::MultiSelectUser => "multi_select_user".to_string(),
            FieldType::Star => "star".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "input" => Option::from(FieldType::Intput),
            "textarea" => Option::from(FieldType::Textarea),
            "select" => Option::from(FieldType::Select),
            "select_user" => Option::from(FieldType::SelectUser),
            "multi_select" => Option::from(FieldType::MultiSelect),
            "switch" => Option::from(FieldType::Switch),
            "date_time" => Option::from(FieldType::DateTime),
            "multi_select_user" => Option::from(FieldType::MultiSelectUser),
            "star" => Option::from(FieldType::Star),
            _ => None,
        }
    }

    pub fn name(&self) -> String {
        match self {
            FieldType::Intput => "输入框".to_string(),
            FieldType::Textarea => "文本框".to_string(),
            FieldType::Select => "下拉框".to_string(),
            FieldType::SelectUser => "下拉用户".to_string(),
            FieldType::MultiSelect => "多选框".to_string(),
            FieldType::Switch => "开关".to_string(),
            FieldType::DateTime => "日期时间".to_string(),
            FieldType::MultiSelectUser => "多选用户".to_string(),
            FieldType::Star => "星级".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum OwnType {
    Template,
    Project,
}

impl Serialize for OwnType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for OwnType {
    fn deserialize<D>(deserializer: D) -> Result<OwnType, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        OwnType::from_string(&s).ok_or(serde::de::Error::custom(format!("unknown OwnType: {}", s)))
    }
}

#[allow(dead_code)]
impl OwnType {
    pub fn to_string(&self) -> String {
        match self {
            OwnType::Template => "template".to_string(),
            OwnType::Project => "project".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "template" => Option::from(OwnType::Template),
            "project" => Option::from(OwnType::Project),
            _ => None,
        }
    }

    pub fn name(&self) -> String {
        match self {
            OwnType::Template => "模板".to_string(),
            OwnType::Project => "项目".to_string(),
        }
    }

    pub fn num(&self) -> i32 {
        match self {
            OwnType::Template => 2,
            OwnType::Project => 3,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Category {
    OriginalRequirement,
    Topic,
    Requirement,
    Task,
    Bug,
    Risk,
}

impl Serialize for Category {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Category {
    fn deserialize<D>(deserializer: D) -> Result<Category, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Category::from_string(&s)
            .ok_or(serde::de::Error::custom(format!("unknown Category: {}", s)))
    }
}

#[allow(dead_code)]
impl Category {
    pub fn to_string(&self) -> String {
        match self {
            Category::OriginalRequirement => "OriginalRequirement".to_string(),
            Category::Topic => "Topic".to_string(),
            Category::Requirement => "Requirement".to_string(),
            Category::Task => "Task".to_string(),
            Category::Bug => "Bug".to_string(),
            Category::Risk => "Risk".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "OriginalRequirement" => Option::from(Category::OriginalRequirement),
            "Topic" => Option::from(Category::Topic),
            "Requirement" => Option::from(Category::Requirement),
            "Task" => Option::from(Category::Task),
            "Bug" => Option::from(Category::Bug),
            "Risk" => Option::from(Category::Risk),
            _ => None,
        }
    }

    pub fn name(&self) -> String {
        match self {
            Category::OriginalRequirement => "原始需求".to_string(),
            Category::Topic => "主题".to_string(),
            Category::Requirement => "需求".to_string(),
            Category::Task => "任务".to_string(),
            Category::Bug => "缺陷".to_string(),
            Category::Risk => "风险".to_string(),
        }
    }
}
