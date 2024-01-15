use json::JsonValue;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ResourceType {
    Space,
    WorkItem,
}
#[allow(dead_code)]
impl ResourceType {
    pub fn to_string(&self) -> String {
        match self {
            ResourceType::Space => "space".to_string(),
            ResourceType::WorkItem => "work_item".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "space" => Option::from(ResourceType::Space),
            "work_item" => Option::from(ResourceType::WorkItem),
            _ => None,
        }
    }

    pub fn name(&self) -> String {
        match self {
            ResourceType::Space => "项目空间".to_string(),
            ResourceType::WorkItem => "工作项".to_string(),
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
    MultiSelect,
    Switch,
    DateTime,
}

#[allow(dead_code)]
impl FieldType {
    pub fn to_string(&self) -> String {
        match self {
            FieldType::Intput => "input".to_string(),
            FieldType::Textarea => "textarea".to_string(),
            FieldType::Select => "select".to_string(),
            FieldType::MultiSelect => "multi_select".to_string(),
            FieldType::Switch => "switch".to_string(),
            FieldType::DateTime => "date_time".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "input" => Option::from(FieldType::Intput),
            "textarea" => Option::from(FieldType::Textarea),
            "select" => Option::from(FieldType::Select),
            "multi_select" => Option::from(FieldType::MultiSelect),
            "switch" => Option::from(FieldType::Switch),
            "date_time" => Option::from(FieldType::DateTime),
            _ => None,
        }
    }

    pub fn name(&self) -> String {
        match self {
            FieldType::Intput => "输入框".to_string(),
            FieldType::Textarea => "文本框".to_string(),
            FieldType::Select => "下拉框".to_string(),
            FieldType::MultiSelect => "多选框".to_string(),
            FieldType::Switch => "开关".to_string(),
            FieldType::DateTime => "日期时间".to_string(),
        }
    }
}