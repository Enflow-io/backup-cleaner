
#[cfg(test)]
mod tests {
    use crate::{file_generator::{cleanup_files, generate_daily_files}, get_checkers, get_files_list, remove_files, store::Store, Config};
    use crate::{check_files};
    use crate::file_generator::generate_weekly_files;

    #[test]
    fn test_daily_files() -> std::io::Result<()> {
        let configs: Vec<Config> = vec![
            Config {
                period: "1d",
                qnt: 4,
            }
        ];
        let folder = "test-data";
        let mut store = Store::new();
        let _ = cleanup_files();
        let _ = generate_daily_files(10);

        let regexp = regex::Regex::new(r"(\d{2}).(\d{2}).(\d{4})").unwrap_or_else(|_| panic!("Can't compile regex"));
        let files_list = get_files_list(&folder, &regexp)?;
        let checkers = get_checkers(configs);
        let _ = check_files(&files_list, &checkers, &mut store, &regexp);

        let _ = remove_files(&store.files_to_delete, folder);
        let file_in_folder = std::fs::read_dir("test-data")?.count();

        assert_eq!(file_in_folder, 4);
        Ok(())
    }

    #[test]
    fn test_weekly_files() -> std::io::Result<()> {
        let configs: Vec<Config> = vec![
            Config {
                period: "1w",
                qnt: 7,
            }
        ];

        let mut store = Store::new();

        let _ = cleanup_files();
        let _ = generate_weekly_files(10);
        let folder = "test-data";
        let regexp = regex::Regex::new(r"(\d{2}).(\d{2}).(\d{4})").unwrap_or_else(|_| panic!("Can't compile regex"));
        let files_list = get_files_list(&folder, &regexp)?;
        let checkers = get_checkers(configs);
        let _ = check_files(&files_list, &checkers, &mut store, &regexp);


        let _ = remove_files(&store.files_to_delete, folder);

        let file_in_folder = std::fs::read_dir("test-data")?.count();
        assert_eq!(file_in_folder, 7);

        Ok(())

    }

    #[test]
    fn test_daily_and_weekly_files() -> std::io::Result<()> {
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
        let mut store = Store::new();

        let _ = cleanup_files();
        let _ = generate_daily_files(27);
        let folder = "test-data";
        let regexp = regex::Regex::new(r"(\d{2}).(\d{2}).(\d{4})").unwrap_or_else(|_| panic!("Can't compile regex"));
        let files_list = get_files_list(folder, &regexp)?;
        let checkers = get_checkers(configs);
        let _ = check_files(&files_list, &checkers, &mut store, &regexp);
        println!("F: {:?}", store.files_to_delete);
        let _ = remove_files(&store.files_to_delete, folder);
        let file_in_folder = std::fs::read_dir(folder)?.count();
        assert_eq!(file_in_folder, 6);
        Ok(())
    }
}