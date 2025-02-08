use std::cell::RefCell;
use chrono::{DateTime, TimeZone, Utc};
use std::fs::{self};
use std::rc::Rc;
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
    let mut store = Rc::new(RefCell::new(Store::new()));

    // формируем конфиг
    // todo: в будущем из .env
    let configs: Vec<Config> = vec![
            Config {
                period: "1w",
                qnt: 2,
            },
            // Config {
            //     period: "1w",
            //     qnt: 7,
            // },
    ];

    // Создаем проверяльщиков
    let checkers = get_checkers(configs, store);

    // Получаем вектор файлов, которые будем проверять
    let files_list = get_files_list().unwrap();

    // насыщаем store информацией, что удалить, а что оставить
    check_files(&files_list, &checkers);


    // println!("Files to keep: {:#?}", store.files_to_keep);
    // println!("Files to delete: {:#?}", store.files_to_delete);

    // todo: производим реальные действия с файлами
}




pub fn check_files(files: &Vec<FileData>, checkers: &Vec<Checker>) {
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

        let mut store = checkers[0].store.borrow_mut();

        // когда все чекеры делали свою работу, добавляем файл в нужный список
        if is_to_keep {
            store.add_file_to_keep(file.file_name());
        } else {
            store.add_file_to_delete(file.file_name());
        }
    }
}

fn remove_files(files: &Vec<String>) -> std::io::Result<()> {

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

fn get_checkers(configs: Vec<Config>, store: Rc<RefCell<Store>>) -> Vec<Checker> {
    let mut checkers = Vec::new();
    for config in configs {
        checkers.push(Checker::new(config.clone(), store.clone()));
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


