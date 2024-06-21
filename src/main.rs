#![feature(exclusive_range_pattern)]
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use parse_duration::parse;
use rand::Rng;
use std::cell::RefCell;
use std::fs::{self, DirEntry, File};
use std::io::prelude::*;
use store::Store;
mod checker;
use checker::Checker;
mod file_data;
mod store;
use file_data::FileData;

#[derive(Debug, Clone)]
struct Config {
    period: &'static str,
    qnt: u64,
}

fn main() {
    let store = RefCell::new(Store::new());
    let checkers = get_checkers(store.clone());

    let files_list = get_files_list().unwrap();

    for file in &files_list {
        // проверяем все файлы с помощью каждого чекера
        // если ни один чекер не выбрал файл, то добавляем его в список файлов на удаление

        let mut is_to_keep = true;
        for checker in &checkers {
            println!("----====== Checker: {:?} =====-----", checker.config.period);
            is_to_keep = checker.check_file(&file, &files_list);
            if is_to_keep {
                break;
            }
        }

        if is_to_keep {
            store.borrow_mut().add_file_to_keep(file.file_name());
        } else {
            store.borrow_mut().add_file_to_delete(file.file_name());
        }
    }


    remove_files(&store.borrow().files_to_delete).unwrap();
    // &store2.files_to_keep.sort();
    println!("Files to keep: {:#?}", store.borrow().files_to_keep);

    // print_configs(&configs);

    // let config = configs.get(0).unwrap();
    // check_period(config);

    // generate_files();
}

fn remove_files(files: &Vec<String>) -> std::io::Result<()> {
    // println!("Files to delete: {:#?}", files);
    let mut clone = files.clone();

    clone.sort();
    println!("Files to delete: {:#?}", clone);

    // for file in files {
    //     fs::remove_file(file)?;
    // }

    Ok(())
}

fn get_checkers(store: RefCell<Store>) -> Vec<checker::Checker> {
    let configs: Vec<Config> = vec![
        {
            Config {
                period: "1d",
                qnt: 100,
            }
        },
        // {
        //     Config {
        //         period: "1w",
        //         qnt: 8,
        //     }
        // },
        // {
        //     Config {
        //         period: "2w",
        //         qnt: 8,
        //     }
        // },
        // {
        //     Config {
        //         period: "1M",
        //         qnt: 12,
        //     }
        // },
        // {
        //     Config {
        //         period: "1y",
        //         qnt: 2,
        //     }
        // },
    ];

    let checkers = configs
        .iter()
        .map(|config| Checker::new(config.to_owned().clone(), store.clone()))
        .collect::<Vec<Checker>>();

    checkers
}

fn print_configs(configs: &Vec<Config>) {
    for config in configs {
        println!("Period: {}, Qnt: {}", config.period, config.qnt);
    }
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

fn get_files_list() -> std::io::Result<Vec<FileData>> {
    let files = fs::read_dir("test-data")?;
    let prepared_files_list: Vec<FileData> = files
        .map(|file| {
            let file = file.unwrap();
            let metadata = file.metadata().unwrap();
            let created = metadata.created().unwrap();
            let date_from_filename =
                extract_date_from_file_name(file.file_name().to_str().unwrap());
            FileData {
                file_name: file.file_name().to_str().unwrap().to_string(),
                created,
                date_from_filename,
            }
        })
        .collect::<Vec<FileData>>();

    Ok(prepared_files_list)
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

    // println!("Prepared files list: {:?}", prepared_files_list);
    // find_files_in_period(prepared_files_list);
    let now_in_ceconds = Utc::now().timestamp();

    for i in 1..=config.qnt {
        let start_time = now_in_ceconds as u64 - (period_in_seconds * i);
        let end_time = start_time + period_in_seconds;
        println!("Period: {}", i);
        println!(
            "Start time: {}, End time: {}, Start formatted: {}",
            start_time,
            end_time,
            Utc.timestamp(start_time as i64, 0)
        );

        for file in prepared_files_list {
            let file_name = file.0.file_name();
            let file_name_str = file_name.to_str().unwrap();
            let date_from_filename = extract_date_from_file_name(file_name_str);
            let date_from_filename_in_seconds = date_from_filename.timestamp() as u64;

            let is_date_in_period = date_from_filename_in_seconds >= start_time
                && date_from_filename_in_seconds <= end_time;

            let start_period_date = Utc.timestamp(start_time as i64, 0);
            let end_period_date = Utc.timestamp(end_time as i64, 0);
            if (is_date_in_period) {
                println!("------------------------------------");
                println!("Start period date: {:?}", start_period_date);
                println!("End period date: {:?}", end_period_date);
                println!("File: {:?}", file_name);
                println!("Date: {:?}", date_from_filename);
                println!("File in period: {:?}", is_date_in_period);
            }
        }
    }

    Ok(())
}

fn extract_date_from_file_name(file_name: &str) -> DateTime<Utc> {
    let regexp = regex::Regex::new(r"(\d{2}).(\d{2}).(\d{4})").unwrap();
    let captures = regexp.captures(file_name).unwrap();

    let day = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let month = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
    let year = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();

    // println!("Filename: {}", file_name);
    let date = Utc::with_ymd_and_hms(&Utc, year, month, day, 0, 0, 0).unwrap();

    date
}
