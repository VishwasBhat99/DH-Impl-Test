use aggregator::account_field_names::AccFieldNames;
use aggregator::reader::account_with_cfs::get_field_value;
use aggregator::structs::{OpDrilldownReport, OpLeadingFields, OpTrailingFields};
use calamine::{open_workbook_auto, Reader};
use chrono::Local;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::{num_days_start_to_end, NaiveDate};
use sdb_dyn_proto_rdr::reader;
use sdb_io::buf_file_wrtr;
use slog::Logger;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

mod account_field_names;
pub mod config;
mod structs;

struct Data {
    int_rate: f64,
    benchmark: String,
    bm_rate: f64,
    int_spread: f64,
    rt_flag: String,
    os_amt: f64,
    rt_type: String,
    mis2: String,
    psl_code: String,
    psl_rt: f64,
}

pub fn aggregate_cashflows(
    config_params: ConfigurationParameters,
    logger: &Logger,
    _diag_logger: &Logger,
) {
    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let tot_amt = 0.0;
    let mut tot_cfs: usize = 0;
    let mut summary_rows = 0;
    let mut drilldown_rows = 0;
    let as_on_date: NaiveDate = NaiveDate::parse_from_str(&config_params.as_on_date(), "%Y%m%d")
        .expect("Could not parse as on date");
    let files_config = config::get_files(config_params.config_file_path());
    //Output file for summary report.
    let mut summary_report_path = String::new();
    summary_report_path.push_str(&files_config.summary_file_path);
    let mut summary_writer = match buf_file_wrtr(&summary_report_path, None) {
        Ok(wrtr) => wrtr,
        Err(error) => {
            panic!(
                "Could not create file: `{}` at location `{}` : {:?}.",
                files_config.summary_file_path,
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };

    //Output file for drilldown report.
    let mut drilldown_report_path = String::new();
    drilldown_report_path.push_str(&files_config.drilldown_file_path);
    let mut drilldown_writer = match buf_file_wrtr(&drilldown_report_path, None) {
        Ok(wrtr) => wrtr,
        Err(error) => {
            panic!(
                "Could not create file: `{}` at location `{}` : {:?}.",
                files_config.drilldown_file_path,
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };

    let summary_header ="Account Number|Source system|Customer Id|Customer Name|Product code|Scheme ID|Booking Date|Validity Date|Maturity Date|MIS 1|Org MIS 2|New MIS 2|MIS 3|CCY|Org PSL Mapping|New PSL Mapping|Source GL|Original Amount|Previous month o/s Amt|Current o/s amount|Old Rate Type|New Rate Type|Old BM|New BM|Old BM Rate|New BM Rate|Old Interest Spread|New Interest Spread|Old Interest Rate|New Interest Rate|Old PSL Rate|New PSL Rate|Interest Rate differential\n";
    let drilldown_header ="Account Number|Source system|Customer Id|Customer Name|Product code|Scheme ID|Booking Date|Validity Date|Maturity Date|MIS 1|Org MIS 2|New MIS 2|MIS 3|CCY|Org PSL Mapping|New PSL Mapping|Source GL|Original Amount|Previous month o/s Amt|Current o/s amount|Old Rate Type|New Rate Type|Old BM|New BM|Old BM Rate|New BM Rate|Old Interest Spread|New Interest Spread|Old Interest Rate|New Interest Rate|Old PSL Rate|New PSL Rate|Interest Rate differential|FTM impact|Residual Tenor|Residual Tenor Impact|Present Value|ALM line|IA Line|Concat|Division|NPA Type|Raw BM|Final BM|Old Rate Flag|New Rate Flag|Cash Flow Date\n";
    write_data(&mut summary_writer, summary_header.to_string(), logger);
    write_data(&mut drilldown_writer, drilldown_header.to_string(), logger);

    let mut psl_map: HashMap<String, String> = HashMap::new();
    let mut psl_master_excel =
        open_workbook_auto(&files_config.psl_master_file).expect("Unable to open PSL Master File.");
    if let Some(Ok(reader)) = psl_master_excel.worksheet_range(&files_config.psl_sheet_name) {
        for row in reader.rows().skip(1) {
            psl_map.insert(row[0].to_string(), row[1].to_string());
        }
    }

    for file in files_config.input_files {
        let keys = AccFieldNames::new_from_path(&file.req_fields_file_path);
        let mut prev_mth_reader =
            reader::Reader::new_at_path(&file.account_metadata_file_path, &file.prev_mth_src_file);
        let prev_mth_reader_for_get_val =
            reader::Reader::new_at_path(&file.account_metadata_file_path, &file.prev_mth_src_file);

        let mut prev_mth_map: HashMap<String, Data> = HashMap::new();
        for account in prev_mth_reader.iter() {
            let acc_no = get_field_value(
                &account,
                &prev_mth_reader_for_get_val,
                keys.acc_no.to_owned(),
            )
            .unwrap_or("NA".to_string());
            let old_int_rt = get_field_value(
                &account,
                &prev_mth_reader_for_get_val,
                keys.int_rt.to_owned(),
            )
            .unwrap_or("NA".to_string())
            .parse()
            .unwrap_or(0.0);
            let old_benchmark = get_field_value(
                &account,
                &prev_mth_reader_for_get_val,
                keys.benchmark.to_owned(),
            )
            .unwrap_or("NA".to_string());
            let int_spread = get_field_value(
                &account,
                &prev_mth_reader_for_get_val,
                keys.int_spread.to_owned(),
            )
            .unwrap_or("NA".to_string())
            .parse()
            .unwrap_or(0.0);
            let os_amt = get_field_value(
                &account,
                &prev_mth_reader_for_get_val,
                keys.cur_os_amt.to_owned(),
            )
            .unwrap_or("NA".to_string())
            .parse()
            .unwrap_or(0.0);

            let bm_rate = get_field_value(
                &account,
                &prev_mth_reader_for_get_val,
                keys.bm_rate.to_owned(),
            )
            .unwrap_or("NA".to_string())
            .parse()
            .unwrap_or(0.0);
            let rt_type = get_field_value(
                &account,
                &prev_mth_reader_for_get_val,
                keys.rt_type.to_owned(),
            )
            .unwrap_or("NA".to_string());
            let mis2 =
                get_field_value(&account, &prev_mth_reader_for_get_val, keys.mis2.to_owned())
                    .unwrap_or("NA".to_string());
            let psl_code = get_field_value(
                &account,
                &prev_mth_reader_for_get_val,
                keys.psl_code.to_owned(),
            )
            .unwrap_or("NA".to_string());
            let psl_rt = get_field_value(
                &account,
                &prev_mth_reader_for_get_val,
                keys.psl_rt.to_owned(),
            )
            .unwrap_or("NA".to_string())
            .parse()
            .unwrap_or(0.0);
            account.get_f64_for_key(&keys.psl_rt).unwrap_or(0.0);
            let rt_flag = get_field_value(
                &account,
                &prev_mth_reader_for_get_val,
                keys.rt_flag.to_owned(),
            )
            .unwrap_or("NA".to_string());

            prev_mth_map.insert(
                acc_no,
                Data {
                    int_rate: old_int_rt,
                    benchmark: old_benchmark,
                    int_spread,
                    os_amt,
                    bm_rate,
                    rt_type,
                    mis2,
                    psl_code,
                    psl_rt,
                    rt_flag,
                },
            );
        }

        let mut cur_mth_reader =
            reader::Reader::new_at_path(&file.account_metadata_file_path, &file.cur_mth_src_file);

        let field_reader =
            reader::Reader::new_at_path(&file.account_metadata_file_path, &file.cur_mth_src_file);

        for mut account in cur_mth_reader.iter() {
            acc_enc += 1;
            let acc_no = get_field_value(&account, &field_reader, keys.acc_no.to_owned())
                .unwrap_or("NA".to_string());
            if let Some(data) = prev_mth_map.get(&acc_no) {
                acc_succ += 1;
                let psl_code = get_field_value(&account, &field_reader, keys.psl_code.to_owned())
                    .unwrap_or("NA".to_string());
                if psl_code != data.psl_code {
                    let cust_id = get_field_value(&account, &field_reader, keys.cust_id.to_owned())
                        .unwrap_or("NA".to_string());
                    let cust_name =
                        get_field_value(&account, &field_reader, keys.cust_name.to_owned())
                            .unwrap_or("NA".to_string());
                    let pdt_code =
                        get_field_value(&account, &field_reader, keys.pdt_code.to_owned())
                            .unwrap_or("NA".to_string());
                    let scheme_id =
                        get_field_value(&account, &field_reader, keys.scheme_id.to_owned())
                            .unwrap_or("NA".to_string());
                    let booking_dt =
                        get_field_value(&account, &field_reader, keys.booking_dt.to_owned())
                            .unwrap_or("NA".to_string())
                            .parse()
                            .unwrap_or(0);
                    let booking_dt = naivedate_from_timestamp(booking_dt)
                        .format("%d-%m-%Y")
                        .to_string();
                    let validity_dt =
                        get_field_value(&account, &field_reader, keys.validity_dt.to_owned())
                            .unwrap_or("NA".to_string())
                            .parse()
                            .unwrap_or(0);
                    let validity_dt = naivedate_from_timestamp(validity_dt)
                        .format("%d-%m-%Y")
                        .to_string();

                    let mat_dt = get_field_value(&account, &field_reader, keys.mat_dt.to_owned())
                        .unwrap_or("NA".to_string())
                        .parse()
                        .unwrap_or(0);

                    let mat_dt = naivedate_from_timestamp(mat_dt)
                        .format("%d-%m-%Y")
                        .to_string();
                    let ccy = get_field_value(&account, &field_reader, keys.ccy.to_owned())
                        .unwrap_or("NA".to_string());
                    let mis1 = get_field_value(&account, &field_reader, keys.mis1.to_owned())
                        .unwrap_or("NA".to_string());
                    let org_mis2 = &data.mis2;
                    let mis2 = get_field_value(&account, &field_reader, keys.mis2.to_owned())
                        .unwrap_or("NA".to_string());
                    let mis3 = get_field_value(&account, &field_reader, keys.mis3.to_owned())
                        .unwrap_or("NA".to_string());
                    let def_str_val = String::from("NA");
                    let org_psl_mapping = psl_map.get(&data.psl_code).unwrap_or(&def_str_val);
                    let new_psl_mapping = psl_map.get(&psl_code).unwrap_or(&def_str_val);
                    let source_gl =
                        get_field_value(&account, &field_reader, keys.source_gl.to_owned())
                            .unwrap_or("NA".to_string());
                    let org_amt = get_field_value(&account, &field_reader, keys.org_amt.to_owned())
                        .unwrap_or("NA".to_string());
                    let prev_os_amt = &data.os_amt;
                    let os_amt = account.get_f64_for_key(&keys.cur_os_amt).unwrap_or(0.00);
                    let old_rt_type = &data.rt_type;
                    let new_rt_type =
                        get_field_value(&account, &field_reader, keys.rt_type.to_owned())
                            .unwrap_or("NA".to_string());
                    let old_benchmark = &data.benchmark;
                    let benchmark =
                        get_field_value(&account, &field_reader, keys.benchmark.to_owned())
                            .unwrap_or("NA".to_string());
                    let old_bm_rt = &data.bm_rate;
                    let new_bm_rt =
                        get_field_value(&account, &field_reader, keys.benchmark.to_owned())
                            .unwrap_or("NA".to_string());
                    let new_int_spread =
                        get_field_value(&account, &field_reader, keys.int_spread.to_owned())
                            .unwrap_or("NA".to_string());
                    let old_int_rt = &data.int_rate;
                    let new_int_rt =
                        get_field_value(&account, &field_reader, keys.int_rt.to_owned())
                            .unwrap_or("NA".to_string())
                            .parse()
                            .unwrap_or(0.0);
                    let int_diff = new_int_rt - old_int_rt;
                    let new_psl_rt =
                        get_field_value(&account, &field_reader, keys.psl_rt.to_owned())
                            .unwrap_or("NA".to_string())
                            .parse()
                            .unwrap_or(0.0);
                    let leading_fields = OpLeadingFields {
                        account_number: acc_no,
                        source_system: file.source_system.to_string(),
                        customer_id: cust_id,
                        customer_name: cust_name,
                        product_code: pdt_code,
                        scheme_id,
                        booking_date: booking_dt,
                        validity_date: validity_dt,
                        maturity_date: mat_dt,
                        mis1,
                        org_mis2: org_mis2.to_string(),
                        new_mis2: mis2,
                        mis3,
                        ccy,
                        org_psl_mapping: org_psl_mapping.to_string(),
                        new_psl_mapping: new_psl_mapping.to_string(),
                        source_gl,
                        original_amount: org_amt,
                        previous_os_amount: prev_os_amt.to_string(),
                        current_os_amount: os_amt.to_string(),
                        old_rate_type: old_rt_type.to_string(),
                        new_rate_type: new_rt_type,
                        old_bm: old_benchmark.to_string(),
                        new_bm: benchmark,
                        old_bm_rate: old_bm_rt.to_string(),
                        new_bm_rate: new_bm_rt.to_string(),
                        old_int_spread: data.int_spread.to_string(),
                        new_int_spread,
                        int_rt_prev_mth: old_int_rt.to_string(),
                        int_rt_cur_mth: new_int_rt.to_string(),
                        old_psl_rate: data.psl_rt.to_string(),
                        new_psl_rate: new_psl_rt.to_string(),
                        int_diff: int_diff.to_string(),
                    };

                    let alm_line =
                        get_field_value(&account, &field_reader, keys.alm_line.to_owned())
                            .unwrap_or("NA".to_string());

                    let ia_line = get_field_value(&account, &field_reader, keys.ia_line.to_owned())
                        .unwrap_or("NA".to_string());

                    let concat = get_field_value(&account, &field_reader, keys.concat.to_owned())
                        .unwrap_or("NA".to_string());

                    let division =
                        get_field_value(&account, &field_reader, keys.division.to_owned())
                            .unwrap_or("NA".to_string());

                    let npa_type =
                        get_field_value(&account, &field_reader, keys.npa_type.to_owned())
                            .unwrap_or("NA".to_string());

                    let raw_bm = get_field_value(&account, &field_reader, keys.raw_bm.to_owned())
                        .unwrap_or("NA".to_string());
                    let final_bm =
                        get_field_value(&account, &field_reader, keys.final_bm.to_owned())
                            .unwrap_or("NA".to_string());
                    let old_rt_flag = &data.rt_flag;
                    let new_rt_flag =
                        get_field_value(&account, &field_reader, keys.rt_flag.to_owned())
                            .unwrap_or("NA".to_string());

                    let trailing_fields = OpTrailingFields {
                        alm_line,
                        ia_line,
                        concat,
                        division,
                        npa_type,
                        raw_bm,
                        final_bm,
                        old_rt_flag: old_rt_flag.to_string(),
                        new_rt_flag,
                    };
                    //write to summary report
                    let mut summary_op: String = leading_fields.print().to_owned();
                    summary_op.push('\n');
                    write_data(&mut summary_writer, summary_op, logger);
                    summary_rows += 1;
                    //calculate derived fields for drilldown report. Append to output string.
                    let mut prev_cf_dt: Option<NaiveDate> = None;
                    let mut cashflows = account
                        .remove_cfs_for_key(&keys.cashflows)
                        .expect("Error while removing cashflow from the pool of cashflows.");
                    tot_cfs += cashflows.len();
                    for cf in cashflows.iter_mut() {
                        let prin_amount = cf.get_principal_amount();
                        let cf_date = naivedate_from_timestamp(cf.get_date());
                        //calculate no_of_days with initial value 30
                        let no_of_days: i64 = match prev_cf_dt {
                            Some(dt) => num_days_start_to_end(dt, cf_date),
                            None => 30,
                        };

                        //FTM impact
                        let ftm_impact = os_amt * int_diff / 100.0 * (no_of_days as f64) / 360.0;
                        prev_cf_dt = Some(cf_date);

                        //Residual Tenor
                        let res_tenure = num_days_start_to_end(as_on_date, cf_date);
                        //Residual tenor in years
                        let res_ten_yr = res_tenure as f64 / 365.0;
                        //Residual tenor impact
                        let res_ten_impact =
                            prin_amount * int_diff / 100.0 * (res_tenure as f64) / 360.0;
                        //Discounted Factor
                        let dis_pdt = 1.0 + &new_int_rt / 100.0;
                        let disc_factor = 1.0 / f64::powf(dis_pdt, res_ten_yr);
                        //Present value
                        let present_val = res_ten_impact * disc_factor;
                        let derived_fields_op = OpDrilldownReport {
                            ftm_impact,
                            residual_tenor: res_tenure,
                            residual_tenor_impact: res_ten_impact,
                            present_value: present_val,
                        };

                        //write to drilldown report
                        let mut drilldown_op: String = leading_fields.print().to_owned();
                        drilldown_op.push_str(&derived_fields_op.print());
                        drilldown_op.push_str(&trailing_fields.print());
                        let cf_date = format!("|{}\n", cf_date.format("%d-%m-%Y"));
                        drilldown_op.push_str(&cf_date);
                        write_data(&mut drilldown_writer, drilldown_op, logger);
                        drilldown_rows += 1;
                    }
                }
            }
        }
    }

    let timestamp = Local::now().naive_local().to_string();
    let footer_summary = format!(
        "FTR|{}|{}|{}\n",
        as_on_date.format("%d-%m-%Y"),
        timestamp,
        summary_rows
    );
    let footer_drilldown = format!(
        "FTR|{}|{}|{}\n",
        as_on_date.format("%d-%m-%Y"),
        timestamp,
        drilldown_rows
    );
    write_data(&mut summary_writer, footer_summary, logger);
    write_data(&mut drilldown_writer, footer_drilldown, logger);

    let health_report = HealthReport::new(
        acc_enc,
        acc_succ,
        acc_enc - acc_succ,
        tot_amt,
        tot_amt,
        tot_cfs as i64,
    );
    health_report.gen_health_rpt(&drilldown_report_path);

    pub fn write_data(writer: &mut BufWriter<File>, op: String, logger: &Logger) {
        let output_as_bytes = op.as_bytes();
        match writer.write(output_as_bytes) {
            Ok(_val) => {}
            Err(err) => {
                log_info!(logger, "Error writing to output file. Error: {}", err);
            }
        }
    }

    fn naivedate_from_timestamp(t: i64) -> NaiveDate {
        let naive_date_time = rbdate::NaiveDateTime::from_timestamp(t, 0);
        naive_date_time.date()
    }
}
