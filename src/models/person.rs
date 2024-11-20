use crate::types::person::Person;
use rusqlite::{ffi::Error, Connection, params};

pub struct PersonModel;

impl PersonModel {

    pub fn create_person(person: &Person) -> Result<i64, Error> {

        let conn = Connection::open("./db/school.db").unwrap();

        conn.execute("
            INSERT INTO person(name,lastName,birthday,active)
            VALUES(?1, ?2, ?3, ?4)
        ", (&person.first_name, &person.last_name, &person.birthday, &person.active)).expect("No se logró registrar a la persona");

        Ok(conn.last_insert_rowid())
    }

    pub fn get_persons() -> Result<Vec<Person>, Error> {

        let mut persons: Vec<Person> = Vec::new();
        let conn = Connection::open("./db/school.db").unwrap();
        let mut query_ = conn.prepare("SELECT * FROM person WHERE active = true").unwrap();
        let results = query_.query_map([], |row| {
            Ok(Person {
                id: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                birthday: row.get(3)?,
                active: true,
                contact: None,
                user_data: None
            })
        }).unwrap();

        for person in results {
            persons.push(person.unwrap());
        }

        Ok(persons)
    }

    pub fn get_person(person: &u32) -> Result<Person, Error> {
        let conn = Connection::open("./db/school.db").unwrap();
        let query_ = conn.query_row("SELECT * FROM person WHERE id = ?1", [&person], |row| {
            Ok(Person {
                id: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                birthday: row.get(3)?,
                active: row.get(4)?,
                contact: None,
                user_data: None
            })
        }).unwrap();

        Ok(query_)
    }

    pub fn update_person(method: &String, person: &u32, mut data: Option<&Person>) -> Result<bool, Error> {

        let conn = Connection::open("./db/school.db").unwrap();
        let method_ = String::from(method);
        let exec_update = match method_.as_str() {
            "PUT" => {
                let params_update = data.unwrap();
                let active: u8 = if params_update.active {1} else {0};
                match conn.execute("UPDATE person SET name = ?1, lastName = ?2, birthday = ?3, active = ?4 WHERE id = ?5", params![&params_update.first_name, &params_update.last_name, &params_update.birthday, &active, &person]) {
                    Ok(_) => Ok(true),
                    Err(_) => Ok(false)
                }
            },
            "DELETE" => {
                match conn.execute("UPDATE person SET active = false WHERE id = ?1", [&person]) {
                    Ok(_) => Ok(true),
                    Err(_) => Ok(false)
                }
            },
            _ => Err(String::from("No es valido mi papu"))
        }.unwrap();
        
        Ok(exec_update)
    }
}