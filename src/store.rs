pub struct Store {
    files_to_keep: Vec<String>,
    files_to_delete: Vec<String>,
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

    fn get_files_to_keep(&self) -> &Vec<String> {
        &self.files_to_keep
    }

    fn get_files_to_delete(&self) -> &Vec<String> {
        &self.files_to_delete
    }

    fn remove_file_from_keep(&mut self, file: String) {
        let index = self.files_to_keep.iter().position(|x| *x == file).unwrap();
        self.files_to_keep.remove(index);
    }

    fn remove_file_from_delete(&mut self, file: String) {
        let index = self.files_to_delete.iter().position(|x| *x == file).unwrap();
        self.files_to_delete.remove(index);
    }

}