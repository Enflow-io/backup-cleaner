#![feature(exclusive_range_pattern)]
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use parse_duration::parse;
use rand::Rng;
use std::fs::{self, DirEntry, File};
use std::io::prelude::*;
use std::time::SystemTime;

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
                period: "1M",
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

    for config in &configs {
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

    let list_to_keep: Vec<String> = Vec::new();

    let config = configs.get(0).unwrap();
    check_period(config);

    // generate_files();
}

fn is_date_in_period(
    day_from: DateTime<Utc>,
    period_secs: i64,
    date_to_check: DateTime<Utc>,
) -> bool {
    let date_to_check_in_seconds = date_to_check.timestamp();
    let day_from_in_seconds = day_from.timestamp();

    return date_to_check_in_seconds >= day_from_in_seconds
        && date_to_check_in_seconds <= (day_from_in_seconds + period_secs);
}

fn generate_files() -> std::io::Result<()> {
    let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let test_data_path = format!("{path}/test-data");
    fs::create_dir_all(test_data_path)?;

    let files_qnt = 10;

    for i in 0..files_qnt {
        let day_num: i32 = rand::thread_rng().gen_range(1..30);
        let mut day_str = day_num.to_string();
        if day_num < 10 {
            day_str = format!("0{}", day_str);
        }

        let month = rand::thread_rng().gen_range(1..12);
        let mut month_str = month.to_string();
        if (month < 10) {
            month_str = format!("0{}", month);
        }
        let year = rand::thread_rng().gen_range(2000..2024);

        let file_name = format!("{path}/test-data/{day_str}.{month_str}.{year}_{i}.tar");
        let cloned_file_name = file_name.clone();
        let mut file = File::create(cloned_file_name)?;
        file.write_all(file_name.as_bytes())?;

        println!("File object: {:?}", file);
    }

    Ok(())
}

fn clear_the_day() -> std::io::Result<()> {
    // TODO:
    // 1. get all files of this day
    // 2. keep only most new file
    // 3. delete all other files

    Ok(())
}

fn check_period(config: &Config) -> std::io::Result<()> {
    let period_in_seconds = parse(config.period).unwrap().as_secs();
    let files = fs::read_dir("test-data")?;
    /*

    1. проверяем все периоды, пока не кончатся файлы или не кончится кол-во периодов
    2. для каждого периода выбираем файлы
    3. выбираем, какие файлы оставить
     */

    // collect files data
    let prepared_files_list: &Vec<_> = &files
        .map(|file| {
            let file = file.unwrap();
            let metadata = file.metadata().unwrap();
            let created = metadata.created().unwrap();
            let date_from_filename =
                extract_date_from_file_name(file.file_name().to_str().unwrap());
            (file, created, date_from_filename)
        })
        .collect();

    println!("Prepared files list: {:?}", prepared_files_list);
    find_files_in_period(prepared_files_list);
    // for file in prepared_files_list {
    //     let file = file?;
    //     let metadata = file.metadata()?;
    //     let created = metadata.created()?;
    //     let now = std::time::SystemTime::now();
    //     let duration = now.duration_since(created).unwrap().as_secs();

    //     let date_from_filename = extract_date_from_file_name(file.file_name().to_str().unwrap());
    //     println!("Date from file name: {:?}", date_from_filename);
    // }

    Ok(())
}

fn extract_date_from_file_name(file_name: &str) -> DateTime<Utc> {
    let regexp = regex::Regex::new(r"(\d{2}).(\d{2}).(\d{4})").unwrap();
    let captures = regexp.captures(file_name).unwrap();

    let day = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let month = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
    let year = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();

    println!("Filename: {}", file_name);
    let date = Utc::with_ymd_and_hms(&Utc, year, month, day, 0, 0, 0).unwrap();

    date
}

fn find_files_in_period(files: &Vec<(DirEntry, SystemTime, DateTime<Utc>)>) -> std::io::Result<()> {

    Ok(())
}
