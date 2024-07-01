use aggregator::account_field_names::AccFieldNames;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::NaiveDate;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::types::Type;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;
use std::time::SystemTime;

mod account_field_names;

pub fn aggregate_cashflows(
    config_params: ConfigurationParameters,
    logger: &Logger,
    _diag_logger: &Logger,
) {
    let start_time = SystemTime::now();
    let mut op_path = String::new();
    op_path.push_str(config_params.output_file_path());
    op_path.push_str(".dat");
    let op_file_name = Path::new(&op_path)
        .file_name()
        .expect("Cannot read output file name.")
        .to_str()
        .expect("Cannot convert file name to &str.");
    let mut writer = match buf_file_wrtr(&op_path, None) {
        Ok(wrtr) => wrtr,
        Err(error) => {
            panic!(
                "Could not create file: `{}` on location `{}` : {:?}.",
                config_params.output_file_path(),
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    let keys = AccFieldNames::new_from_path(config_params.req_fields_file_path());
    let mut account_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );
    let field_reader = reader::Reader::new_at_path(
        config_params.account_metadata_file_path(),
        config_params.input_file_path(),
    );
    let data_origin = config_params.source();
    let as_on_date = config_params.as_on_date();
    let scenario_num = "1";
    let n_acc_id = "1";
    let org_unit_code = "NA";
    let identity_code = "";
    let cf_type = "I";
    let mut header = String::new();
    header.push_str("HDR~");
    header.push_str(&config_params.as_on_date());
    header.push_str("~");
    header.push_str(op_file_name);
    header.push_str("\n");
    write_data(&mut writer, header);
    let all_bucket_years = gen_all_bucket_years(&as_on_date.as_str(), config_params.bucket_years());
    let mut count = 0;
    let mut total_cfs: i64 = 0;
    let mut total_rec: i64 = 0;
    let mut tot_bal = 0.0;
    for mut account in account_reader.iter() {
        let currency_code = account
            .get_string_for_key(&keys.currency)
            .unwrap_or(&"INR".to_string())
            .to_string();
        total_rec += 1;
        let mut op = String::new();
        let gl_code = match field_reader.get_field_type(&keys.gl_code) {
            Some(typ) => match typ {
                Type::I64 => account
                    .get_i64_for_key(&keys.gl_code)
                    .unwrap_or(0)
                    .to_string(),
                Type::F64 => account
                    .get_f64_for_key(&keys.gl_code)
                    .unwrap_or(0.0)
                    .to_string(),
                Type::String => account
                    .get_string_for_key(&keys.gl_code)
                    .unwrap_or(&"NA".to_string())
                    .to_string(),
                _ => "NA".to_string(),
            },
            None => "NA".to_string(),
        };
        let account_num = match field_reader.get_field_type(&keys.account_num) {
            Some(typ) => match typ {
                Type::I64 => account
                    .get_i64_for_key(&keys.account_num)
                    .unwrap_or(0)
                    .to_string(),
                Type::F64 => account
                    .get_f64_for_key(&keys.account_num)
                    .unwrap_or(0.0)
                    .to_string(),
                Type::String => account
                    .get_string_for_key(&keys.account_num)
                    .unwrap_or(&"".to_string())
                    .to_string(),
                _ => "1".to_string(),
            },
            None => "1".to_string(),
        };
        let currency_type_code = "1";
        let mut cf_seq = 1;
        for cf in account
            .remove_cfs_for_key(&keys.cashflows)
            .expect("Error while removing cashflow from the pool of cashflows.")
            .iter_mut()
        {
            total_cfs += 1;
            let principal_amount = cf.principal_amount;
            let interest_amount = cf.interest_amount;
            let cf_date = naivedate_from_timestamp(cf.get_date());
            let mut fin_ele_code;
            if principal_amount != 0.0 {
                tot_bal += principal_amount;
                count += 1;
                fin_ele_code = 210;
                append_cf(
                    &mut op,
                    cf_seq,
                    fin_ele_code,
                    &gl_code,
                    &account_num,
                    &currency_type_code,
                    principal_amount,
                    cf_date,
                    data_origin,
                    &as_on_date,
                    &currency_code,
                    scenario_num,
                    org_unit_code,
                    identity_code,
                    &all_bucket_years,
                    &config_params.source_sys_code(),
                    cf_type,
                    n_acc_id,
                );
                cf_seq += 1;
            }
            if interest_amount != 0.0 {
                count += 1;
                fin_ele_code = 430;
                append_cf(
                    &mut op,
                    cf_seq,
                    fin_ele_code,
                    &gl_code,
                    &account_num,
                    &currency_type_code,
                    interest_amount,
                    cf_date,
                    data_origin,
                    &as_on_date,
                    &currency_code,
                    scenario_num,
                    org_unit_code,
                    identity_code,
                    &all_bucket_years,
                    &config_params.source_sys_code(),
                    cf_type,
                    n_acc_id,
                );
                cf_seq += 1;
            }
        }
        write_data(&mut writer, op);
    }
    let mut footer = String::new();
    footer.push_str("TRL~");
    footer.push_str(&config_params.as_on_date());
    footer.push_str("~");
    footer.push_str(&count.to_string());
    footer.push_str("\n");
    write_data(&mut writer, footer);

    let health_stat = HealthReport::new(total_rec, total_rec, 0, tot_bal, tot_bal, total_cfs);
    health_stat.gen_health_rpt(config_params.output_file_path());

    let total_duration = print_return_time_since!(start_time);
    log_info!(logger, "Total time for aggregation: {:?}", total_duration);
}

pub fn write_data(writer: &mut BufWriter<File>, op: String) {
    let output_as_bytes = op.as_bytes();
    match writer.write(output_as_bytes) {
        Ok(_val) => {}
        Err(err) => println!("Error writing to output file. Error: {}", err),
    }
}

fn naivedate_from_timestamp(t: i64) -> NaiveDate {
    let naive_date_time = rbdate::NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}

fn append_cf(
    op: &mut String,
    cf_no: i64,
    fin_ele_code: i64,
    gl_code: &str,
    account_num: &str,
    currency_type_code: &str,
    amount: f64,
    date: NaiveDate,
    data_origin: &str,
    as_on_date: &str,
    currency_code: &str,
    scenario_num: &str,
    org_unit_code: &str,
    identity_code: &str,
    all_bucket_years: &Vec<NaiveDate>,
    source_sys_code: &str,
    cf_type: &str,
    n_acc_id: &str,
) {
    let modified_acc_num: String = source_sys_code.to_string() + &account_num.to_string();
    let ason: NaiveDate =
        NaiveDate::parse_from_str(as_on_date, "%Y%m%d").expect("Could not parse as on date");
    op.push_str(&modified_acc_num);
    op.push_str("~");
    op.push_str(&ason.format("%d/%m/%Y").to_string());
    op.push_str("~");
    op.push_str(&cf_no.to_string());
    op.push_str("~");
    op.push_str(scenario_num);
    op.push_str("~");
    op.push_str(&date.format("%d/%m/%Y").to_string());
    op.push_str("~");
    op.push_str(data_origin);
    op.push_str("~");
    op.push_str(cf_type);
    op.push_str("~");
    op.push_str(&amount.to_string());
    op.push_str("~");
    if fin_ele_code == 210 {
        op.push_str("P");
    } else {
        op.push_str("I");
    }
    op.push_str("~");
    op.push_str(currency_code);
    op.push_str("~");
    op.push_str(org_unit_code);
    op.push_str("~");
    op.push_str(&date.format("%Y%m%d").to_string());
    op.push_str("~");
    op.push_str(n_acc_id);
    op.push_str("~");
    op.push_str(currency_type_code);
    op.push_str("~");
    op.push_str(gl_code);
    op.push_str("\n");
}

fn gen_all_bucket_years(as_on_date: &str, years: i32) -> Vec<NaiveDate> {
    let date_parse = rbdate::DateParser::new("%Y%m%d".to_string(), false);
    let as_on_dt = date_parse.parse(as_on_date);
    let mut all_bucket_years: Vec<NaiveDate> = Vec::new();
    for i in 1..=years {
        let next_year = match rbdate::incr_dt_by_mon_presrv_eom_checked(as_on_dt, (i * 12) as usize)
        {
            None => continue,
            Some(data) => data,
        };
        all_bucket_years.push(next_year);
    }
    all_bucket_years
}

fn get_bucket_year_for_date(cf_date: &NaiveDate, all_bucket_years: &Vec<NaiveDate>) -> i32 {
    for (i, curr_date) in all_bucket_years.iter().enumerate() {
        if cf_date <= curr_date {
            return (i as i32) + 1;
        }
    }
    -1
}

#[test]
fn test_bucketing_years() {
    let all_bucket_years = gen_all_bucket_years("20200531", 100);
    print!("{}", all_bucket_years[0]);
    let cf_date = NaiveDate::parse_from_str("20200724", "%Y%m%d").unwrap();
    assert_eq!(get_bucket_year_for_date(&cf_date, &all_bucket_years), 1);
}
