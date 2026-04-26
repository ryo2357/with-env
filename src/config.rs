use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Secrets {
    pub keys: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub allow_run_path: Vec<PathBuf>,
    pub allow_command: Vec<Vec<String>>, // String から Vec<String> に変更
}

pub struct Config {
    pub secrets: Secrets,
    pub settings: Settings,
    pub config_dir: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_dir = if cfg!(debug_assertions) {
            // 開発ビルド時はカレントディレクトリの .dev_config" を使用
            std::env::current_dir()?.join(".dev_config")
        } else {
            // リリースビルド時は OS 標準の設定ディレクトリを使用
            dirs::config_dir()
                .context("設定ディレクトリが見つかりませんでした")?
                .join("with-env")
        };

        let secrets_path = config_dir.join("secrets.json");
        let settings_path = config_dir.join("settings.json");

        let secrets_content = std::fs::read_to_string(&secrets_path).with_context(|| {
            format!(
                "シークレットファイルが読み込めませんでした: {:?}",
                secrets_path
            )
        })?;
        let secrets: Secrets = serde_json::from_str(&secrets_content)?;

        let settings_content = std::fs::read_to_string(&settings_path)
            .with_context(|| format!("設定ファイルが読み込めませんでした: {:?}", settings_path))?;
        let settings: Settings = serde_json::from_str(&settings_content)?;

        Ok(Config {
            secrets,
            settings,
            config_dir,
        })
    }
}
