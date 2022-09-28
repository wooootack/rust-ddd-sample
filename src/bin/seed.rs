use chrono::{TimeZone, Utc};
use diesel::{prelude::*, Connection, PgConnection};
use rust_ddd_sample::{
  modules::diesel::{tasks::Task, user::User},
  schema::{tasks::dsl::*, users::dsl::*},
};
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let target_env = args[1].as_str();

  match target_env {
    "dev" => {
      println!("開発環境のデータベースにシードデータを投入します");
      dotenvy::from_filename(".env").ok();
    }
    "test" => {
      println!("テスト環境のデータベースにシードデータを投入します");
      dotenvy::from_filename(".env.test").ok();
    }
    _ => {
      println!("実行対象の環境を指定してください");
      println!("例: cargo run dev,  cargo run test");
      return;
    }
  }

  dotenvy::from_filename(".env").ok();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let conn = &mut PgConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {database_url}"));

  match conn.transaction::<(), diesel::result::Error, _>(|tx| {
    truncate(tx)?;
    seed(tx)
  }) {
    Ok(_) => println!("シードデータの投入に成功しました"),
    Err(e) => println!("シードデータの投入に失敗しました: {e:?}"),
  }
}

// FIXME: deleteで対応しているが、truncateできたらする
fn truncate(conn: &mut PgConnection) -> Result<(), diesel::result::Error> {
  println!("シードデータのクリーンナップ");
  diesel::delete(tasks).execute(conn)?;
  diesel::delete(users).execute(conn)?;
  Ok(())
}

fn seed(conn: &mut PgConnection) -> Result<(), diesel::result::Error> {
  let mock_date = Utc
    .datetime_from_str("2023-01-01 10:00:00", "%Y-%m-%d %H:%M:%S")
    .unwrap();

  diesel::insert_into(users)
    .values([User {
      id: "01GSJ8XX68PXMNKGFWTM4G06MQ".to_owned(),
      first_name: "太郎".to_owned(),
      last_name: "山田".to_owned(),
      mail_address: "taro_yamada@example.com".to_owned(),
      age: 25,
      created_at: mock_date,
      updated_at: mock_date,
    }])
    .execute(conn)?;
  diesel::insert_into(tasks)
    .values([Task {
      id: "01GSJ8Y8HJY585M0SKJZG2C4HF".to_owned(),
      title: "テストタイトル".to_owned(),
      body: "テストボディ".to_owned(),
      user_id: "01GSJ8XX68PXMNKGFWTM4G06MQ".to_owned(),
      created_at: mock_date,
      updated_at: mock_date,
    }])
    .execute(conn)?;

  Ok(())
}
