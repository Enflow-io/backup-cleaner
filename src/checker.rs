use chrono::{TimeZone, Utc};
use parse_duration::parse;

use crate::{extract_date_from_file_name, Config, FileData};

pub(crate) struct Checker<'a> {
    pub config: Config<'a>,
}

impl<'a> Checker<'a> {
    pub fn new(config: Config) -> Checker {
        Checker { config }
    }

    fn get_max_file_age(&self, period: &str, qnt: u64) -> i64 {
        let now = Utc::now();
        // let start_of_today = now.date().and_hms(0, 0, 0);
        let start_of_today = now.date_naive().and_hms_opt(0, 0, 0);
        let start_of_today_timestamp = start_of_today.unwrap().and_utc().timestamp();

        let period_in_seconds = (parse(period).unwrap().as_secs() as i64) * (qnt as i64);
        start_of_today_timestamp - period_in_seconds
    }
    fn get_period_bounds(&self, file: &FileData, period: &str) -> (i64, i64) {
        // начал сегодняшнего дня
        // let now = Utc::now().add(chrono::Duration::days(5));
        let now = Utc::now();
        let start_of_day = now.date_naive().and_hms_opt(0, 0, 0).unwrap();
        let start_of_day_timestamp = start_of_day.and_utc().timestamp();

        // файл таймштамп в секундах
        let file_date = extract_date_from_file_name(&file.file_name()).unwrap();
        let file_date_timestamp = file_date.timestamp();

        let period_in_seconds = parse(period).unwrap().as_secs() as i64;

        let period_from_today_to_file_date = start_of_day_timestamp - file_date_timestamp;
        let periods_qnt_passed: i64 = period_from_today_to_file_date / period_in_seconds;

        let start = (start_of_day_timestamp - ((periods_qnt_passed + 1) * period_in_seconds)) + 1;
        let end = start_of_day_timestamp - (periods_qnt_passed * period_in_seconds);
        // let remainder = (start_of_day_timestamp - file_date_timestamp) % start_of_day_timestamp;
        // let start = start_of_day_timestamp - remainder;
        // let end = start + period_in_seconds - 1;

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
                let f_date = extract_date_from_file_name(&f.file_name()).unwrap();
                let f_date_seconds = f_date.timestamp();
                f_date_seconds >= start && f_date_seconds <= end
            })
            .cloned()
            .collect();

        result
    }

    pub fn check_file(&self, file: &FileData, files_list: &Vec<FileData>) -> bool {
        let mut is_to_keep = false;

        let filename = file.file_name();

        let date_from_filename = extract_date_from_file_name(&file.file_name()).unwrap();
        let date_from_filename_in_seconds = date_from_filename.timestamp();
        let max_file_age = self.get_max_file_age(self.config.period, self.config.qnt);
        if date_from_filename_in_seconds < max_file_age {
            return false;
        }


        // 1. находим рамки периода для файла - start и end
        let (start, end) = self.get_period_bounds(&file, &self.config.period);

        // 2. находим все файлы, которые попадают в этот период
        let mut files_in_period = self.find_files_in_period(start, end, files_list);
        files_in_period.sort_by(|a, b| b.created.cmp(&a.created));

        let from_date = Utc.timestamp_opt(start as i64, 0).unwrap();
        let end_date = Utc.timestamp_opt(end as i64, 0).unwrap();
        println!("Period, from: {}, to: {}", from_date, end_date);
        println!(
            "Files in period: {:#?}",
            files_in_period
                .iter()
                .map(|f| f.file_name())
                .collect::<Vec<_>>()
        );

        // 3. выбираем самый молодой и старый файлы
        let most_old_file = files_in_period.first().unwrap();
        let most_new_file = files_in_period.last().unwrap();

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
