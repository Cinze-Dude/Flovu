mod error;
mod port;
mod request;
mod validate;

use port::Port;

fn main() -> std::io::Result<()> {
    let mut port = Port::new(4050);

    port.begin()?;

    println!("Listening on port 4050");

    port.run()?;

    Ok(())
}
