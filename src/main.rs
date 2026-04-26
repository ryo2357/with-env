use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::process::Command;

mod config;
mod env;

#[derive(Parser)]
#[command(name = "with-env")]
#[command(about = "APIキー隠蔽・管理CLIツール", long_about = None)]
struct Cli {
    #[command(subcommand)] // @ ではなく #[ ] です
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        /// 実行するコマンド
        #[arg(last = true)]
        command: Vec<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { command } => {
            if command.is_empty() {
                anyhow::bail!("実行するコマンドを指定してください。例: with-env run -- ls");
            }

            // 1. 設定の読み込み
            let config = config::Config::load()?;

            // 2. 環境変数の展開
            let envs = env::EnvManager::get_expanded_envs(&config)?;
            // デバッグ用にキーと値をすべて表示
            if cfg!(debug_assertions) {
                println!("--- 展開された環境変数 ---");
                for (key, value) in &envs {
                    println!("{}: {}", key, value);
                }
                println!("実行予定のコマンド: {:?}", command);
            }

            // 4. セキュリティバリデーション
            let current_dir = std::env::current_dir()?;
            config.settings.validate_path(&current_dir)?;
            config.settings.validate_command(&command)?;

            // 5. コマンドの実行
            let mut child = if let Some(shell_cmd) = &config.settings.shell {
                let mut cmd = Command::new(shell_cmd);
                // Nushell や多くのシェルで -c は「後に続く文字列をコマンドとして実行」するフラグ
                cmd.arg("-c").arg(command.join(" "));
                cmd
            } else {
                // シェル指定がない場合は直接実行
                let mut cmd = Command::new(&command[0]);
                cmd.args(&command[1..]);
                cmd
            };

            // 展開された環境変数を注入
            for (key, value) in envs {
                child.env(key, value);
            }

            // プロセスの実行と待機
            let status = child
                .status()
                .with_context(|| format!("コマンドの実行に失敗しました: {}", command[0]))?;
            // 終了コードを継承して終了
            std::process::exit(status.code().unwrap_or(1));
        }
    }

    Ok(())
}
