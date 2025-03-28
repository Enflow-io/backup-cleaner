#[cfg(test)]
mod tests {
    use core::panic;
    use std::io::Result as IoResult;

    use crate::{file_generator::{cleanup_files, generate_daily_files}, get_checkers, get_files_list, remove_files, store::Store, Config};
    use crate::{check_files};
    use crate::file_generator::generate_weekly_files;

    #[test]
    fn test_daily_files() -> IoResult<()> {
        let configs: Vec<Config> = vec![
            Config {
                period: "1d",
                qnt: 4,
            }
        ];
        let root_path = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| panic!("Can't get root path"));
        let folder = format!("{}/test-data", root_path);
        let mut store = Store::new();
        let _ = cleanup_files();
        let _ = generate_daily_files(10, Some(&folder));

        Ok(())
    }

    #[test]
    fn test_weekly_files() -> IoResult<()> {
        let configs: Vec<Config> = vec![
            Config {
                period: "1w",
                qnt: 1,
            }
        ];

        let mut store = Store::new();

        let _ = cleanup_files();
        // let _ = generate_weekly_files(10);
        let folder = "test-data";

        let _ = generate_daily_files(27, None);
        let regexp = regex::Regex::new(r"(\d{2}).(\d{2}).(\d{4})").unwrap_or_else(|_| panic!("Can't compile regex"));
        let files_list = get_files_list(&folder, &regexp)?;
        let checkers = get_checkers(configs);
        let _ = check_files(&files_list, &checkers, &mut store, &regexp);

        let _ = remove_files(&store.files_to_delete, folder);

        let file_in_folder = std::fs::read_dir("test-data")?.count();
        assert_eq!(file_in_folder, 1);

        Ok(())

    }

    #[test]
    fn test_daily_and_weekly_files() -> IoResult<()> {
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
        let _ = generate_daily_files(27, None);

        let root_path = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| panic!("Can't get root path"));
        let folder = format!("{}/test-data/", root_path);
        let regexp = regex::Regex::new(r"(\d{2}).(\d{2}).(\d{4})").unwrap_or_else(|_| panic!("Can't compile regex"));
        let files_list = get_files_list(&folder, &regexp)?;
        let checkers = get_checkers(configs);
        let _ = check_files(&files_list, &checkers, &mut store, &regexp);
        let _ = remove_files(&store.files_to_delete, &folder);
        let file_in_folder = std::fs::read_dir(folder)?;
        let files_list: Vec<_> = file_in_folder.collect::<Result<Vec<_>, _>>()?;
        println!("{:#?}", files_list);

        assert_eq!(files_list.len(), 5);
        Ok(())
    }


    #[test]
    fn generate_backups () -> IoResult<()> {
        let path = "/Users/constantine/Projects/Rust/Backups";
        let _ = generate_daily_files(50, Some(&path));
        println!("Files generated");
        Ok(())
    }
    
}