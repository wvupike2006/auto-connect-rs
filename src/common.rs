extern crate simplelog;

use log::*;
use simplelog::*;
use std::fs::File;


pub fn init_log()
{
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug, Config::default(),TerminalMode::Mixed,
                            ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(),
                             File::create("automation-report.log").unwrap())
        ]
    ).unwrap();
}

pub fn log_to_file(output_txt: &str) {
    init_log();
    info!("{}", output_txt);
}