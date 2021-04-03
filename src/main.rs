#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = PgConnection::establish(&database_url).unwrap();

    let book = models::NewBooks {
        title: String::from("The Little Prince"),
        author: String::from("Antoine de Saint-Exupéry"),
        published: true,
    };

    if models::Book::insert(book, &conn) {
        println!("success")
    } else {
        println!("failed")
    }
}
