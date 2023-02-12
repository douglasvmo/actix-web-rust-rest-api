use diesel::r2d2::PooledConnection;
use diesel::{self,RunQueryDsl, QueryDsl, ExpressionMethods};

use crate::errors::ServiceError;
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

    pub fn get_all(&self) -> Result<Vec<User>, ServiceError> {
        Ok(users.load(&self.db)?)
    }
    pub fn find(&self, user_id: i32) -> Result<User, ServiceError> {
        Ok(users.filter(users::id.eq(user_id)).first(&self.db)?)
    }

    pub fn create(&self, new_user:NewUser) -> Result<User, ServiceError>{
       Ok(diesel::insert_into(users::table)
        .values(new_user)
        .get_result(&self.db)?)
    }
}
