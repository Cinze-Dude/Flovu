use flovu::{port::Port, rfuncs::file::FileRfuncs, router::Router};

const PORT: u16 = 1600;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut port = Port::new(PORT);
    let mut router = Router::new();

    router.route_html("/john", "john.html");
    router.route_html("/mark", "mark.html");

    port.begin().await?;

    println!("Listening on port {}", PORT);

    port.run(router.arc()).await?;

    Ok(())
}
