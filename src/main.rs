use chrono::{DateTime, TimeZone, Utc};
use std::{any, fs::{self}};
use anyhow::{anyhow, Error};
use store::Store;
mod checker;
use checker::Checker;
mod file_data;
mod store;
use file_data::FileData;
mod file_generator;
mod tests;
use clap::Parser;

#[derive(Debug, Clone)]
struct Config<'a> {
    period: &'a str,
    qnt: u64,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// periods configs, example: 1d-3 1w-2 that means 3 files per day and 2 files per week
    #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
    periods: Vec<String>,

    /// folder with files
    #[arg(short, long)]
    folder: String,
}   
fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let folder = args.folder;

    // формируем конфиг
    let configs: Vec<Config> = args.periods.iter().map(|checker| {
        let parsed = checker.split("-").collect::<Vec<&str>>();
        Config {
            period: parsed[0],
            qnt: parsed[1].parse::<u64>().unwrap_or(1),
        }
    }).collect();

    print_configs(&configs);    
    let mut store = Store::new();

    // // Создаем проверяльщиков
    let checkers = get_checkers(configs);

    // // Получаем вектор файлов, которые будем проверять
    let files_list = get_files_list(&folder)?;

    // // насыщаем store информацией, что удалить, а что оставить
    check_files(&files_list, &checkers, &mut store);

    let _ = remove_files(&store.files_to_delete, &folder);

    Ok(())
}

pub fn check_files(files: &Vec<FileData>, checkers: &Vec<Checker>, store: &mut Store) {
    // проверяем все файлы с помощью каждого чекера
    // если ни один чекер не выбрал файл,
    //      то добавляем его в список файлов на удаление

    for file in files {
        let mut is_to_keep = false;
        for checker in checkers {
            is_to_keep = checker.check_file(&file, &files);

            // если хоть один чекер захочет, чтобы файл жил, останавливаем цикл
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

fn remove_files(files: &Vec<String>, folder: &str) -> std::io::Result<()> {
    let mut clone = files.clone();

    clone.sort();
    println!("Files to delete: {:#?}", clone);

    for file in files {
        fs::remove_file(format!("{folder}/{file}"))?;
    }

    Ok(())
}

fn get_checkers(configs: Vec<Config>) -> Vec<Checker> {
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

fn get_files_list(folder: &str) -> std::io::Result<Vec<FileData>> {
    let files = fs::read_dir(folder)?;
    let prepared_files_list: Vec<FileData> = files
        .map(|file| -> Result<FileData, Error> {
            let file = file?;
            let metadata = file.metadata()?;
            let created = metadata.created()?;

            let filename = file.file_name(); // todo: почему в одну строчку не выходит?
            let filename_string = filename.to_str().ok_or_else(|| anyhow!("filename_stringerror"))?;
            
            let date_from_filename =
                extract_date_from_file_name(filename_string)?;
            Ok(FileData {
                file_name: filename_string.to_string(),
                created,
                date_from_filename,
            })
        })
        .filter_map(Result::ok)
        .collect::<Vec<FileData>>();

    Ok(prepared_files_list)
}

fn extract_date_from_file_name(file_name: &str) -> anyhow::Result<DateTime<Utc>>  {
    let regexp = regex::Regex::new(r"(\d{2}).(\d{2}).(\d{4})")?;
    let captures = regexp.captures(file_name).ok_or_else(|| anyhow!("Failed to capture"))?;


    // let day = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
    // let day = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
    // let month = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
    // let year = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();


    let day = captures.get(1)
        .ok_or_else(|| anyhow!("Failed to capture day"))?
        .as_str()
        .parse::<u32>()
        .map_err(|_| anyhow!("Failed to parse day"))?;
    let month = captures.get(2)
        .ok_or_else(|| anyhow!("Failed to capture month"))?
        .as_str()
        .parse::<u32>()
        .map_err(|_| anyhow!("Failed to parse month"))?;
    let year = captures.get(3)
        .ok_or_else(|| anyhow!("Failed to capture year"))?
        .as_str()
        .parse::<i32>()
        .map_err(|_| anyhow!("Failed to parse year"))?;

    // println!("Filename: {}", file_name);
    let date = match Utc::with_ymd_and_hms(&Utc, year, month, day, 0, 0, 0) {
        chrono::LocalResult::Single(date) => date,
        _ => return Err(anyhow!("Failed to create date")),
    };

    Ok(date)
}
