# Rust + DDD + モジュラモノリスのサンプルコードリポジトリ

- 直近の案件で実装したコードの一部を抜粋するような形でサンプルリポジトリを作りました
- 誰かがRustでWebAPIを開発する時の何かの参考になれば良いなと思って作りました
- こう直したらどう？とか、これってどういうこと？みたいな質問があれば気軽にご連絡ください！

## サンプルアプリケーションの動かし方

### .envの作成

- .exampleをコピーして、`.env` と `.env.test` を作成してください
- docker-compose.ymlを参考に、.env.testのDATABASE_URLは変更してください

### dockerを起動

以下のコマンドでDockerコンテナが起動します

```bash
docker-compose up -d
```

同時にAPIサーバーも起動するようになっているので、test.httpを使ってリクエストを送ることもできます

### マイグレーション

```bash
./scripts db:dev:migration run
```

### シード

```bash
./scripts db:dev:seed
```

### 単体テスト

```bash
./scripts test:all
```

### 統合テスト

```bash
./scripts db:test:migration run
./scripts db:test:seed
./scripts test:integration
```

## アーキテクチャ

以下の考え方を参考にしています

一部うまく実装できていない部分があったりしますが、本番用のプロジェクトから一部だけ引っぱり出したりした都合上、そこはお許し...

- モジュラモノリス
- DDD
- オニオンアーキテクチャ

## モジュールの可視性

- 各モジュールは外部にpresentationレイヤーのみを公開します
- presentationレイヤーでは、ドメイン知識などが現れないシグネチャを定義し、モジュ
ール内の情報漏洩を防ぎます
- 各モジュールのmod.rsファイルで、presentationレイヤーのみを公開するように設定が可能です

## ディレクトリ構造

### migrations

- dieselで使用するマイグレーションファイルが入っています

### src

- アプリケーションのソースコードが入っています

#### src/bin

- アプリケーションのエントリーポイントが入っています

##### src/bin/seed.rs

- シードデータを投入する際に実行します

##### src/bin/api.rs

- APIサーバーを起動する際に実行します

#### src/modules

- 各モジュールを定義していきます

##### src/modules/common

- 全てのモジュールで共有して使うような処理を定義します

##### src/modules/diesel

- dieselに関連する処理を定義します

##### src/modules/task

- taskモジュールのソースコードが入っています

###### src/modules/task/domain

- ドメイン層の処理を定義します

###### src/modules/task/infra

- インフラ層の処理を定義します

###### src/modules/task/presentation

- プレゼンテーション層の処理を定義します

###### src/modules/task/usecase

- ユースケース層の処理を定義します

##### src/modules/user

- taskと同じなので省略します

#### src/scenario

- HTTPエンドポイントと紐づけるシナリオを定義します
- マイクロサービスの文脈で登場するSagaパターンのオーケストレーターのような使い方を想定しています
- 今回のサンプルリポジトリでは、実装できていませんが、DBのトランザクション管理をしても問題ありません
  - 現在の実装だと、presentation層の関数に、`&mut`ではない`conn`を渡す必要がありますが、dieselでトランザクションを管理しようとすると、必ず`&mut`な`conn`を渡す必要が生まれてしまいます
  - ここの制御が現時点ではうまくできていません

#### src/lib.rs

- モジュール宣言を行っています

#### src/routes.rs

- ルーティングを定義しています

#### src/schema.rs

- dieselが自動生成するスキーマファイルです
- 自分で修正することはありません

### tests

- 統合テストを書きます
- 統合テストと単体テストを分けて動かすために、`Cargo.toml` に 統合テスト用の `features` を定義しています
- 統合テストのモジュールには必ず `#[cfg(feature = "integration")]` をつける必要があります
- これを書かないと、単体テストのときに統合テストも動いてしまいます
