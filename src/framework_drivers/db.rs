use postgres::{Client, NoTls};
use crate::entities::entity::Conversation;


pub trait ConversationDAO {

    fn save(&self, conversation: &Conversation);

    fn get(&self) -> &Conversation;

    fn delete(&self, id: i32) -> bool;
    
}




pub struct DbClient {
    client: Client
}

impl DbClient {

    pub fn new(connection_string: &str) -> Result<Client, postgres::Error> {
        let client = Client::connect(connection_string, NoTls);
        client
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use dotenv::dotenv;
    use super::DbClient;
        
    #[test]
    fn test_should_new_connect_return_db_connection() {
        dotenv().ok();
        let db_uri = env::var("DB_CONNECTION_STRING").expect("DB_CONNECTION_STRING env variable must be set");
        let db = DbClient::new(&db_uri);
        match db {
            Ok(_) => assert!(true),
            Err(e) => panic!("Error connecting to db: {:?}", e),
            
        }
        assert!(db.is_ok());

    }

    #[test]
    fn test_should_new_panic_when_connection_string_is_empty() {
        let db_uri = String::from("");
        let db = DbClient::new(&db_uri);
        assert!(db.is_err());
        
    }

    
}




