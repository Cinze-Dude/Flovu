mod error;
mod port;
mod request;
mod response;
mod router;
mod validate;

use port::Port;

use crate::router::Router;

const PORT: u16 = 1600;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut port = Port::new(PORT);
    let mut router = Router::new();

    port.begin().await?;

    println!("Listening on port {}", PORT);

    port.run(&router).await?;

    Ok(())
}
