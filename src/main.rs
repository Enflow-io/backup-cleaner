#![feature(exclusive_range_pattern)]
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use parse_duration::parse;
use rand::Rng;
use std::fs::{self, DirEntry, File};
use std::io::prelude::*;
use std::sync::Arc;
use store::Store;
mod checker;
use checker::Checker;
mod file_data;
mod store;
use file_data::FileData;
mod tests;
mod file_generator;
#[derive(Debug, Clone)]
struct Config {
    period: &'static str,
    qnt: u64,
}

fn main() {
    let mut store = Store::new();

    // формируем конфиг
    // todo: в будущем из .env
    let configs: Vec<Config> = vec![
            Config {
                period: "1d",
                qnt: 7,
            },
            // Config {
            //     period: "1w",
            //     qnt: 7,
            // },
    ];

    // Создаем проверяльщиков
    let mut checkers = Vec::new();
    for config in configs {
        checkers.push(Checker::new(config.clone()));
    }

    // Получаем вектор файлов, которые будем проверять
    let files_list = get_files_list().unwrap();

    // насыщаем store информацией, что удалить, а что оставить
    check_files(&files_list, &checkers, &mut store);


    println!("Files to keep: {:#?}", store.files_to_keep);
    println!("Files to delete: {:#?}", store.files_to_delete);

    // todo: производим реальные действия с файлами
}




pub fn check_files(files: &Vec<FileData>, checkers: &Vec<Checker>, store: &mut Store) {
    // проверяем все файлы с помощью каждого чекера
    // если ни один чекер не выбрал файл,
    //      то добавляем его в список файлов на удаление
    for file in files {
        let mut is_to_keep = true;
        for checker in checkers {
            println!("Checker: {:?}", checker.config.period);
            is_to_keep = checker.check_file(&file, &files);
            if is_to_keep {
                break;
            }
        }

        // когда все чекеры делали свою работу, добавляем файл в нужный список
        if is_to_keep {
            store.add_file_to_keep(file.file_name());
        } else {
            store.add_file_to_delete(file.file_name());
        }
    }
}

fn remove_files(files: &Vec<String>) -> std::io::Result<()> {
    // println!("Files to delete: {:#?}", files);
    let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let test_data_path = format!("{path}/test-data");
    
    let mut clone = files.clone();

    clone.sort();
    println!("Files to delete: {:#?}", clone);

    for file in files {
        fs::remove_file(format!("{test_data_path}/{file}"))?;
    }

    Ok(())
}

fn get_checkers() -> Vec<Checker> {
    let configs: Vec<Config> = vec![
        {
            Config {
                period: "1d",
                qnt: 100,
            }
        },
    ];

    let mut checkers = Vec::new();
    for config in configs {
        checkers.push(Checker::new(config.clone()));
    }

    checkers
}

fn print_configs(configs: &Vec<Config>) {
    for config in configs {
        println!("Period: {}, Qnt: {}", config.period, config.qnt);
    }
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


