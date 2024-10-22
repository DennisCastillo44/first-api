pub mod database_mod {
    use rusqlite::{Connection, Result};

    pub struct Database_ {
        pub db: String,
    }
    
    impl Database_ {
        
        pub fn init(path: String) -> Database_ {
            Database_ {db: path}
        }

        pub fn import_tables(&self) -> Result<()> {
            
            //let connection = Connection::open(&self.db);
            let connection = if &self.db == "" { Connection::open_in_memory() } else { Connection::open(&self.db) };
            let mut binding = connection?;
            let conn = binding.transaction()?;

            println!("Creando tablas");
            conn.execute("
                CREATE TABLE IF NOT EXISTS person(
                    id INTEGER PRIMARY KEY,
                    name VARCHAR(255) NOT NULL,
                    lastName VARCHAR(255) NOT NULL,
                    birthday DATE NOT NULL,
                    active BOOLEAN DEFAULT true
                );
            ", [])?;

            conn.execute("
                CREATE TABLE IF NOT EXISTS contact(
                    email VARCHAR(255),
                    phone VARCHAR(255),
                    person_id INTEGER NOT NULL
                );
            ", [])?;

            conn.commit()?;
            binding.close().unwrap();

            println!("Tablas creadas");
            Ok(())
        }
    }
}