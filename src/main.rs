use std::io::prelude::*;
use std::net::TcpStream;
use std::fs::{read_to_string, read_dir};
use std::path::Path;

fn main() {
    let linux_server_address = "127.0.0.1:1234";

    println!("Connecting to Linux server...");
    if let Ok(stream) = TcpStream::connect(linux_server_address) {
        println!("Connected to Linux server!");

        // Get a list of runnable programs on macOS or a demo app for testing on Linux
        let runnable_programs = get_runnable_programs().unwrap_or_else(|| {
            vec![
                (
                    String::from("Finder"),  // Demo app name
                    String::from("/usr/bin/finder")  // Demo app path
                )
            ]
        });

        // Send the runnable programs to the Linux server
        send_runnable_programs(stream, runnable_programs);

        println!("Runnable programs sent successfully!");
    } else {
        eprintln!("Failed to connect to Linux server");
    }
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
                                path.to_string_lossy().to_owned().into_owned()
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

fn send_runnable_programs(mut stream: TcpStream, runnable_programs: Vec<(String, String)>) {
    for (app_name, app_path) in runnable_programs {
        // Create the desktop file contents
        let desktop_file_contents = format!(
            "Name={}\nExec={}\n",
            app_name, app_path
        );

        // Send the desktop file contents to the Linux server
        if let Err(err) = stream.write_all(desktop_file_contents.as_bytes()) {
            eprintln!("Failed to send application information: {}", err);
        }
    }
}

// Function for testing on a Linux host
fn test_on_linux() {
    let linux_server_address = "127.0.0.1:1234";

    println!("Connecting to Linux server...");
    if let Ok(stream) = TcpStream::connect(linux_server_address) {
        println!("Connected to Linux server!");

        // Create a demo app for testing
        let demo_app = (
            String::from("Finder"),  // Demo app name
            String::from("/usr/bin/finder")  // Demo app path
        );

        // Send the demo app information to the Linux server
        send_runnable_programs(stream, vec![demo_app]);

        println!("Demo app information sent successfully!");
    } else {
        eprintln!("Failed to connect to Linux server");
    }
}
