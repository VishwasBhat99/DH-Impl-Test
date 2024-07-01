use super::chrono::NaiveDate;
use super::process_data::calc_duration_data;
use super::read_data::account_structs::*;
use std::fs::File;
use std::io::Write;
pub fn write_account_info(mut output_file: &File, account: AccountInfo, delimiter: &str) {
    let output_data = format!("{}", account.display(delimiter));
    output_file
        .write_all(output_data.as_bytes())
        .expect("unable to write data");
}
pub fn write_aggrdata(
    mut output_file: &File,
    aggrmap: Accounts,
    asondate: NaiveDate,
    delimiter: &str,
) {
    for (aggrkey, aggrdata) in aggrmap {
        let output_data = get_aggrdata_write_data(aggrkey, aggrdata, asondate, delimiter);
        output_file
            .write_all(output_data.as_bytes())
            .expect("unable to write");
    }
}

pub fn get_aggrdata_write_data(
    aggrkey: AggrKey,
    aggrdata: AggrData,
    asondate: NaiveDate,
    delimiter: &str,
) -> String {
    let duration = calc_duration_data(&aggrdata);

    let output_data = format!(
        "{}{}{}{}{}{}{}{}{}{}{}",
        asondate,
        delimiter,
        aggrkey.llg_id,
        delimiter,
        aggrkey.currency_id,
        delimiter,
        aggrdata.balance,
        delimiter,
        aggrdata.balance,
        delimiter,
        duration
    )
    .to_string();
    output_data
}
