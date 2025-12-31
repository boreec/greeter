mod duration;
mod pacman;
mod sentences;

fn main() {
    let welcome_message = sentences::welcome_message();
    println!("Welcome back. {welcome_message}\n");

    match pacman::time_since_last_pacman_update() {
        Err(e) => eprintln!("checking system updates: {}", e),
        Ok(d) => println!("Last system update: {} ago.", duration::HumanDuration(d)),
    }
}
