use std::{env, fs::File, io::Read, time::Duration};

use chrono::NaiveTime;
use encoding_rs::WINDOWS_1252;
use sqlx::postgres::PgPoolOptions;

#[derive(Debug)]
pub struct Sub {
    pub id: usize,
    pub start: NaiveTime,
    pub end: NaiveTime,
    pub text: String,
}

// write a function that parses a srt file with a windows encoding
pub async fn parse_srt(path: &str, movie_id: i64) -> Vec<Sub> {
    let mut file = File::open(path).unwrap();

    // Read the bytes into a Vec<u8>
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();

    // Choose the encoding
    let encoding = WINDOWS_1252; // replace this with your custom encoding

    // let db = create_client().await;
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL env is not set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to postgres");

    // Decode the bytes into a string
    let (decoded_str, _, _) = encoding.decode(&bytes);
    let mut subs = Vec::new();

    let transaction = pool.begin().await.unwrap();
    for (i, sub) in decoded_str.split("\r\n\r\n").into_iter().enumerate() {
        let mut lines = sub.lines();
        let next = lines.next();
        if next.is_none() {
            continue;
        }
        let _id = next.unwrap().parse::<usize>().unwrap();

        let times = lines.next().unwrap();
        let mut times = times.split(" --> ");

        let start_str = times.next().unwrap();
        let start = parse_duration(start_str);
        let end_str = times.next().unwrap();
        let end = parse_duration(end_str);

        let text = lines.collect::<Vec<_>>().join(" ");

        sqlx::query!(
            "INSERT INTO sentence (movie_id, start_time, end_time, text, position) VALUES ($1, $2, $3, $4, $5);",
            movie_id,
            start,
            end,
            text,
            i as i64
        ).execute(&pool).await.unwrap();
    }
    transaction.commit().await.unwrap();

    dbg!(&subs);

    subs
}

// write a function that takes a srt duration as param and returns a i64 in seconds
fn parse_duration(duration: &str) -> i64 {
    let mut times = duration.split(":");
    let hours = times.next().unwrap().parse::<i64>().unwrap();
    let minutes = times.next().unwrap().parse::<i64>().unwrap();
    let mut times = times.next().unwrap().split(",");
    let seconds = times.next().unwrap().parse::<i64>().unwrap();
    hours * 60 * 60 + minutes * 60 + seconds
}
