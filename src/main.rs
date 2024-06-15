#![feature(exclusive_range_pattern)]
use parse_duration::parse;
use rand::Rng;
use std::fs::{self, File};
use std::io::prelude::*;

#[derive(Debug)]
struct Config {
    period: &'static str,
    qnt: i32,
}

fn main() {
    let configs: Vec<Config> = vec![
        {
            Config {
                period: "1d",
                qnt: 7,
            }
        },
        {
            Config {
                period: "1w",
                qnt: 8,
            }
        },
        {
            Config {
                period: "2w",
                qnt: 8,
            }
        },
        {
            Config {
                period: "1m",
                qnt: 12,
            }
        },
        {
            Config {
                period: "1y",
                qnt: 2,
            }
        },
    ];

    for config in configs {
        let parsed_period = parse(config.period);

        match parsed_period {
            Ok(duration) => {
                println!(
                    "Period: {}, Qnt: {}, Parsed duration: {}",
                    config.period,
                    config.qnt,
                    duration.as_secs()
                );
            }
            Err(e) => {
                println!("Error: {:?}", e);
                println!("Period: {}, Qnt: {}", config.period, config.qnt);
            }
        }
    }

    generate_files();
}

fn generate_files() -> std::io::Result<()> {
    let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let test_data_path = format!("{path}/test-data");
    fs::create_dir_all(test_data_path)?;

    let files_qnt = 10;

    for i in 0..files_qnt {
        let day_num: i32 = rand::thread_rng().gen_range(0..30);
        let mut day_str = day_num.to_string();
        if (day_num < 10) {
            day_str = format!("0{}", day_str);
        }

        let mut month = rand::thread_rng().gen_range(1..12);
        let mut month_str = month.to_string();
        if (month < 10) {
            month_str = format!("0{}", month);
        }
        let year = rand::thread_rng().gen_range(2000..2024);

        let file_name = format!("{path}/test-data/{day_str}.{month_str}.{year}_{i}.tar");
        // let mut file = File::create(format!("{path}/test-data/12.09_{i}.tar"))?;
        let mut file = File::create(file_name)?;
        file.write_all(b"Hello, world!")?;
    }

    Ok(())
}
