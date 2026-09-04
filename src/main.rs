mod error;
mod handler;
mod port;
mod request;
mod response;
mod validate;

use port::Port;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    const PORT: u16 = 1600;

    let mut port = Port::new(PORT);

    port.begin().await?;

    println!("Listening on port {}", PORT);

    port.run().await?;

    Ok(())
}
