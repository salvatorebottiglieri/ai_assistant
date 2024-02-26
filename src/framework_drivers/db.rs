use postgres::{Client, NoTls};

pub struct DB {
    client: Client
}

impl DB {

    pub fn new(connection_string: &str) -> Result<Client, postgres::Error> {
        let client = Client::connect(connection_string, NoTls);
        client
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use dotenv::dotenv;
    use super::DB;
        
    #[test]
    fn test_should_new_connect_return_db_connection() {
        dotenv().ok();
        let db_uri = env::var("DB_CONNECTION_STRING").expect("DB_CONNECTION_STRING env variable must be set");
        let db = DB::new(&db_uri);
        assert!(db.is_ok());

    }

    #[test]
    fn test_should_new_panic_when_connection_string_is_empty() {
        let db_uri = String::from("");
        let db = DB::new(&db_uri);
        assert!(db.is_err());
        
    }

    
}




