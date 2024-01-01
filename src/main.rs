use std::fs;
use std::fs::OpenOptions;
use std::io;
// use clap::Parser;

use std::io::Write;

struct CLI {
   
}



fn is_file_exist(path:&str) -> bool {
    return std::path::Path::new(path).is_file();
}

fn block_websites(website_list_path:&str) -> io::Result<()> {
           
    
    let hosts_path = "/etc/hosts";
    let backup_path:&str = "./original_host_backup";


    let file_exist = is_file_exist(backup_path);


    if file_exist != true {
        fs::File::create(backup_path).expect("Error while creating hosts backup file");
    }




    
   
    

    let mut hosts_content:String = fs::read_to_string(hosts_path).expect("Error while reading host file coontent");
    let websites_list_content:String = fs::read_to_string(website_list_path).expect("Error while reading website contetn");



    let mut backup_file = OpenOptions::new().write(true).truncate(true).open(backup_path).expect("Not able to open backup file");


    backup_file.write_all(hosts_content.as_bytes())?;

    

    // website_list_path.clone();
     
    let website_list: std::str::Split<'_, &str> = websites_list_content.split("\n");
    
    for website in website_list {
        // println!("Website: {}",website);
        if !hosts_content.contains(website){
            hosts_content.push_str(&format!("\n127.0.0.1\t{}", website));
           

        }
    }
    println!("Content:\n {}",hosts_content);

    
    // let mut file = OpenOptions::new().write(true).truncate(true).open(hosts_path)?;
    // file.write_all(hosts_content.as_bytes())?;
  
    Ok(())
}

fn main() {
    let website_list_path = "./websites.txt";
   block_websites(website_list_path).expect("Error")
}

// use kbar::Bar;
// use std::thread::sleep;
// use std::time::Duration;

// fn main() {
//     let mut bar = Bar::new();
//     bar.set_job_label("Timer");

//     for i in 0..101 {
//         sleep(Duration::from_millis(36000));
//         bar.reach_percent(i);
//     }
// }


// sec*1000/100