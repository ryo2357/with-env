use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "with-env")]
#[command(about = "APIキー隠蔽・管理CLIツール", long_about = None)]
struct Cli {
    #[command(subcommand)] // @ ではなく #[ ] です
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 指定したコマンドを環境変数を注入して実行します
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
            println!("実行予定のコマンド: {:?}", command);
        }
    }

    Ok(())
}
