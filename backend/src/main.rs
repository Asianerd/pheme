use rocket::State;
use sqlx::{sqlite::SqliteConnectOptions, Pool, Sqlite, SqlitePool};

#[macro_use] extern crate rocket;

mod utils;
mod cors;

mod soterius;


#[get("/")]
pub fn index() -> String {
    "pheme at your service".to_string()
}

#[get("/<page>")]
pub async fn increment(db: &State<Pool<Sqlite>>, page: String) -> String {
    let db = db.inner();

    let result = sqlx::query_as::<_, utils::ValueInt>("select count(*) from pages where name = $1;")
        .bind(&page)
        .fetch_one(db)
        .await
        .unwrap();

    if result.0 <= 0 {
        return "doesnt exist".to_string();
    }

    sqlx::query("insert into views(page, time) values($1, $2);")
        .bind(&page)
        .bind(utils::get_time())
        .execute(db)
        .await
        .unwrap();

    "ok".to_string()
}

#[get("/<page>")]
pub async fn get(db: &State<Pool<Sqlite>>, page: String) -> String {
    let db = db.inner();

    let result = sqlx::query_as::<_, utils::ValueInt>("select count(*) from pages where name = $1;")
        .bind(&page)
        .fetch_one(db)
        .await
        .unwrap();

    if result.0 <= 0 {
        return "0".to_string();
    }

    format!("{}",
        sqlx::query_as::<_, utils::ValueInt>("select count(*) from views where page = $1")
            .bind(page)
            .fetch_one(db)
            .await
            .unwrap()
            .0
    )
}

#[launch]
async fn rocket() -> _ {
    rocket::custom(rocket::config::Config::figment().merge(("port", 8006)))
        .manage(SqlitePool::connect_with(SqliteConnectOptions::new()
            .filename("db")
        ).await.unwrap())
        .mount("/", routes![index])

        .mount("/get", routes![get])
        .mount("/increment", routes![increment])

        .attach(cors::CORS)
}