use std::time::SystemTime;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub(crate) struct FileData {
    pub file_name: String,
    pub created: SystemTime,
    pub date_from_filename: DateTime<Utc>,
}

impl FileData {
   pub fn file_name (&self) -> String {
       self.file_name.clone()
   }
}