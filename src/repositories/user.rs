use diesel::r2d2::PooledConnection;
use diesel::RunQueryDsl;

use crate::config::{Connection, Pool};
use crate::schema::users::dsl::users as all_users;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Queryable, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub cpf: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub active: bool,
}

pub struct UserRepository {
    db: PooledConnection<Connection>,
}

impl UserRepository {
    pub fn new(pool: &Pool) -> Self {
        let db = pool.get().unwrap();
        Self { db }
    }

    pub fn get_all(&self) -> Vec<User> {
        all_users.load(&self.db).expect("error loanding  the books")
    }
}
