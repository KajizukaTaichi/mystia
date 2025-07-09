# Mystia
WebAssemblyに直接コンパイルするプログラミング言語

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/KajizukaTaichi/mystia)

## 目次
- [概要](#概要)
- [特徴](#特徴)
- [インストール](#インストール)
- [使用方法](#使用方法)
- [言語構文](#言語構文)
- [サンプルコード](#サンプルコード)
- [アーキテクチャ](#アーキテクチャ)
- [開発](#開発)
- [貢献](#貢献)
- [ライセンス](#ライセンス)

## 概要

MystiaはWebAssembly (WASM)にコンパイルされるよう設計されたモダンなプログラミング言語です。Rustの安全性とパフォーマンスを、OCamlのような関数型プログラミング言語の表現力と組み合わせています。静的型付けと型推論、メモリ安全性、JavaScript環境とのシームレスな統合を特徴としています。

## 特徴

### 静的型付け
- **コンパイル時型チェック**: 型システムに関するエラーはコンパイル時に検出されます
- **型推論**: コンパイラが自動的に型を推論し、ボイラープレートコードを削減します
- **メモリ安全性**: ヌルポインタ参照などの一般的なプログラミングエラーを防ぎます

### Rust/ML風構文
- **馴染みやすい構文**: RustとOCamlにインスパイアされています
- **`let`文**: 変数定義には`let`キーワードを使用します
- **パターンマッチング**: 強力なパターンマッチング機能
- **関数型プログラミング**: 第一級関数とクロージャ

### WebAssembly統合
- **高速実行**: 効率的なWebAssemblyバイトコードにコンパイル
- **JavaScript相互運用**: JavaScript APIとのシームレスな統合
- **Web・Node.js対応**: ブラウザとNode.js環境の両方で動作
- **仮想DOM**: 仮想DOMサポート付きの組み込みUIフレームワーク

### モダンな言語機能
- **マクロ**: コンパイル時コード生成
- **モジュールシステム**: インポート機能を持つ整理されたコード構造
- **エラー処理**: 堅牢なエラー処理メカニズム
- **パッケージ管理**: 統合された依存関係管理

## インストール

### 前提条件
- Rust（最新安定版）
- Node.js（v16以上）
- wasm-pack

### ソースからのビルド
```bash
git clone <repository-url>
cd mystia/main
cargo build --release
```

### WebAssemblyバインディングのビルド
```bash
# ビルドスクリプトを実行
./build_wasm.sh

# または手動で:
cd wasm
wasm-pack build --target nodejs
wasm-pack build --target web
```

## 使用方法

### コマンドラインインターフェース
```bash
# Mystiaファイルをコンパイル
./target/release/mystia example/fizzbuzz.ms

# 型推論サマリーを表示
./target/release/mystia example/fizzbuzz.ms --summary

# Node.jsランタイムでコンパイル・実行
node run.mjs example/fizzbuzz.ms
```

### REPLモード
```bash
node repl.mjs
```

### Web統合
```html
<!DOCTYPE html>
<html>
<head>
    <script type="module">
        import { mystia } from './docs/runtime/web.mjs';
        
        const code = `
            load print(_: str): void;
            print("Hello, WebAssembly!")
        `;
        
        mystia(code);
    </script>
</head>
</html>
```

## 言語構文

### 変数と関数
```mystia
// 変数宣言
let x = 42;
let message = "Hello, world!";

// 関数定義
let add(a: int, b: int) -> int = a + b;

// 型推論付き関数
let multiply(a, b) = a * b;
```

### 制御フロー
```mystia
// 条件式
let result = if x > 0 then "positive" else "non-positive";

// ループ
let i = 0;
while i < 10 loop {
    print(i: str);
    let i + 1
}
```

### データ型
```mystia
// 基本型
let number: int = 42;
let decimal: num = 3.14;
let text: str = "Hello";
let flag: bool = true;

// コレクション
let numbers = [1, 2, 3, 4, 5];
let person = @{ name: "Alice", age: 30 };

// カスタム型
type Status = Success | Error | Pending;
```

### マクロ
```mystia
// マクロ定義
macro times(n, block) = {
    let i = 0;
    while i < n loop {
        block;
        let i + 1
    }
};

// 使用例
5.times({
    print("繰り返しメッセージ")
});
```

### モジュールシステム
```mystia
// 外部関数のインポート
load print(_: str): void;
load to_str(n: num): str;

// パブリック関数
pub let main() = {
    print("Hello from Mystia!")
};
```

## サンプルコード

### FizzBuzz
```mystia
load to_str(n: num): str;
load print(n: str): void;

let fizzbuzz(n: int) = {
    if n % 15 == 0 then "FizzBuzz"
    else if n % 3 == 0 then "Fizz"
    else if n % 5 == 0 then "Buzz"
    else n: str
};

let i = 1;
while i <= 100 loop {
    i.fizzbuzz().print();
    let i + 1
}
```

### カウンターアプリ
```mystia
type Elm = int;

load new_elm(tag: str, parent: Elm): Elm;
load upd_elm(id: Elm, prop: str, content: str): void;
load evt_elm(id: Elm, name: str, func: str): void;

pub let model = @{
    title: "Counter App",
    count: 0,
    layout: @{
        panel: null:Elm
    }
};

let view() = {
    let formatted = "Number: " + (model.count: str);
    upd_elm(model.layout.panel, "innerHTML", formatted)
};

pub let inc_btn() = {
    let model.count + 1;
    view()
};
```

その他のサンプルは `example/` ディレクトリにあります。

## アーキテクチャ

### プロジェクト構造
```
mystia/
├── core/           # 核となる言語実装
│   ├── src/
│   │   ├── lexer.rs    # トークン化
│   │   ├── expr.rs     # 式の解析
│   │   ├── stmt.rs     # 文の解析
│   │   ├── type.rs     # 型システム
│   │   └── value.rs    # 値の型
│   └── Cargo.toml
├── app/            # コマンドラインインターフェース
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
├── wasm/           # WebAssemblyバインディング
│   ├── src/
│   │   └── lib.rs
│   └── Cargo.toml
├── docs/           # ドキュメントとランタイム
│   ├── runtime/    # JavaScriptランタイム
│   └── wasm/       # 生成されたWASMバインディング
├── example/        # サンプルプログラム
└── build_wasm.sh   # ビルドスクリプト
```

### コンパイルパイプライン
1. **字句解析**: ソースコードがトークンストリームにトークン化されます
2. **構文解析**: トークンが抽象構文木（AST）に解析されます
3. **型チェック**: 型推論による静的型解析
4. **コード生成**: ASTがWebAssembly Text format（WAT）にコンパイルされます
5. **WebAssembly**: WATがバイナリWebAssembly形式にコンパイルされます

### ランタイム環境
- **Node.jsランタイム**: ファイルシステムアクセス付きのフル機能ランタイム
- **Webランタイム**: DOM統合付きのブラウザ互換ランタイム
- **標準ライブラリ**: math、OS、random、datetime、time操作のための組み込みモジュール

## 開発

### 開発環境のセットアップ
```bash
# Rustをインストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# wasm-packをインストール
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# クローンとビルド
git clone <repository-url>
cd mystia/main
cargo build
```

### テストの実行
```bash
# Rustテストを実行
cargo test

# サンプルのテスト
node run.mjs example/fizzbuzz.ms
node run.mjs example/app.ms
```

### ドキュメントのビルド
```bash
# Rustドキュメントを生成
cargo doc --open

# WebAssemblyバインディングをビルド
./build_wasm.sh
```

## 貢献

貢献大歓迎！以下の手順に従ってください：

1. リポジトリをフォーク
2. 機能ブランチを作成 (`git checkout -b feature/amazing-feature`)
3. 変更を実装
4. 変更に対するテストを追加
5. すべてのテストが通ることを確認 (`cargo test`)
6. 変更をコミット (`git commit -m 'Add amazing feature'`)
7. ブランチにプッシュ (`git push origin feature/amazing-feature`)
8. プルリクエストを作成

### コードスタイル
- Rustの命名規則に従う
- コードフォーマットには `rustfmt` を使用
- パブリックAPIにはドキュメントを追加
- 新機能にはテストを含める

### 問題の報告
バグの報告や機能要求にはGitHubのissueを使用してください。以下を含めてください：
- Mystiaのバージョン
- オペレーティングシステム
- 最小限の再現ケース
- 期待される動作と実際の動作

## ライセンス

このプロジェクトはMITライセンスの下でライセンスされています - 詳細は [LICENSE](LICENSE) ファイルをご覧ください。

## リンク

- [ドキュメント](docs/index.html)
- [サンプル](example/)
- [DeepWiki](https://deepwiki.com/KajizukaTaichi/mystia)

---

*Mystia - WebAssembly時代のモダンプログラミング言語*
