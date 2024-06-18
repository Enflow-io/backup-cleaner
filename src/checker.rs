use chrono::Utc;
use parse_duration::parse;

use crate::{extract_date_from_file_name, Config, FileData};

pub(crate) struct Checker {
    pub config: Config,
}

impl Checker {
    pub fn new(config: Config) -> Checker {
        Checker { config: config }
    }

    pub fn check_file(&self, file: &FileData, files_list: &Vec<FileData>) -> bool {
        let mut is_to_keep = true;

        let filename = file.file.file_name();

        // 1. находим рамки периода для файла - start и end
        let now = Utc::now();
        let start_of_day = now.date().and_hms(0, 0, 0);
        let start_of_day_timestamp = start_of_day.timestamp();

        let file_date = extract_date_from_file_name(file.file.file_name().to_str().unwrap());
        let file_date_seconds = file_date.timestamp();
        let period_in_seconds = parse(self.config.period).unwrap().as_secs() as i64;

        let remainder = (start_of_day_timestamp - file_date_seconds) % start_of_day_timestamp;
        let start = start_of_day_timestamp - remainder;
        let end = start + period_in_seconds - 1;

        // 2. находим все файлы, которые попадают в этот период
        let mut files_in_period: Vec<&FileData> = files_list
            .iter()
            .filter(|f| {
                let f_date = extract_date_from_file_name(f.file.file_name().to_str().unwrap());
                let f_date_seconds = f_date.timestamp();
                f_date_seconds >= start && f_date_seconds <= end
            })
            .collect();

        files_in_period.sort_by(|a, b| b.created.cmp(&a.created));

        println!("-----------------");
        println!(
            "Files in period: {:#?}",
            files_in_period
                .iter()
                .map(|f| f.file.file_name())
                .collect::<Vec<_>>()
        );
        println!("-----------------");

        // 3. выбираем самый близкий файл к концу периода
        let most_new_file = files_in_period.first().unwrap();

        // если файл самый новый - оставляем его
        if filename == most_new_file.file.file_name() {
            is_to_keep = true;
        } else {
            is_to_keep = false;
        }

        is_to_keep
    }
}
