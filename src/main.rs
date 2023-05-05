mod ascii;

use whoami::{username,hostname,distro};


fn main() {
    let username = username();
    let hostname = hostname();
    let os_name= distro();
    let kernel = nixinfo::kernel().unwrap_or_else(|_| "Unavailable".to_string());
    let uptime = nixinfo::uptime().unwrap_or_else(|_| "Unavailable".to_string());
     let mem = nixinfo::memory().unwrap_or_else(|_| "Unavailable".to_string());

    let ascii_art = ascii::AS;
    println!("{}@{}",username,hostname);
    println!(" {}", ascii_art);
    println!("   ");
    println!("OS: {}", os_name);
    println!("Kernel: {}", kernel);
    println!("Uptime: {}", uptime);
    println!("Memory : {}", mem);
}
