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

        // 1. находим рамки периода для файла - start и end
        // 2. находим все файлы, которые попадают в этот период
        // 3. выбираем самый близкий файл к концу периода
        // 4. проверяем оставшиеся файлы, можно ли их удалить
        //     если вокруг файла в пределах периода есть файлы, которые не попадают в период, то удаляем
        //     если нет - оставляем

        let mut is_to_keep = true;
       
        is_to_keep
    }
}

