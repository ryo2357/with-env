use anyhow::Result;
use clap::{Parser, Subcommand};

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
            println!("--- 展開された環境変数 ---");
            for (key, value) in &envs {
                println!("{}: {}", key, value);
            }
            println!("実行予定のコマンド: {:?}", command);

            // 3. 次のフェーズ: バリデーションとコマンド実行
        }
    }

    Ok(())
}
