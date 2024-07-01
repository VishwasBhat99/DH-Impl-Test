use self::zip_writer::get_zip;
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use rbdate::incr_dt_by_mon_presrv_eom_checked;
use slog::Logger;
use std::convert::TryInto;
use std::fs;
use std::io::BufRead;
mod folder_paths;
mod zip_writer;

pub fn process(config_param: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let files_path = fs::read(config_param.files_path()).expect("Can not read files-path file");

    let curr_dt = NaiveDate::parse_from_str(
        &chrono::offset::Local::now().format("%d%m%Y").to_string(),
        "%d%m%Y",
    )
    .unwrap();

    let folder_paths = folder_paths::get_folder(config_param.files_path());

    for path in folder_paths.file_paths {
        let dirs = fs::read_dir(&path);

        if dirs.is_err() {
            info!(log, "Can not find the path: {}", path);
        } else {
            for dir in dirs.unwrap() {
                let folder = dir.unwrap().path().display().to_string();

                if !folder.ends_with(".zip") {
                    let folder_name = get_folder_name(&folder);

                    info!(log, "got folder name: {}", folder_name);

                    let check_folder = NaiveDate::parse_from_str(&folder_name, "%d%m%Y");

                    if check_folder.is_err() {
                        info!(
                            log,
                            "skipped directory: {}, as not in the date format", &folder
                        );
                    } else {
                        let folder_dt = check_folder.unwrap();

                        if is_older(curr_dt, folder_dt, config_param.months) {
                            let mut zip_name = folder.clone();
                            zip_name.push_str(".zip");

                            get_zip(&folder, &zip_name, log, _diag_log);

                            match fs::remove_dir_all(&folder) {
                                Ok(_) => println!("successfully removed: {}", &folder),
                                Err(_) => println!("Can not remove: {}", &folder),
                            }
                        } else {
                            info!(
                                log,
                                "skipped directory: {}, as not older than {} months",
                                folder,
                                config_param.months()
                            );
                        }
                    }
                } else {
                    info!(
                        log,
                        "skipped directory: {}, as already a zipped folder", folder
                    );
                }
            }
        }
    }
}

fn is_older(curr_dt: NaiveDate, from_dt: NaiveDate, months: i32) -> bool {
    let dt_after_mon = incr_dt_by_mon_presrv_eom_checked(from_dt, months.try_into().unwrap());

    if dt_after_mon < Some(curr_dt) {
        return true;
    }

    return false;
}

fn get_folder_name(folder_path: &String) -> String {
    let mut last_occ = 0;

    for (i, c) in folder_path.chars().enumerate() {
        if c == '\\' || c == '/' {
            last_occ = i;
        }
    }

    let len = folder_path.len();

    (&folder_path[last_occ + 1..len]).to_string()
}
