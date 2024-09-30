use std::net::SocketAddr;

use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use crate::config::InfoscreenConfig;
use crate::web_server::WebServer;

pub struct WebServerHost;

impl WebServerHost {
    pub async fn run(config: InfoscreenConfig) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let address: SocketAddr = config.listen_address.parse().unwrap();
        let listener = TcpListener::bind(address).await?;
        println!("Listening on http://{}", address);

        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);
            let static_file_directory = config.static_file_directory.clone();

            tokio::task::spawn(async move {
                let builder = http1::Builder::new();
                let web_server = WebServer::new(static_file_directory);
                if let Err(err) = builder.serve_connection(io, web_server).await {
                    println!("Failed to serve connection: {:?}", err);
                }
            });
        }
    }
}