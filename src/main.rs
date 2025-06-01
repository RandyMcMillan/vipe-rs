use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;

use rand::RngCore;

// For checking if stdin is a TTY (Terminal)
use atty::is as is_atty;

// Unix-specific imports for handling raw file descriptors
#[cfg(unix)]
use std::os::unix::io::AsRawFd;

/// The version of the vipe utility.
const VERSION: &str = "0.1.1";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Collect command-line arguments.
    let args: Vec<String> = env::args().collect();

    // Handle command-line arguments: -h for help, -V for version.
    if args.len() > 1 {
        match args[1].as_str() {
            "-h" => {
                // Print usage information and exit successfully.
                println!("usage: vipe [-hV]");
                return Ok(());
            }
            "-V" => {
                // Print the version and exit successfully.
                println!("{}", VERSION);
                return Ok(());
            }
            _ => {
                // Handle unknown options, print usage, and exit with an error.
                eprintln!("unknown option: \"{}\"", args[1]);
                eprintln!("usage: vipe [-hV]");
                std::process::exit(1);
            }
        }
    }

    // Determine the editor to use.
    // It first tries to get the value of the EDITOR environment variable.
    // If EDITOR is not set, it defaults to "notepad.exe" on Windows or "vi" on other systems (like Unix).
    let editor = env::var("EDITOR").unwrap_or_else(|_| {
        if cfg!(windows) {
            "notepad.exe".to_string()
        } else {
            "vi".to_string()
        }
    });

    // Create a temporary file.
    // `NamedTempFile` ensures a unique file name and automatically handles deletion
    // when the `temp_file` variable goes out of scope.
    let mut temp_file = NamedTempFile::new()?;
    // Get the path of the temporary file. We need this path to pass to the editor.
    let temp_path = temp_file.path().to_owned();

    // Read from standard input (stdin) if it's not a TTY.
    // This means if data is being piped into `vipe` (e.g., `echo "hello" | vipe`),
    // that data will be written to the temporary file.
    if !is_atty(atty::Stream::Stdin) {
        let mut stdin = io::stdin();
        io::copy(&mut stdin, &mut temp_file)?;
    }

    // Flush any buffered content to the temporary file.
    // This ensures that all data written from stdin is persisted to disk
    // before the editor attempts to open the file.
    temp_file.flush()?;

    // Spawn the editor process.
    // The behavior for connecting the editor's stdio differs based on the operating system.
    let status = if cfg!(unix) {
        Command::new(&editor)
            .arg(&temp_path)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()? // Execute the command and wait for it to complete.
    } else {
        // On non-Unix systems (e.g., Windows), we cannot use `/dev/tty` directly.
        // Instead, we inherit the standard I/O streams from the `vipe` process.
        // This typically means the editor will interact with the console.
        Command::new(&editor)
            .arg(&temp_path)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()? // Execute the command and wait for it to complete.
    };

    // Check the exit status of the editor.
    if !status.success() {
        // If the editor exited with a non-zero status (indicating an error or cancellation),
        // print a message and exit `vipe` with the same status code.
        eprintln!("Editor exited with non-zero status: {:?}", status.code());
        std::process::exit(status.code().unwrap_or(1));
    }

    // Read the content of the temporary file after the editor has finished.
    let content = fs::read_to_string(&temp_path)?;

    // Write the modified content to standard output (stdout).
    print!("{}", content);

    // The `temp_file` variable goes out of scope here, triggering the automatic
    // deletion of the temporary file.

    Ok(())
}
