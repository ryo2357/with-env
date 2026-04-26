use crate::config::Config;
use anyhow::Result;
use std::collections::HashMap;

pub struct EnvManager;

impl EnvManager {
    pub fn get_expanded_envs(config: &Config) -> Result<HashMap<String, String>> {
        let mut expanded_envs = HashMap::new();

        // 開発時は設定ディレクトリ直下の .env、リリース時はカレントディレクトリ直下の .env
        let env_path = if cfg!(debug_assertions) {
            config.config_dir.join(".env")
        } else {
            std::env::current_dir()?.join(".env")
        };

        // 指定したパスの .env ファイルをパース
        // ファイルが存在しない場合はエラーにせず空のMapを返すか、必要に応じて処理
        if env_path.exists() {
            for item in dotenvy::from_path_iter(&env_path)? {
                let (key, value) = item?;

                let mut final_value = value.clone();

                if value.starts_with("WITH{{") && value.ends_with("}}") {
                    let secret_key = &value[6..value.len() - 2];
                    if let Some(secret_val) = config.secrets.keys.get(secret_key) {
                        final_value = secret_val.clone();
                    } else {
                        anyhow::bail!("Secrets にキー '{}' が見つかりません。", secret_key);
                    }
                }
                expanded_envs.insert(key, final_value);
            }
        }

        Ok(expanded_envs)
    }
}
