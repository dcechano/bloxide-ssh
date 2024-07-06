use clap::Parser;
use std::process::Command;

#[derive(Parser, Clone)]
pub struct Settings {
    #[arg(long)]
    pub key_path: String,

    #[arg(long)]
    pub ssh_port: String,

    #[arg(long)]
    pub local_port: u32,

    #[arg(long)]
    pub remote_host: String,

    #[arg(long)]
    pub remote_port: u32,

    #[arg(long)]
    pub username: String,

    #[arg(long)]
    pub server: String,
}

fn main() {
    let settings = Settings::parse();

    let pid = ssh(&settings);

    println!("ssh complete. Pid = {pid}");
}

fn ssh(
    Settings {
        key_path,
        ssh_port,
        local_port,
        remote_host,
        remote_port,
        username,
        server,
    }: &Settings,
) -> u32 {
    let child = Command::new("ssh")
        .arg("-i")
        .arg(key_path)
        .arg("-p")
        .arg(ssh_port)
        .arg("-L")
        .arg(format!("{local_port}:{remote_host}:{remote_port}"))
        .arg(format!("{}@{}", username, server))
        .spawn()
        .expect("Failed to start ssh tunnel");

    child.id()
}
