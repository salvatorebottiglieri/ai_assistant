mod entities {
    pub mod entity; 
}
mod framework_drivers {
    pub mod db;
}
use entities::entity::Conversation;
use std::env;


fn main() {
    const DB_URI: &str = "postgresql://postgres:password@localhost:5432";
    let db = framework_drivers::db::DB::new(DB_URI);

    match db {
        Ok(_) => println!("Connection established"),
        Err(error) => println!("{}", error),
    }
}
