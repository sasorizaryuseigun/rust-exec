<!-- SPDX-License-Identifier: AGPL-3.0-only -->

# Rust EXEC

指定した実行ファイルを引数付きでそのまま起動するための小さなRust製ラッパーです。

## できること

- 環境変数`EXEC_TARGET_PATH`で指定したプログラムを起動する
- 自分自身に渡された引数を、そのまま対象プログラムへ引き継ぐ
- 起動に失敗した場合は、Unixの慣例に合わせて終了コード`127`または`126`を返す

## 必要条件

- Rust 2024 edition対応のツールチェーン
- `EXEC_TARGET_PATH`がビルド時に設定されていること

## 使い方

ビルド時に実行対象のパスを渡してコンパイルします。

```bash
EXEC_TARGET_PATH=/path/to/program cargo build --release
```

実行時は、後ろに続く引数がそのまま対象プログラムに渡されます。

```bash
EXEC_TARGET_PATH=/usr/bin/echo cargo run -- hello world
```

上記の例では、`echo hello world`のように起動されます。
