mod port;

use port::Port;

use crate::port::VecInitOption;

fn main() -> std::io::Result<()> {
    let port = Port::begin(4050, VecInitOption::default())?;

    println!("Listening on port 4050");

    port.run()?;

    Ok(())
}
