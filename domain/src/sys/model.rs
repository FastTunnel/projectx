pub enum Config {
    SysInfo { is_init: bool },
}

impl Config {
    pub fn name(&self) -> String {
        match self {
            Config::SysInfo { .. } => "sys_info".to_string(),
        }
    }
}
