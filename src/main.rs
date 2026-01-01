mod dbus;
mod duration;
mod pacman;
mod random;
mod sentences;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let welcome_message = sentences::welcome_message();
    println!("Welcome back. {welcome_message}\n");

    match pacman::time_since_last_pacman_update() {
        Err(e) => eprintln!("checking system updates: {}", e),
        Ok(d) => println!("Last system update: {} ago.", duration::HumanDuration(d)),
    }

    match dbus::retrieve_boot_time() {
        Err(e) => eprintln!("retrieving boot time: {}", e),
        Ok(d) => println!("boot time: {}.", d),
    }

    Ok(())
}
