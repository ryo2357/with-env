# プロジェクト仕様書: with-env

## プロジェクト概要

AIエージェント環境下でのシークレット情報（APIキー等）の流出リスクを低減するためのCLIツールです。
`.env` ファイルにプレースホルダ（`WITH{{KEY}}`）を記述し、実行時のみ実数値に展開して環境変数として注入します。

## プロジェクト構造

- [src/main.rs](src/main.rs): エントリポイント。CLI引数の解析と実行フローの制御。
- [src/config.rs](src/config.rs): 設定ファイル（`settings.json`）とシークレットファイル（`secrets.json`）の読み込みと管理。
- [src/env.rs](src/env.rs): `.env` ファイルのパースとプレースホルダの展開ロジック。

## データ構造

### 設定ディレクトリの解決

- **Debugビルド時**: プロジェクトルートの `.dev_config/` ディレクトリを参照。
- **Releaseビルド時**: OS標準の設定ディレクトリ（例: `~/.config/with-env/`）を参照。

### secrets.json

機密情報を保持するJSONファイル。

```json
{
  "keys": {
    "KEY_NAME": "ACTUAL_VALUE"
  }
}
```

### settings.json

実行制限を定義するJSONファイル。

```json
{
  "allow_run_path": ["PATH_TO_ALLOW"],
  "allow_command": [["command", "subcommand"]]
}
```

- `allow_command` は、コマンドと引数の配列のリストとして定義されます。

### .env ファイル

プレースホルダを含む環境変数定義ファイル。

- 形式: `VARIABLE_NAME=WITH{{SECRET_KEY}}`
- 実行時に `secrets.json` の `SECRET_KEY` に対応する値に置換されます。
