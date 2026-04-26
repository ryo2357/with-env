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
    pub allow_command: Vec<Vec<String>>,
    pub shell: Option<String>, //任意指定とする
}

impl Settings {
    pub fn validate_path(&self, current_dir: &std::path::Path) -> Result<()> {
        let is_allowed = self
            .allow_run_path
            .iter()
            .any(|allowed| current_dir.starts_with(allowed));

        if !is_allowed {
            anyhow::bail!("実行パスが許可されていません: {:?}", current_dir);
        }
        Ok(())
    }

    pub fn validate_command(&self, command: &[String]) -> Result<()> {
        // 1. メタ文字のチェック（追加コマンドの実行を防止）
        #[cfg(debug_assertions)]
        let meta_chars = ['&', '|', ';', '>', '<', '`', '(', ')'];

        #[cfg(not(debug_assertions))]
        let meta_chars = ['&', '|', ';', '>', '<', '`', '$', '(', ')'];

        for arg in command {
            if arg.chars().any(|c| meta_chars.contains(&c)) {
                anyhow::bail!("コマンドに不正な文字が含まれています: {}", arg);
            }
        }

        // 2. 許可リストとの前方一致照合
        // 例: allowed が ["cargo", "test"] の場合、
        // command が ["cargo", "test", "mod2"] なら許可される
        let is_allowed = self
            .allow_command
            .iter()
            .any(|allowed| command.starts_with(allowed));

        if !is_allowed {
            anyhow::bail!("許可されていないコマンドです: {:?}", command);
        }
        Ok(())
    }
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
            dirs::home_dir()
                .context("ホームディレクトリが見つかりませんでした")?
                .join(".config")
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
