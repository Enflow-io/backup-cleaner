
#[derive(Clone)]
pub struct Store {
    pub files_to_keep: Vec<String>,
    pub files_to_delete: Vec<String>,
}


impl Store {
    pub fn new() -> Store {
        Store {
            files_to_keep: Vec::new(),
            files_to_delete: Vec::new(),
        }
    }

    pub fn add_file_to_keep(&mut self, file: String) {
        self.files_to_keep.push(file);
    }

    pub fn add_file_to_delete(&mut self, file: String) {
        self.files_to_delete.push(file);
    }

}