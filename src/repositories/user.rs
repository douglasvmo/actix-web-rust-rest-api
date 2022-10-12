use diesel::r2d2::PooledConnection;
use diesel::{self,RunQueryDsl};

use crate::config::{Connection, Pool};
use crate::schema::users::dsl::*;
use crate::schema::users;
use serde::{Deserialize, Serialize};

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

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "users"]
pub struct  NewUser {
    pub name: String,
    pub email: String,
    pub cpf: String, 
    pub password: String, 
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
        users.load(&self.db).expect("error loanding  the books")
    }
    pub fn create(&self, new_user:NewUser) -> bool {
        diesel::insert_into(users::table)
        .values(new_user)
        .execute(&self.db)
        .is_ok()
    }
}
