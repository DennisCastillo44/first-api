use rusqlite::{ffi::Error, params, Connection };
use crate::lib_::hashing256::{Hash, Salt};

pub struct User {
    pub id_user: Option<i64>,
    pub username: String,
    pub password: String,
    pub id_person: Option<u64>
}

impl User {
    
    pub fn new(username: &String, password: &String, id_person: Option<u64>) -> User {
        User {id_user: None, username: username.to_string(), password: password.to_string(), id_person: id_person}
    }

    pub fn create_user(&mut self) -> Result<i64, rusqlite::Error>{

        let salt = Salt::generate_salt();
        let conn = Connection::open("./db/school.db").unwrap();
        let mut user_password = String::new();
        let mut password_salted = String::from(&salt);
        password_salted.push_str(&self.password);
        let hash_ = Hash::new( password_salted).generate_hash();

        user_password.push_str(&hash_);
        user_password.push_str("??_");
        user_password.push_str(&salt);
        self.password = user_password; //--> hashed
        conn.execute("
            INSERT INTO users(username, password, idPerson)
            VALUES(?1, ?2, ?3)", (&self.username, &self.password, &self.id_person)).unwrap();

        Ok(conn.last_insert_rowid())
    }

    pub fn get_user_by_username(&self) -> Result<User, rusqlite::Error> {
        
        let conn = Connection::open("./db/school.db").unwrap();
        let user_ = conn.query_row("SELECT * FROM users WHERE username = ?1", [&self.username], |row| {
            Ok(User {
                id_user: row.get(0)?,
                username: row.get(1)?,
                password: row.get(2)?,
                id_person: row.get(3)?,
            })
        });
        
        match user_ {
            Ok(u) => Ok(u),
            Err(e) => Err(e)
        }
    }
}