mod ascii;

use whoami::{username,hostname,distro};
use colored::Colorize;

fn main() {
    let username = username();
    let hostname = hostname();
    let os_name= distro();
    let kernel = nixinfo::kernel().unwrap_or_else(|_| "Unavailable".to_string());
    let uptime = nixinfo::uptime().unwrap_or_else(|_| "Unavailable".to_string());
     let mem = nixinfo::memory().unwrap_or_else(|_| "Unavailable".to_string());

    let ascii_art = ascii::AS;
    println!(" ");
    println!("{}@{}",username.red(),hostname.blue());
    println!(" {}", ascii_art.green());
    println!("   ");
    println!("OS: {}", os_name.purple());
    println!("Kernel: {}", kernel.cyan());
    println!("Uptime: {}", uptime.yellow());
    println!("Memory : {}", mem.white());
}
