use std::process::Command;
use clap::Parser;


#[derive(Parser, Clone)]
pub struct Settings {
    #[arg(long)]
    pub ip: String,

    #[arg(long)]
    pub port: Option<u32>,

    #[arg(long)]
    pub username: String,
}

fn main() {
    let settings = Settings::parse();

    let pid = ssh(&settings).unwrap();

    println!("ssh complete. Pid = {pid}");
}

fn ssh(Settings { ip, port, username }: &Settings) -> Result<u32, String> {
    let port = if port.is_some() {
        format!(":{}", port.unwrap())
    } else {
        String::new()
    };

    let mut child = Command::new("ssh")
        .arg("-Y")
        .arg(format!("{}@{}{}", username, ip, port))
        .spawn().expect("Failed to start ssh tunnel");

    Ok(child.id())
}


