use sqlx::sqlite::SqlitePool;
use sqlx::types::chrono::NaiveDateTime;
use std::env;

#[derive(sqlx::Type, sqlx::FromRow, Debug)]
#[sqlx(transparent)]
struct Time(NaiveDateTime);

impl From<NaiveDateTime> for Time {
    fn from(ndt: NaiveDateTime) -> Self {
        Self(ndt)
    }
}

#[derive(sqlx::FromRow, Debug)]
struct Record {
    id: i64,
    timestamp: Time,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    let mytime = Time(
        chrono::NaiveDate::from_ymd_opt(2016, 7, 8)
            .unwrap()
            .and_hms_opt(9, 10, 11)
            .unwrap(),
    );
    let row_id = add_timestamp(&pool, mytime).await?;

    let record = get_timestamp(&pool, row_id).await?;

    println!("{:?}", record);

    Ok(())
}

async fn add_timestamp(pool: &SqlitePool, timestamp: Time) -> anyhow::Result<i64> {
    let mut conn = pool.acquire().await?;

    let id = sqlx::query!(
        r#"
INSERT INTO timestamps ( timestamp )
VALUES ( ?1 )
        "#,
        timestamp
    )
    .execute(&mut *conn)
    .await?
    .last_insert_rowid();

    Ok(id)
}

async fn get_timestamp(pool: &SqlitePool, id: i64) -> anyhow::Result<Record> {
    let mut conn = pool.acquire().await?;
    let rec = sqlx::query_as!(
        Record,
        r#" SELECT id, timestamp FROM timestamps WHERE id = ?1"#,
        id
    )
    .fetch_one(&mut *conn)
    .await?;

    Ok(rec)
}
