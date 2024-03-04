use anyhow::Ok;
use qdrant_client::client::QdrantClient;
use qdrant_client::qdrant::{CreateCollection, Distance};
use qdrant_client::qdrant::{vectors_config::Config, VectorParams, VectorsConfig};


pub struct VectorDbClient {
    client: QdrantClient
}

impl VectorDbClient {

    pub fn new(host: &str, port: &str) -> Result<VectorDbClient, anyhow::Error> {
        let q_client = QdrantClient::from_url(&format!("http://{}:{}", host, port)).build()?;
        Ok(VectorDbClient{
            client: q_client
        })
    }
    
    pub async fn create_collection(&self, name:&str) -> Result<(), anyhow::Error> {
        
        self.client.create_collection(
            &CreateCollection{
                collection_name: name.to_string(),
                vectors_config: Some(VectorsConfig {
                    config: Some(Config::Params(VectorParams {
                        size: 384,
                        distance: Distance::Dot.into(),
                        ..Default::default()
                    })),
                }),
                ..Default::default()
            }

        ).await?; 
        Ok(())
    }


    pub async fn delete_collection(&self,name:&str) -> Result<(), anyhow::Error>{
        self.client.delete_collection(name).await?;
        Ok(())
    }

}



#[cfg(test)]
mod tests {

    use core::panic;

    use super::*;


    #[test]
    fn test_should_new_connect_return_db_connection() {
        let db = VectorDbClient::new("localhost", "6334");
        assert!(db.is_ok());
    }

    #[tokio::test]
    async fn test_should_create_collection_create_test_collection() {
        //Arrange
        let db = VectorDbClient::new("localhost", "6334").expect("failed to connect");

        //Act
        let res = db.create_collection("test").await;

        //Assert
        assert!(res.is_ok());

        //Clean up
        let delete_result = db.delete_collection("test").await;
        assert!(delete_result.is_ok());
    }



}