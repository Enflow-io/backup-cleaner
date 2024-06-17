use crate::{Config, FileData};

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
        let mut is_to_keep = false;
       
        is_to_keep
    }
}

