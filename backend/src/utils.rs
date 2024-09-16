use std::time::{SystemTime, UNIX_EPOCH};

use sqlx::prelude::FromRow;

pub fn get_time() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards (???)")
        .as_secs() as i64
}

pub fn parse_response<T: serde::Serialize>(data: Result<T, T>) -> String {
    match data {
        Ok(d) => format!(r#"{{"type":"success","data":"{}"}}"#, urlencoding::encode(serde_json::to_string(&d).unwrap().as_str()).to_string()),
        Err(e) => format!(r#"{{"type":"fail","error":"{}"}}"#, urlencoding::encode(serde_json::to_string(&e).unwrap().as_str()).to_string())
    }
}

#[derive(FromRow, Debug)]
pub struct Value(pub f64);
// f64 doesnt implement FromRow for some reason???

#[derive(FromRow, Debug)]
pub struct ValueInt(pub i64);
// cant believe i have to do this
