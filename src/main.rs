use hyper::server::conn::Http;
use hyper::service::service_fn;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::handler::cars_handler;

mod model;
mod services;
mod handler;

// #[derive(Serialize, Deserialize)]
// struct Car {
//     id: String,
//     brand: String,
//     model: String,
//     year: u16,
// }


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = Http::new()
                .serve_connection(stream, service_fn(cars_handler))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
