use std::pin::Pin;
use std::str::from_utf8;
use bytes::Bytes;
use futures_util::Future;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::service::Service;
use hyper::Response;
use hyper::Request;
use hyper::StatusCode;
use tokio::fs::read;

use crate::data_manager::DATA_MANAGER;

static NOTFOUND: &[u8] = b"Not Found";

pub struct WebServer {
    static_file_directory: String
}

impl WebServer {
    pub fn new(static_file_directory: String) -> WebServer {
        WebServer {
            static_file_directory
        }
    }

    fn get_mime_type(filename: &str) -> &str {
        match filename.split('.').last().unwrap_or("") {
            "html" => "text/html",
            "css" => "text/css",
            "js" => "application/javascript",
            "jpg" | "jpeg" => "image/jpeg",
            "png" => "image/png",
            "gif" => "image/gif",
            "svg" => "image/svg+xml",
            "tiff" => "image/tiff",
            "xml" => "application/xml",
            "woff" => "font/woff",
            "woff2" => "font/woff2",
            "ttf" => "font/ttf",
            "otf" => "font/otf",
            _ => "application/octet-stream",
        }
    }

    async fn simple_file_send(filename: &str) -> hyper::Result<Response<Full<Bytes>>> {
        let file_content = read(filename).await;
        if file_content.is_err() {
            eprintln!("Unable to open file {}", filename);
            let not_found = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("Content-Type", "text/plain")
                .header("Connection", "close")
                .body(Full::new(NOTFOUND.into()))
                .unwrap();
            return Ok(not_found)
        }

        let bytes: Bytes = file_content.unwrap().into();

        if filename.ends_with("/index.html") {
            let json = DATA_MANAGER.get_data().await;
            let content = from_utf8(&bytes).unwrap().replace("$DATA", &json);

            let response = Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", Self::get_mime_type(filename))
                .header("Connection", "close")
                .body(Full::new(content.into()))
                .unwrap();

            return Ok(response);
        }

        let response = Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", Self::get_mime_type(filename))
            .header("Connection", "close")
            .header("Cache-Control", "public, max-age=86400")
            .body(Full::new(bytes))
            .unwrap();

        Ok(response)
    }
}

impl Service<Request<Incoming>> for WebServer {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let path = req.uri().path();

        println!("Request to path: {}", path);

        let directory = self.static_file_directory.clone();
        let filename = format!("{}{}", directory, path);

        Box::pin(async move { Self::simple_file_send(filename.as_str()).await })
    }
}
