use super::{
    extract_lines, macros, read_file, AggregateData, ConfigurationParameters, GetDates, HashMap,
    InputAccount, InputParsedAccount, Logger,
};
use std::fs::{create_dir, metadata, File};

pub fn get_data(
    dates: &GetDates,
    config_params: &ConfigurationParameters,
    bal_org: &mut HashMap<String, AggregateData>,
    account_pool: &mut HashMap<String, Vec<f64>>,
    log: &Logger,
) {
    let mut nxt_aggr_dt = dates.start_date;
    let date_format = config_params.date_format();
    let as_on_dt = config_params.as_on_date().format(date_format).to_string();
    let mut day: usize = 1;
    while nxt_aggr_dt <= dates.end_date {
        let date_folder_name = nxt_aggr_dt.format(date_format).to_string();
        let inp_file_path = config_params
            .input_file_path()
            .replace(&as_on_dt, &date_folder_name);
        let empty_file_path = inp_file_path.to_string();
        if !metadata(inp_file_path.to_string()).is_ok() {
            log_error!(
                log,
                "Average Balance File: `{}` not available for date: `{}`.",
                inp_file_path,
                nxt_aggr_dt.format("%d-%m-%Y"),
            );
            let directory_path = &empty_file_path
                .rsplit_once("/")
                .expect("Could not extract directory path.")
                .0;
            // create the empty file in the directory
            let _ = create_dir(directory_path).is_ok();
            let _ = File::create(&empty_file_path);
        }
        let mut reader = read_file(&inp_file_path, &empty_file_path, log);
        for (line_num, lines) in reader.deserialize().enumerate() {
            let input_account: InputAccount = extract_lines(line_num, lines, &inp_file_path, log);
            let account: InputParsedAccount = input_account.parse();
            let mut builder = AggregateData::new();
            let mut def_bals = vec![0.0; dates.no_of_days as usize];
            def_bals[day - 1] = account.amt;
            if let Some(mut bals) = account_pool.remove(&account.acc_no) {
                bals[day - 1] = account.amt;
                account_pool.insert(account.acc_no.to_string(), bals);
            } else {
                account_pool.insert(account.acc_no.to_string(), def_bals.clone());
            }

            for (acc_no, def_bals) in account_pool.iter_mut() {
                let mut index = 0;
                let mut last_non_zero_index = 0;
                let mut first_non_zero_index = 0;
                let mut first_index_assigned = false;
                for i in def_bals.clone() {
                    if i != 0.0 {
                        if !first_index_assigned {
                            first_non_zero_index = index;
                            first_index_assigned = true;
                        }
                        last_non_zero_index = index;
                    }
                    index += 1;
                }
                if last_non_zero_index > 1 {
                    for index in first_non_zero_index..last_non_zero_index + 1 {
                        if def_bals[index] == 0.0 {
                            def_bals[index] = def_bals[index - 1];
                            let new_holiday_acc = InputParsedAccount {
                                acc_no: acc_no.to_string(),
                                date: account.date,
                                amt: def_bals[index],
                                int_rt: account.int_rt,
                            };
                            let mut builder = AggregateData::new();
                            builder.add(&new_holiday_acc);
                            bal_org
                                .entry(account.acc_no.to_string())
                                .and_modify(|m| m.add(&new_holiday_acc))
                                .or_insert(builder);
                        }
                    }
                }
            }
            builder.add(&account);
            bal_org
                .entry(account.acc_no.to_string())
                .and_modify(|m| m.add(&account))
                .or_insert(builder);
        }
        nxt_aggr_dt = nxt_aggr_dt.succ();
        day += 1;
    }
}
