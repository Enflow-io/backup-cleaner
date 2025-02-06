
#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::{file_generator::{cleanup_files, generate_daily_files}, get_checkers, get_files_list, store::Store};
    use crate::{check_files};
    use super::*;

    #[test]
    fn test_daily_files() {
        let mut store = Store::new();
        let _ = cleanup_files();
        let _ = generate_daily_files();
        let files_list = get_files_list().unwrap();
        let checkers = get_checkers();
        let _ = check_files(&files_list, &checkers, &mut store);

        let file_in_folder = std::fs::read_dir("test-data").unwrap().count();
        assert_eq!(file_in_folder, 15);
    }
}