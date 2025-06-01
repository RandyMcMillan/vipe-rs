use std::env;
use std::fs::{self};
use std::io::{self, Write};
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;

// For checking if stdin is a TTY (Terminal)
use atty::is as is_atty;

// Unix-specific imports for handling raw file descriptors

/// The version of the vipe utility.
const VERSION: &str = "0.0.1";

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "-a" => {
                // Print about information and exit successfully.
                println!("about:");
                println!("       https://github.com/madx/moreutils.git");
                println!("       https://raw.githubusercontent.com/madx/moreutils/refs/heads/master/vipe");
                println!("       https://github.com/juliangruber/vipe.git");
                println!("       https://raw.githubusercontent.com/juliangruber/vipe/refs/heads/master/vipe.sh");
                return Ok(());
            }
            "--about" => {
                // Print about information and exit successfully.
                println!("about:");
                println!("       https://github.com/madx/moreutils.git");
                println!("       https://raw.githubusercontent.com/madx/moreutils/refs/heads/master/vipe");
                println!("       https://github.com/juliangruber/vipe.git");
                println!("       https://raw.githubusercontent.com/juliangruber/vipe/refs/heads/master/vipe.sh");
                return Ok(());
            }
            "-h" => {
                // Print usage information and exit successfully.
                println!("usage: vipe [-ahV]");
                return Ok(());
            }
            "--help" => {
                // Print usage information and exit successfully.
                println!("usage: vipe [-ahV]");
                return Ok(());
            }
            "-V" => {
                // Print the version and exit successfully.
                println!("{}", VERSION);
                return Ok(());
            }
            "--version" => {
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

    let editor = env::var("EDITOR").unwrap_or_else(|_| {
        if cfg!(windows) {
            "notepad.exe".to_string()
        } else {
            "vi".to_string()
        }
    });

    let mut temp_file = NamedTempFile::new()?;
    let temp_path = temp_file.path().to_owned();
    if !is_atty(atty::Stream::Stdin) {
        let mut stdin = io::stdin();
        io::copy(&mut stdin, &mut temp_file)?;
    }

    temp_file.flush()?;

    let status = Command::new(&editor)
        .arg(&temp_path)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if !status.success() {
        eprintln!("Editor exited with non-zero status: {:?}", status.code());
        std::process::exit(status.code().unwrap_or(1));
    }

    let content = fs::read_to_string(&temp_path)?;

    print!("{}", content);

    Ok(())
}
