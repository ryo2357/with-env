# with-env

`.env` ファイルに直接キーを書かず、プレースホルダで管理して流出リスクを低減するためのCLIツール

## インストール方法

リポジトリをクローンし、以下のコマンドを実行します。

```bash
cargo install --path .
```

## 設定方法

1. 設定ディレクトリの作成
   OSに関わらず、以下のディレクトリに設定ファイルを配置します。
   `~/.config/with/` (Windowsの場合は `C:\Users\<ユーザー名>\.config\with\`)

2. シークレットの設定 (secrets.json)
   機密情報を保持するファイルです。

```json
{
  "keys": {
    "API_KEY": "your-key",
    "DATABASE_PASSWORD": "your-password"
  }
}
```

3. アプリケーション設定 (settings.json)

実行許可パスやコマンド、使用するシェルを定義します。

```json
{
  "allow_run_path": ["/path/to/your/project"],
  "allow_command": [
    ["cargo", "run"],
    ["npm", "start"]
  ],
  "shell": "nu"
}
```

allow_command: 指定した配列で始まるコマンドのみ許可されます（例: ["cargo", "run"] は cargo run -- --debug も許可します）。
shell: (オプション) 指定すると shell -c "command args" の形式で実行されます。

## 使い方

1. .env ファイルの準備
   プロジェクトのルートにある .env ファイルにプレースホルダを記述します。

```.env
# secrets.json から展開される
API_KEY=WITH{{OPENAI_API_KEY}}
# 直接指定も可能
DEBUG=true
```

2. コマンドの実行
   with run -- の後に実行したいコマンドを続けます。
   `with run -- cargo run`
