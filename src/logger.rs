extern crate chrono;

use self::chrono::{ DateTime, Local, Timelike, Datelike };

use std::fs::{ File, OpenOptions };
use std::io::prelude::*;

lazy_static!{ static ref LOGGER : Logger = Logger::new(); }

pub struct Logger {
    file_name:  String,
    file_handle: File,
}

fn new_file() -> String {
    let local : DateTime<Local> = Local::now();
    let date = local.date();

    if let Err(e) = std::fs::create_dir("logs") {
        eprintln!("Failed to create directory! {}", e);
    }

    let mut build_flag = "";

     if cfg!( feature = "debug" ) {
        build_flag = "Debug";
     } else if cfg!(release){
        build_flag = "Release";
    }

    String::from(format!("logs\\{}{}{}_{:02}{:02}{:02}_{}Log.txt",
    date.year().to_string(),
    date.month().to_string(),
    date.day().to_string(),
    local.hour().to_string(), 
    local.minute().to_string(), 
    local.second().to_string(),
    String::from(build_flag).to_string(),
    ).to_string())


}

fn open_file( f : &str ) -> File {
    OpenOptions::new()
    .append(true)
    .create(true)
    .open(f)
    .unwrap() 
}

impl Logger {
    pub fn new() -> Logger {
        let filename = new_file();
        Logger {
             file_name : filename.clone(), 
             file_handle : open_file(&filename) 
        }
    }

    pub fn write(&self, msg : &str) {
        if let Err(e) = writeln!(&self.file_handle, "{}", msg) {
            eprintln!("Failed to write to file: {}", e);
        }
    }
}

pub fn write(msg : &str) {
    LOGGER.write(msg);
}
