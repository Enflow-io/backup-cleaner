use std::io::Result as IoResult;
use std::{fs::{self, File}, io::Write};
#[allow(dead_code)]
pub fn generate_daily_files(files_qnt: i64, mayby_path: Option<&str>) -> IoResult<()> {
    let root_path = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| panic!("Can't get root path"));
    let path = match mayby_path {
        Some(path) => path.to_string(),
        None => format!("{root_path}/test-data").to_string()
    };
        
    
    fs::create_dir_all(&path)?;

    for i in 0..files_qnt {
        let double_create = i % 2 == 0;
        let today = chrono::Local::now() - chrono::Duration::days(i);

        let day_str = today.format("%d").to_string();
        let month_str = today.format("%m").to_string();
        let year = today.format("%Y").to_string();

        let file_name = format!("{path}/{day_str}.{month_str}.{year}_{i}.tar");
        let cloned_file_name = file_name.clone();
        let mut file = File::create(cloned_file_name)?;
        file.write_all(file_name.as_bytes())?;

        if double_create {
            let second_file_name = format!("{path}/{day_str}.{month_str}.{year}_{i}_2.tar");
            let mut file = File::create(second_file_name.clone())?;
            file.write_all(second_file_name.as_bytes())?;
        }
    }

    Ok(())
}

#[allow(dead_code)]
pub fn generate_weekly_files(files_qnt: i64) -> IoResult<()> {
    let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let test_data_path = format!("{path}/test-data");
    fs::create_dir_all(test_data_path)?;


    for i in 0..files_qnt {

        let double_create = i % 2 == 0;
        let today = chrono::Local::now() - chrono::Duration::weeks(i+1);

        let day_str = today.format("%d").to_string();
        let month_str = today.format("%m").to_string();
        let year = today.format("%Y").to_string();

        let file_name = format!("{path}/test-data/{day_str}.{month_str}.{year}_{i}.tar");
        let cloned_file_name = file_name.clone();
        let mut file = File::create(cloned_file_name)?;
        file.write_all(file_name.as_bytes())?;

        if double_create {
            let second_file_name = format!("{path}/test-data/{day_str}.{month_str}.{year}_{i}_2.tar");
            let mut file = File::create(second_file_name.clone())?;
            file.write_all(second_file_name.as_bytes())?;
        }
    }
    Ok(())
}

#[allow(dead_code)]
pub fn cleanup_files() -> IoResult<()> {

    let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let test_data_path = format!("{path}/test-data");
    println!("Cleaning up {}", test_data_path);
    fs::remove_dir_all(test_data_path)?;

    Ok(())
}