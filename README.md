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
- **コンパイル時型チェック**: 型システムに関する問題はコンパイル時に検出されます
- **型推論**: コンパイラが自動的に型を推論し、ボイラープレートコードを削減します
- **メモリ安全性**: ヌル参照などの一般的なバグをヌル確認演算子`?`で防ぎます

### Rust/ML風構文
- **馴染みやすい構文**: RustとOCamlにインスパイアされています
- **`let`文**: 変数定義には`let`キーワードを使用します
- **マクロ**: コンパイル時に型に縛られずコード生成

### WebAssembly統合
- **高速実行**: 効率的なWebAssemblyバイトコードにコンパイル
- **JavaScript相互運用**: JavaScript APIとのシームレスな統合
- **Web・Node.js対応**: ブラウザとNode.js環境の両方で動作
- **仮想DOM**: 仮想DOMサポート付きの組み込みUIフレームワーク

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
            load alert(_: str): void;
            alert("Hello, WebAssembly!")
        `;
        
        mystia(code);
    </script>
</head>
</html>
```

## 言語構文

### 変数と関数
```mystia
~~ 変数宣言 ~~
pub let x = 42; ~~ グローバル ~~
let message = "Hello, world!";

~~ 関数定義 ~~
let add(a: int, b: int) = a + b;
```

### 制御フロー
```mystia
~~ 条件式 ~~
let result = {
    if x > 0 then "positive"
    else if x == 0 then "zero"
    else ""negative"
};

~~ ループ ~~
let i = 0;
while i < 10 loop {
    print(i: str);
    let i + 1
}
```

### データ型
```mystia
~~ 基本型 ~~
let number = 42: int;
let decimal = 3.14: num;
let text = "Hello": str;
let flag = true: bool;

~~ コレクション ~~ 
let numbers = [1, 2, 3, 4, 5];
let person = @{ name: "Alice", age: 30 };

~~ カスタム型 ~~
type Status = ( Success | Error | Pending );
```

### マクロ
```mystia
~~ マクロ定義 ~~
macro times(n, block) = {
    let i = 0;
    while i < n loop {
        block;
        let i + 1
    }
};

~~ 使用例 ~~
5.times({
    print("繰り返しメッセージ")
});
```

### モジュールシステム
```mystia
~~ 外部関数のインポート ~~
load print(_: str): void;
load to_str(n: num): str;

~~ パブリック関数 ~~
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

### リンクリスト
```mystia
type LinkList = @{ value: int, next: LinkList };

let car(self: LinkList) = self.value;
let cdr(self: LinkList) = self.next;
let node(value: int) = memcpy(@{ value: value, next: LinkList! });
let append(self: LinkList, other: LinkList) = {
    let original_object = self;
    while self.next? loop { let self = self.next };
    let self.next = other;
    original_object
};

let a = node(1);
let b = node(2).append(node(3));
a.append(b)
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
