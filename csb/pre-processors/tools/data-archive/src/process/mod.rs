use self::get_dates::GetDatesForTwoMonths;
use chrono::NaiveDate;
use configuration_parameters::ConfigurationParameters;
use std::fs::{read_dir, remove_dir, remove_file};
use std::io::prelude::*;
use zip::write::FileOptions;

mod get_dates;

pub fn compress_and_delete(
    zip_file_name: String,
    nxt_aggr_dt: NaiveDate,
    input_folder_path: &str,
    date_format: &str,
    end_date: NaiveDate,
    no_of_days: i64,
) {
    let mut nxt_aggr_dt = nxt_aggr_dt;
    let file = std::fs::File::create(&zip_file_name).unwrap();
    let mut zip_file = zip::ZipWriter::new(file);
    let mut day = 1;
    while nxt_aggr_dt <= end_date {
        let date_folder_name = nxt_aggr_dt.format(date_format).to_string();
        let inp_path = input_folder_path.to_string() + &date_folder_name + "/";
        if read_dir(&inp_path).is_ok() {
            let files = read_dir(&inp_path).unwrap();
            for path in files {
                let path_val = &path.unwrap().path();
                zip_file.start_file_from_path(path_val, FileOptions::default());
                //exclude last date file
                if day != no_of_days {
                    remove_file(path_val).expect("Cannot remove the file.");
                }
            }
            if day != no_of_days {
                remove_dir(&inp_path).unwrap();
            }
        }
        day += 1;
        nxt_aggr_dt = nxt_aggr_dt.succ();
    }
}
pub fn compress_to_zip(
    input_folder_path: &str,
    config_params: &ConfigurationParameters,
    file_name: &str,
    dates: &GetDatesForTwoMonths,
) {
    let date_format = config_params.date_format();
    let dt_extension = dates.start_date_1.format("%m%Y").to_string();
    let zip_file_name = config_params.zip_file_path().to_string()
        + &(file_name.to_string() + &dt_extension + ".zip");

    compress_and_delete(
        zip_file_name,
        dates.start_date_1,
        input_folder_path,
        date_format,
        dates.end_date_1,
        dates.no_of_days_1,
    );

    let dt_extension = dates.start_date_2.format("%m%Y").to_string();
    let zip_file_name = config_params.zip_file_path().to_string()
        + &(file_name.to_string() + &dt_extension + ".zip");
    compress_and_delete(
        zip_file_name,
        dates.start_date_2,
        input_folder_path,
        date_format,
        dates.end_date_2,
        dates.no_of_days_2,
    );
}

pub fn process_name(config_params: &ConfigurationParameters) {
    let input_folder_path = config_params.input_file_path();
    let preprocess_folder = config_params.preprocess_path();
    let preprocess_basel_folder = preprocess_folder.to_string() + &"BASEL/";
    let preprocess_care_folder = preprocess_folder.to_string() + &"CARE/";
    let cfdata_folder = config_params.cfdata_path();
    let cfdata_basel_folder = cfdata_folder.to_string() + &"BASEL/";
    let cfdata_care_folder = cfdata_folder.to_string() + &"CARE/";
    let summary_folder = config_params.summary_path();
    let summary_basel_folder = summary_folder.to_string() + &"BASEL/";
    let summary_care_folder = summary_folder.to_string() + &"CARE/";
    let logs_folder = config_params.logs_path();

    let dates = GetDatesForTwoMonths::new(config_params.as_on_date());

    compress_to_zip(input_folder_path, config_params, "SH_INPUTDATA_", &dates);
    compress_to_zip(
        preprocess_folder,
        config_params,
        "SH_PREPROCESSDATA_",
        &dates,
    );
    compress_to_zip(
        &preprocess_basel_folder,
        config_params,
        "SH_PREPROCESSDATA_BASEL_",
        &dates,
    );
    compress_to_zip(
        &preprocess_care_folder,
        config_params,
        "SH_PREPROCESSDATA_CARE_",
        &dates,
    );
    compress_to_zip(cfdata_folder, config_params, "SH_CFDATA_", &dates);
    compress_to_zip(
        &cfdata_basel_folder,
        config_params,
        "SH_CFDATA_BASEL_",
        &dates,
    );
    compress_to_zip(
        &cfdata_care_folder,
        config_params,
        "SH_CFDATA_CARE_",
        &dates,
    );
    compress_to_zip(summary_folder, config_params, "SH_SUMMARYDATA_", &dates);
    compress_to_zip(
        &summary_basel_folder,
        config_params,
        "SH_SUMMARYDATA_BASEL_",
        &dates,
    );
    compress_to_zip(
        &summary_care_folder,
        config_params,
        "SH_SUMMARYDATA_CARE_",
        &dates,
    );
    compress_to_zip(logs_folder, config_params, "SH_LOGS_", &dates);
}
