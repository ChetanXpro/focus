use argh::{from_env, FromArgs};
use directories::ProjectDirs;
use kbar::Bar;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::time::Duration;
use std::{env::args, thread::sleep};

use std::io::Write;

#[derive(FromArgs)]
/// Reach new heights.
struct GoUp {
    /// timer for blocker
    #[argh(option, short = 't')]
    time: String,

    /// task name
    #[argh(option)]
    task: String,
}

fn is_file_exist(path: &str) -> bool {
    return std::path::Path::new(path).is_file();
}

fn block_websites(website_list_path: &str, time_to_sleep: u64, task: &String) -> io::Result<()> {
    let hosts_path = "/etc/hosts";
    let backup_path: &str = "./original_host_backup";

    let file_exist = is_file_exist(backup_path);

    if file_exist != true {
        fs::File::create(backup_path).expect("Error while creating hosts backup file");
    }

    let mut hosts_content: String =
        fs::read_to_string(hosts_path).expect("Error while reading host file coontent");
    let websites_list_content: String =
        fs::read_to_string(website_list_path).expect("Error while reading website contetn");

    let mut backup_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(backup_path)
        .expect("Not able to open backup file");

    backup_file
        .write_all(hosts_content.as_bytes())
        .expect("Error while writing to backup file");

    // website_list_path.clone();

    let website_list: std::str::Split<'_, &str> = websites_list_content.split("\n");

    for website in website_list {
        // println!("Website: {}",website);
        if !hosts_content.contains(website) {
            hosts_content.push_str(&format!("\n127.0.0.1\t{}", website));
        }
    }
    println!("Content:\n {}", hosts_content);

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(hosts_path)?;
    file.write_all(hosts_content.as_bytes())?;

    println!("Websites blocked");

    let mut bar = Bar::new();
    bar.set_job_label(&task);

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

fn main() {
    let args: GoUp = argh::from_env();
    println!("Time: {:#?} , Task: {}", args.time, args.task);

    let mut time_in_milliseconds: u64;
    let time: String = args.time;
    let task: String = args.task;

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

    // for _ in 0..args.time {
    //     println!("Hello {}!", args.time)
    // }
    let website_list_path = "./websites.txt";
    block_websites(website_list_path, time_in_milliseconds, &task).expect("Error")
}
