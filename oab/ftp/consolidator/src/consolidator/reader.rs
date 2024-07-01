use super::{
    current_dir, macros, AggregateData, ConfigurationParameters, GetDates, HashMap, InputAccount,
    InputParsedAccount, Logger, ReaderBuilder,
};
use std::fs::OpenOptions;

pub fn get_data(
    dates: &GetDates,
    config_params: &ConfigurationParameters,
    bal_org: &mut HashMap<String, AggregateData>,
    log: &Logger,
) {
    let mut nxt_aggr_dt = dates.start_date;
    let date_format = config_params.date_format();
    let as_on_dt = config_params.as_on_date().format(date_format).to_string();
    while nxt_aggr_dt <= dates.end_date {
        let date_folder_name = nxt_aggr_dt.format(date_format).to_string();
        let inp_file_path = config_params
            .input_file_path()
            .replace(&as_on_dt, &date_folder_name);
        let empty_file_path = inp_file_path.to_string();
        let _ = OpenOptions::new().create(true).open(&empty_file_path);
        let mut reader = match ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'|')
            .from_path(inp_file_path.to_string())
        {
            Ok(read) => read,
            Err(error) => {
                log_error!(
                    log,
                    "Could not find file `{}` on location `{}` : {}.",
                    inp_file_path,
                    current_dir()
                        .expect("Error while getting current directory path.")
                        .display(),
                    error
                );
                return;
            }
        };
        for (line_num, lines) in reader.deserialize().enumerate() {
            let input_account: InputAccount = match lines {
                Ok(line) => line,
                Err(error) => {
                    log_error!(
                        log,
                        "Unable to read file `{}` at line number: `{}` : {}",
                        inp_file_path,
                        line_num + 1,
                        error
                    );
                    Default::default()
                }
            };
            let account: InputParsedAccount = input_account.parse();
            let mut builder = AggregateData::default();
            builder.add(&account);
            bal_org
                .entry(account.account_id.to_string())
                .and_modify(|m| m.add(&account))
                .or_insert(builder);
        }
        nxt_aggr_dt = nxt_aggr_dt.succ();
    }
}
