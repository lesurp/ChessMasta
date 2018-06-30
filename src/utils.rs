use diesel::sqlite::SqliteConnection;
use diesel::Connection;
use dotenv::dotenv;
use schema::moves;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[derive(Deserialize, Insertable)]
#[table_name = "moves"]
struct MoveForm {
    pub parent: Option<i32>,
    pub turn: i32,
    pub name_: String,
    pub special_name: Option<String>,
    pub line_description: Option<String>,
}
