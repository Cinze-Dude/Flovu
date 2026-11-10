mod error;
mod port;
mod request;
mod response;
mod rfuncs;
mod router;
mod validate;

use port::Port;

use crate::{rfuncs::basic::BasicRfuncs, router::Router};

const PORT: u16 = 1600;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut port = Port::new(PORT);
    let mut router = Router::new();

    router.route_get("/plswork", "text/plain", "Hello, World!");

    port.begin().await?;

    println!("Listening on port {}", PORT);

    port.run(router.arc()).await?;

    Ok(())
}
