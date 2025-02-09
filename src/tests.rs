use chrono::NaiveDate;

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use chrono::{Datelike, NaiveDate};
    use crate::{file_generator::{cleanup_files, generate_daily_files}, get_checkers, get_files_list, remove_files, store::Store, Config};
    use crate::{check_files};
    use crate::file_generator::generate_weekly_files;

    #[test]
    fn test_daily_files() {
        let configs: Vec<Config> = vec![
            Config {
                period: "1d",
                qnt: 4,
            }
        ];
        let mut store = Rc::new(RefCell::new(Store::new()));
        let _ = cleanup_files();
        
        
        let _ = generate_daily_files(10);
        let files_list = get_files_list().unwrap();
        let checkers = get_checkers(configs, store.clone());
        let _ = check_files(&files_list, &checkers);

        let _ = remove_files(&store.borrow().files_to_delete);
        let file_in_folder = std::fs::read_dir("test-data").unwrap().count();

        assert_eq!(file_in_folder, 4);
    }

    #[test]
    fn test_weekly_files() {
        let configs: Vec<Config> = vec![
            Config {
                period: "1w",
                qnt: 7,
            }
        ];

        let mut store = Rc::new(RefCell::new(Store::new()));

        let _ = cleanup_files();
        let _ = generate_weekly_files(10);
        let files_list = get_files_list().unwrap();
        let checkers = get_checkers(configs, store.clone());
        let _ = check_files(&files_list, &checkers);


        let _ = remove_files(&store.borrow().files_to_delete);

        let file_in_folder = std::fs::read_dir("test-data").unwrap().count();
        assert_eq!(file_in_folder, 7);

    }

    #[test]
    fn test_daily_and_weekly_files() {
        let configs: Vec<Config> = vec![
            Config {
                period: "1d",
                qnt: 3,
            },
            Config {
                period: "1w",
                qnt: 3,
            }
        ];
        let _ = cleanup_files();
        let mut store = Rc::new(RefCell::new(Store::new()));

        let _ = cleanup_files();
        let _ = generate_daily_files(27);
        let files_list = get_files_list().unwrap();
        let checkers = get_checkers(configs, store.clone());
        let _ = check_files(&files_list, &checkers);

        let _ = remove_files(&store.borrow().files_to_delete);
        let file_in_folder = std::fs::read_dir("test-data").unwrap().count();
        assert_eq!(file_in_folder, 6);
    }

    #[test]
    fn parse_date_from_file_name() {
        let date_str = "05.09.2015";
        let format = "%d.%m.%Y";
        let date = NaiveDate::parse_from_str(date_str, format).unwrap();
        assert_eq!(date.day(), 5);
        assert_eq!(date.month(), 9);
        assert_eq!(date.year(), 2015);

        let date_str = "2015-09-05";
        let format = "%d.%m.%Y";
        let date = NaiveDate::parse_from_str(date_str, format);

        assert!(date.is_err());

        let date_str = "2015-09-05";
        let format = "%Y-%m-%d";
        let date = NaiveDate::parse_from_str(date_str, format).unwrap();

        assert_eq!(date.day(), 5);
        assert_eq!(date.month(), 9);
        assert_eq!(date.year(), 2015);
    }

    #[test]
    fn extract_date_from_name(){
        let file_name = "05.09.2015_1.tar";
        let reg_str = r"(.*)_(.*)\.tar";
        let regexp = regex::Regex::new(reg_str).unwrap();
        let captures = regexp.captures(file_name);
        assert!(captures.is_some());
        assert_eq!(captures.unwrap().get(1).unwrap().as_str(), "05.09.2015");


        let file_name = "05.09.2015_.tar";
        let regexp = regex::Regex::new(reg_str).unwrap();
        let captures = regexp.captures(file_name);

        print!("{:?}", captures);
        assert!(captures.is_some());
    }
}