use clap::{Args, Parser, Subcommand};
use directories::ProjectDirs;
use kbar::Bar;
use serde::Serialize;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::path::PathBuf;

use std::thread::sleep;
use std::time::Duration;
use toml;

use toml::Value;

use std::io::Write;

// Top level struct to hold the TOML data.
#[derive(Serialize)]
struct Config {
    website_list_path: String,
}

#[derive(Parser)]
#[command(author, version)]
#[command(
    about = "stringer - a simple CLI to transform and inspect strings",
    long_about = "stringer is a super fancy CLI (kidding)

One can use stringer to modify or inspect strings straight from the terminal"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    /// timer for blocker
    #[arg(long = "time")]
    time: Option<String>,
    /// task name
    #[arg(long = "task")]
    task: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Reverses a string
    Setup(Setup),
}

#[derive(Args)]
struct Setup {
    /// task name
    #[arg(long = "list")]
    list: String,
}

// fn is_file_exist(path: &str) -> bool {
//     return std::path::Path::new(path).is_file();
// }

const LINUX_HOSTS_PATH: &str = "/etc/hosts";
const MACOS_HOSTS_PATH: &str = "/etc/hosts";
const WINDOWS_HOSTS_PATH: &str = "C:\\Windows\\System32\\drivers\\etc\\hosts";

#[inline]
const fn get_hosts_path() -> &'static str {
    if cfg!(target_os = "linux") {
        LINUX_HOSTS_PATH
    } else if cfg!(target_os = "windows") {
        WINDOWS_HOSTS_PATH
    } else if cfg!(target_os = "macos") {
        MACOS_HOSTS_PATH
    } else {
        panic!("Unsupported operating system");
    }
}

fn block_websites(time_to_sleep: u64, task: &String) -> io::Result<()> {
    let hosts_path: &str = get_hosts_path();
    let mut backup_path: PathBuf = PathBuf::new();
    let mut toml_config_path: PathBuf = PathBuf::new();

    if let Some(proj_dirs) = ProjectDirs::from("com", "chetanxpro", "focusguard") {
        let config_dir = proj_dirs.config_dir();

        if !config_dir.exists() {
            fs::create_dir_all(config_dir).expect("Error while creating config directory");

            backup_path = config_dir.join("hosts_backup");

            fs::File::create(&backup_path).expect("Error while creating hosts backup file");
        }

        backup_path = config_dir.join("hosts_backup");
        toml_config_path = config_dir.join("config.toml");

        dbg!(config_dir);
    }

    let website_file_option = get_websites_path(toml_config_path);

    let websites_file_path = website_file_option.as_deref().unwrap_or("default");

    let mut hosts_content: String =
        fs::read_to_string(hosts_path).expect("Error while reading host file content");
    let websites_list_content: String =
        fs::read_to_string(websites_file_path).expect("Error while reading website contetn");

    dbg!(&backup_path);
    let mut backup_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&backup_path)
        .expect("Not able to open backup file");

    backup_file
        .write_all(hosts_content.as_bytes())
        .expect("Error while writing to backup file");

    // website_list_path.clone();

    let website_list: std::str::Split<'_, &str> = websites_list_content.split("\n");

    hosts_content.push_str(&format!("\n# ========== Temp Hosts ========="));
    for website in website_list {
        // println!("Website: {}",website);

        if !hosts_content.contains(website) {
            hosts_content.push_str(&format!("\n127.0.0.1\t{}", website));
        }
    }
    hosts_content.push_str(&format!("\n# ========== Temp Hosts ========="));
    println!("Content:\n {}", hosts_content);

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(hosts_path)?;
    file.write_all(hosts_content.as_bytes())?;

    println!("Websites blocked");

    let mut bar = Bar::new();
    let format_task = format!("Timer for Task: {}", task);
    bar.set_job_label(&format_task);

    for i in 0..101 {
        sleep(Duration::from_millis(time_to_sleep));
        bar.reach_percent(i);
    }

    sleep(Duration::from_millis(time_to_sleep));

    println!("Websites unblocked");
    let backup_file_content: String =
        fs::read_to_string(backup_path).expect("Error while reading backup file");

    let mut backup_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(hosts_path)
        .unwrap();

    backup_file
        .write_all(backup_file_content.as_bytes())
        .unwrap();

    Ok(())
}

fn get_websites_path(config_path: PathBuf) -> Option<String> {
    let config_content = fs::read_to_string(config_path).unwrap();
    let value: Value = toml::from_str(&config_content).unwrap();

    let website_list_path = value
        .get("website_list_path")
        .and_then(Value::as_str)
        .map(String::from);
    // .unwrap();

    website_list_path
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Setup(setup)) => {
            // println!("List path: {}", setup.list);
            if let Some(proj_dirs) = ProjectDirs::from("com", "chetanxpro", "focusguard") {
                let config_dir = proj_dirs.config_dir();

                if !config_dir.exists() {
                    fs::create_dir_all(config_dir).expect("Error while creating config directory");

                    fs::File::create(config_dir.join("hosts_backup"))
                        .expect("Error while creating hosts backup file");
                }

                let config = Config {
                    website_list_path: setup.list.clone(),
                };

                let toml = toml::to_string(&config).unwrap();

                fs::File::create(config_dir.join("config.toml")).unwrap();

                let mut config_file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(config_dir.join("config.toml"))
                    .unwrap();

                config_file.write_all(toml.as_bytes()).unwrap();

                println!("Website file path saved in config ✅")
            }
        }
        None => {
            if let (Some(time), Some(task)) = (cli.time, cli.task) {
                // println!("lol")
                println!("Time: {:#?} , Task: {}", time, task);

                let mut time_in_milliseconds: u64;
                // let time: String = cli.time;
                // let task: String = args.task;

                if time.contains("m") {
                    time_in_milliseconds = time.replace("m", "").parse().unwrap();
                    time_in_milliseconds = (time_in_milliseconds * 60 * 1000) / 100;
                } else if time.contains("s") {
                    time_in_milliseconds = time.replace("s", "").parse().unwrap();
                    time_in_milliseconds = (time_in_milliseconds * 1000) / 100;
                } else if time.contains("h") {
                    time_in_milliseconds = time.replace("h", "").parse().unwrap();
                    time_in_milliseconds = (time_in_milliseconds * 60 * 60 * 1000) / 100;
                } else {
                    time_in_milliseconds = 0
                }

                block_websites(time_in_milliseconds, &task).expect("Error")
            } else {
                println!("No command provided");
            }
        }
    }
}
