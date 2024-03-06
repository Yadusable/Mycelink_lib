pub mod api;
mod crypto;
pub mod db;
mod fcp_tools;
mod model;

#[cfg(test)]
mod test {
    use mycelink_lib_fcp::fcp_connector::FCPConnector;
    use std::sync::Arc;
    use tokio::net::TcpStream;

    pub async fn create_test_fcp_connector(test_name: &str) -> Arc<FCPConnector> {
        let stream = TcpStream::connect("localhost:9481").await.unwrap();
        let connector = FCPConnector::new(stream, format!("MycelinkTest {test_name}").as_str())
            .await
            .unwrap();
        let connector = Arc::new(connector);
        let listen_connector = connector.clone();

        let _handle = tokio::spawn(async move { listen_connector.listen().await });

        connector
    }
}
