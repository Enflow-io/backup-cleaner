use chrono::Utc;
use parse_duration::parse;

use crate::{extract_date_from_file_name, Config, FileData};

pub(crate) struct Checker {
    pub config: Config,
}

impl Checker {
    pub fn new(config: Config) -> Checker {
        Checker {
            config: config
        }
    }

    pub fn check_file(&self, file: &FileData, files_list: &Vec<FileData>) -> bool {

        // 1. находим рамки периода для файла - start и end
        // 2. находим все файлы, которые попадают в этот период
        // 3. выбираем самый близкий файл к концу периода
        // 4. проверяем оставшиеся файлы, можно ли их удалить
        //     если вокруг файла в пределах периода есть файлы, которые не попадают в период, то удаляем
        //     если нет - оставляем

        let mut is_to_keep = true;
        let now_in_ceconds = Utc::now().timestamp();
        let file_date = extract_date_from_file_name(file.file.file_name().to_str().unwrap());
        let file_date_seconds = file_date.timestamp();
        let period_in_seconds = parse(self.config.period).unwrap().as_secs() as i64;

        let remainder = (now_in_ceconds - file_date_seconds) % now_in_ceconds;
        let start = now_in_ceconds - remainder;
        let end = start + period_in_seconds;

        let mut files_in_period: Vec<&FileData> = files_list.iter().filter(|f| {
            let f_date = extract_date_from_file_name(f.file.file_name().to_str().unwrap());
            let f_date_seconds = f_date.timestamp();
            f_date_seconds >= start && f_date_seconds <= end
        }).collect();

        files_in_period.sort_by(|a, b| {
            b.created.cmp(&a.created)
        });

        println!("-----------------");
        println!("Files in period: {:#?}", files_in_period.iter().map(|f| f.file.file_name()).collect::<Vec<_>>());
        println!("-----------------");
        is_to_keep
    }
}

