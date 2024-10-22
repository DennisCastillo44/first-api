mod server_;
mod database;
mod router;
mod controllers;
mod models;
mod types;
use database::database_mod::Database_;
//use database::database_mod::Database_;
pub use server_::Server_;

fn main() {

    println!("Inciando...");
    let database = Database_::init(String::from("./db/school.db"));
    database.import_tables().unwrap();

    let server = Server_;
    server.run_server_().unwrap();
}
