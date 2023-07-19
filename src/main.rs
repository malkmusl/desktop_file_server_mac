use std::io::prelude::*;
use std::net::TcpStream;
use std::fs::{read_dir, File};
use std::path::Path;
use serde::{Deserialize, Serialize};
use config::{Config, File as ConfigFile};
use std::process::Command;

#[derive(Debug, Deserialize, Serialize)]
struct ServerConfig {
    ip: String,
    port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            ip: String::from("127.0.0.1"),
            port: 1234,
        }
    }
}

fn main() {
    let config_path = "config.toml";
    create_config_if_not_exist(config_path);

    let server_config = load_server_config(config_path);

    let linux_server_address = format!("{}:{}", server_config.ip, server_config.port);

    println!("Connecting to Linux server...");
    if let Ok(mut stream) = TcpStream::connect(linux_server_address) {
        println!("Connected to Linux server!");

        // Get a list of runnable programs on macOS or a demo app for testing on Linux
        let runnable_programs = get_runnable_programs().unwrap_or_else(|| {
            vec![
                (
                    String::from("Finder"),  // Demo app name
                    String::from("/usr/bin/finder"),  // Demo app path
                )
            ]
        });

        // Send the runnable programs to the Linux server
        send_runnable_programs(&mut stream, runnable_programs);

        println!("Runnable programs sent successfully!");
    } else {
        eprintln!("Failed to connect to Linux server");
    }
}

fn create_config_if_not_exist(config_path: &str) {
    let path = Path::new(config_path);
    if !path.exists() {
        let server_config = ServerConfig::default();
        let toml = toml::to_string(&server_config).expect("Failed to serialize default config");

        let mut file = File::create(path).expect("Failed to create config file");
        file.write_all(toml.as_bytes()).expect("Failed to write config file");
    }
}

fn load_server_config(config_path: &str) -> ServerConfig {
    let mut config = Config::default();
    config
        .merge(ConfigFile::new(config_path, config::FileFormat::Toml))
        .expect("Failed to merge config file");

    config.try_into().expect("Failed to deserialize config")
}

fn get_runnable_programs() -> Option<Vec<(String, String)>> {
    // Directory to scan for runnable programs (change as needed)
    let directory = "/Applications";

    // Read the directory contents and filter out executable files
    if let Ok(entries) = read_dir(directory) {
        let runnable_programs: Vec<(String, String)> = entries
            .filter_map(|entry| {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() && is_executable(&path) {
                        if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
                            return Some((
                                file_name.to_owned(),
                                path.to_string_lossy().to_owned().into_owned(),
                            ));
                        }
                    }
                }
                None
            })
            .collect();

        if !runnable_programs.is_empty() {
            return Some(runnable_programs);
        }
    }

    None
}

fn is_executable(path: &Path) -> bool {
    #[cfg(target_os = "macos")]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = path.metadata() {
            let permissions = metadata.permissions();
            return permissions.mode() & 0o111 != 0;
        }
    }
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = path.metadata() {
            let permissions = metadata.permissions();
            return permissions.mode() & 0o111 != 0 && !path.is_dir();
        }
    }
    false
}

fn get_client_info() -> Result<(String, String), Box<dyn std::error::Error>> {
    let client_user = get_current_user()?;
    let client_ip = get_current_ip()?;

    Ok((client_user, client_ip))
}

fn get_current_user() -> Result<String, std::io::Error> {
    let output = Command::new("whoami")
        .output()
        .expect("Failed to execute command 'whoami'");
    let client_user = String::from_utf8_lossy(&output.stdout).trim().to_string();

    Ok(client_user)
}

#[cfg(target_os = "linux")]
fn get_current_ip() -> Result<String, std::io::Error> {
    let output = Command::new("hostname")
        .output()
        .expect("Failed to execute command 'hostname'");
    let client_ip = String::from_utf8_lossy(&output.stdout).trim().to_string();

    Ok(client_ip)
}

#[cfg(target_os = "macos")]
fn get_current_ip() -> Result<String, Box<dyn std::error::Error>> {
    let client_ip = Sysctl::new("net.iphase2.inet.ip")
        .value_string()
        .expect("Failed to retrieve IP address from sysctl");

    Ok(client_ip)
}

fn send_runnable_programs(stream: &mut TcpStream, runnable_programs: Vec<(String, String)>) {
    if let Ok((client_user, client_ip)) = get_client_info() {
        println!("Client IP: {}", client_ip);
        println!("Client User: {}", client_user);

        for (app_name, app_path) in runnable_programs {
            // Create the desktop file contents
            let desktop_file_contents = format!(
                "Name={}\nExec=ssh -X {}@{} {}\n",
                app_name, client_user, client_ip, app_path
            );

            // Send the desktop file contents to the Linux server
            if let Err(err) = stream.write_all(desktop_file_contents.as_bytes()) {
                eprintln!("Failed to send application information: {}", err);
            }
        }
    } else {
        eprintln!("Failed to get client information");
    }
}

/// .
// Function for testing on a Linux host
#[warn(dead_code.)]
fn test_on_linux() {
    let config_path = "config.toml";
    create_config_if_not_exist(config_path);

    let server_config = load_server_config(config_path);

    let linux_server_address = format!("{}:{}", server_config.ip, server_config.port);

    println!("Connecting to Linux server...");
    if let Ok(mut stream) = TcpStream::connect(linux_server_address) {
        println!("Connected to Linux server!");

        // Create a demo app for testing
        let demo_app = (
            String::from("Finder"),  // Demo app name
            String::from("/usr/bin/finder"),  // Demo app path
        );

        // Send the demo app information to the Linux server
        send_runnable_programs(&mut stream, vec![demo_app]);

        println!("Demo app information sent successfully!");
    } else {
        eprintln!("Failed to connect to Linux server");
    }
}
