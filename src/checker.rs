use std::{cell::RefCell, fs::DirEntry, time::Duration};

use chrono::{DateTime, Utc};
use parse_duration::parse;

use crate::{extract_date_from_file_name, store::Store, Config, FileData};

pub(crate) struct Checker {
    pub config: Config,
}

pub struct PeriodBounds {
    pub start: i64,
    pub end: i64,
}

impl Checker {
    pub fn new(config: Config) -> Checker {
        Checker { config }
    }

    fn get_period_bounds(&self, file: &FileData, period: &str) -> (i64, i64) {
        let now = Utc::now();
        let start_of_day = now.date().and_hms(0, 0, 0);
        let start_of_day_timestamp = start_of_day.timestamp();

        let file_date = extract_date_from_file_name(&file.file_name());
        let file_date_seconds = file_date.timestamp();
        let period_in_seconds = parse(period).unwrap().as_secs() as i64;

        let remainder = (start_of_day_timestamp - file_date_seconds) % start_of_day_timestamp;
        let start = start_of_day_timestamp - remainder;
        let end = start + period_in_seconds - 1;

        (start, end)
    }

    fn find_files_in_period(
        &self,
        start: i64,
        end: i64,
        files_list: &Vec<FileData>,
    ) -> Vec<FileData> {
        let result: Vec<FileData> = files_list
            .iter()
            .filter(|f| {
                let f_date = extract_date_from_file_name(&f.file_name());
                let f_date_seconds = f_date.timestamp();
                f_date_seconds >= start && f_date_seconds <= end
            })
            .cloned()
            .collect();

        result
    }

    pub fn check_file(&self, file: &FileData, files_list: &Vec<FileData>) -> bool {
        let mut is_to_keep = true;

        let filename = file.file_name();

        // 1. находим рамки периода для файла - start и end
        let (start, end) = self.get_period_bounds(&file, &self.config.period);

        // 2. находим все файлы, которые попадают в этот период
        let mut files_in_period = self.find_files_in_period(start, end, files_list);
        files_in_period.sort_by(|a, b| b.created.cmp(&a.created));

        println!("Period, from: {}, to: {}", start, end);
        println!(
            "Files in period: {:#?}",
            files_in_period
                .iter()
                .map(|f| f.file_name())
                .collect::<Vec<_>>()
        );

        // 3. выбираем самый молодой и старый файлы
        let most_new_file = files_in_period.first().unwrap();
        let most_old_file = files_in_period.last().unwrap();

        // если файл самый новый - оставляем его
        if filename == most_new_file.file_name() {
            is_to_keep = true;
        } else if filename == most_old_file.file_name() {
            // проверяем, не пустой ли предыдущий период
            let is_previous_period_empty = false;

            // todo: проверить предыдущий период
            // если предыдущий период пустой, а текущий файл - самый старый из списка, тогда не удаляем файл
            // let previous_period_bounds: PeriodBounds = PeriodBounds {
            //     start: start - parse(self.config.period).unwrap().as_secs() as i64,
            //     end: start - 1,
            // };
            //
            // let found_files = self.find_files_in_period(
            //     previous_period_bounds.start,
            //     previous_period_bounds.end,
            //     files_list,
            // ).iter().filter(|f| {}).count();

            // if is_previous_period_empty {
            //     is_to_keep = true;
            // } else {
            //     is_to_keep = false;
            // }

            is_to_keep = false;
        } else {
            // середину удаляем
            is_to_keep = false;
        }

        is_to_keep
    }
}
