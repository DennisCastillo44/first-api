use rusqlite::{params, Connection };
use serde::{Deserialize, Serialize};
use crate::{handlers::user::User, lib_::hashing256::{Hash, Salt}};


#[derive(Debug, Serialize, Deserialize)]
pub struct UserModel {
    pub user_id: u64,
    pub username: String,
    pub password: String,
    pub person_id: u64
}

impl UserModel {
    
    pub fn create_user(user: &User) -> Result<i64, rusqlite::Error>{

        let salt = Salt::generate_salt();
        let conn = Connection::open("./db/school.db").unwrap();
        let mut user_password = String::new();
        let mut password_salted = String::from(&salt);
        password_salted.push_str(&user.password);
        let hash_ = Hash::new( password_salted).generate_hash();

        user_password.push_str(&hash_);
        user_password.push_str("??_");
        user_password.push_str(&salt);

        conn.execute("
            INSERT INTO users(username, password, idPerson)
            VALUES(?1, ?2, ?3)", (&user.username, user_password, &user.person_id)).unwrap();

        Ok(conn.last_insert_rowid())
    }

    pub fn get_user_by_username(username: String) -> Result<UserModel, rusqlite::Error> {
        
        let conn = Connection::open("./db/school.db").unwrap();
        let user_ = conn.query_row("SELECT * FROM users WHERE username = ?1", [username], |row| {
            Ok(UserModel {
                user_id: row.get(0)?,
                username: row.get(1)?,
                password: row.get(2)?,
                person_id: row.get(3)?,
            })
        })?;
        
        Ok(user_)
    }

    pub fn verify_user(user: &User) -> Result<bool, rusqlite::Error> {

        let conn = Connection::open("./db/school.db").unwrap();
        let mut query = conn.prepare("SELECT * FROM users WHERE username = ?1")?;
        let exists = query.exists(params![&user.username])?;

        Ok(exists)
    }
}