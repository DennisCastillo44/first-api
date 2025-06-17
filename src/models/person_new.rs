use crate::{handlers::person::Person, types::person::Person as Person_};
use rusqlite::{ffi::Error, Connection, params};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonObject {
    id: u64,
    first_name: String,
    last_name: String,
    birthday: String,
    active: bool

}
pub struct PersonModel;

impl PersonModel {

    pub fn create_person(person: &Person) -> Result<u64, rusqlite::Error> {

        let conn = Connection::open("./db/school.db").unwrap();
        conn.execute("
            INSERT INTO person(name,lastName,birthday,active)
            VALUES(?1, ?2, ?3, ?4)
        ", (&person.first_name, &person.last_name, &person.birthday, &person.active)).expect("No se logró registrar a la persona");

        Ok(conn.last_insert_rowid() as u64)
    }

    pub fn get_persons() -> Result<Vec<PersonObject>, rusqlite::Error> {

        let mut persons: Vec<PersonObject> = Vec::new();
        let conn = Connection::open("./db/school.db").unwrap();
        let mut query_ = conn.prepare("SELECT * FROM person WHERE active = true").unwrap();
        let results = query_.query_map([], |row| {
            Ok(PersonObject {
                id: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                birthday: row.get(3)?,
                active: true,
            })
        }).unwrap();

        for person in results {
            persons.push(person.unwrap());
        }

        Ok(persons)
    }

    pub fn get_person(person: &u64) -> Result<PersonObject, rusqlite::Error> {
        let conn = Connection::open("./db/school.db").unwrap();
        let query_ = conn.query_row("SELECT * FROM person WHERE id = ?1", [&person], |row| {
            Ok(PersonObject {
                id: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                birthday: row.get(3)?,
                active: row.get(4)?
            })
        })?;

        Ok(query_)
    }

    pub fn update_person(person: &u64, data: &Person) -> Result<bool, rusqlite::Error> {

        let conn = Connection::open("./db/school.db").unwrap();
        conn.execute("UPDATE person SET name = ?1, lastName = ?2, birthday = ?3, active = ?4 WHERE id = ?5", params![&data.first_name, &data.last_name, &data.birthday, &data.active, person])?;
        
        Ok(true)
    }

    pub fn delete_person(person: u64) -> Result<bool, rusqlite::Error> {

        let conn = Connection::open("./db/school.db").unwrap();
        conn.execute("UPDATE person SET active = false WHERE id = ?1", params![person])?;

        Ok(true)
    }

    pub fn verify_person(person: u64) -> Result<bool, rusqlite::Error> {
        
        let conn = Connection::open("./db/school.db").unwrap();
        let mut query = conn.prepare("SELECT * FROM person WHERE id = ?1")?;
        let exist = query.exists(params![person])?;

        Ok(exist)
    }
}