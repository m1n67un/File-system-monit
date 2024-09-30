use file_system_monit::handler::file_handler::watch;
use notify::{
    RecommendedWatcher, 
    RecursiveMode, 
    Watcher, 
    Config
};
use std::path::Path;
use syslog::{
    Facility, 
    Formatter3164, 
    BasicLogger
};
use log::LevelFilter;

fn main() {
    let path = std::env::args().nth(1).expect("Argument 1 needs to be a path");

    // syslog 설정
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "fs-watcher".into(),
        pid: 0,
    };
    let logger = syslog::unix(formatter).expect("could not connect to syslog");
    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
        .map(|()| log::set_max_level(LevelFilter::Warn))
        .expect("could not register logger");

    println!("Watching {}", path);
    if let Err(e) = watch(path) {
        println!("Error: {:?}", e);
    }
}