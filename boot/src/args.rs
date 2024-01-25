use std::path::PathBuf;

use crate::config::AppConf;
use clap::{arg, command, value_parser};

pub async fn parse_config_from_shell() -> AppConf {
    let matches = command!()
        .arg(
            arg!(
                -c --config <FILE> "Sets config file"
            )
            .required(false)
            .default_value("config.toml")
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();
    let path = matches
        .get_one::<PathBuf>("config")
        .expect("解析配置文件路径失败");
    let config = tokio::fs::read_to_string(path)
        .await
        .expect("读取配置文件失败");
    toml::from_str(&config).expect("解析配置文件失败")
}
