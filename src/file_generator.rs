use std::{cell::RefCell, fs::{self, File}, io::Write};

use rand::Rng;

use crate::{get_checkers, store::Store};



fn generate_random_files() -> std::io::Result<()> {
    let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let test_data_path = format!("{path}/test-data");
    fs::create_dir_all(test_data_path)?;

    let files_qnt = 10;

    for i in 0..files_qnt {
        let day_num: i32 = rand::thread_rng().gen_range(1..30);
        let mut day_str = day_num.to_string();
        if day_num < 10 {
            day_str = format!("0{}", day_str);
        }

        let month = rand::thread_rng().gen_range(1..12);
        let mut month_str = month.to_string();
        if (month < 10) {
            month_str = format!("0{}", month);
        }
        let year = rand::thread_rng().gen_range(2000..2024);

        let file_name = format!("{path}/test-data/{day_str}.{month_str}.{year}_{i}.tar");
        let cloned_file_name = file_name.clone();
        let mut file = File::create(cloned_file_name)?;
        file.write_all(file_name.as_bytes())?;

        println!("File object: {:?}", file);
    }

    Ok(())
}


pub fn generate_daily_files(files_qnt: i64) -> std::io::Result<()> {
    let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let test_data_path = format!("{path}/test-data");
    fs::create_dir_all(test_data_path)?;

    for i in 0..files_qnt {

        let double_create = i % 2 == 0;
        let today = chrono::Local::now() - chrono::Duration::days(i+1);

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


        // let mut store = Store::new();
        // let checkers = get_checkers(store);
    }

    Ok(())
}

pub fn generate_weekly_files(files_qnt: i64) -> std::io::Result<()> {
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

pub fn cleanup_files() -> std::io::Result<()> {

    let path = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let test_data_path = format!("{path}/test-data");
    println!("Cleaning up {}", test_data_path);
    fs::remove_dir_all(test_data_path)?;

    Ok(())
}