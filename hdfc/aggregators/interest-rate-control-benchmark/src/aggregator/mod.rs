use aggregator::account_field_names::AccFieldNames;
use aggregator::structs::{OpDrilldownReport, OpLeadingFields, OpTrailingFields};
use chrono::Local;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::{num_days_start_to_end, NaiveDate};
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::types::Type;
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

pub struct Data {
    pub int_rate: f64,
    pub rate_flag: String,
    pub benchmark: String,
    pub bm_spread: f64,
    pub os_amt: f64,
    pub rt_type: String,
}

pub fn aggregate_cashflows(
    config_params: ConfigurationParameters,
    logger: &Logger,
    diag_logger: &Logger,
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

    let summary_header ="Account Number|Source system|Customer Id|Customer Name|Product code|Scheme ID|Booking Date|Validity Date|Maturity Date|CCY|MIS 1|MIS 2|MIS 3|Source GL|Rate Type- Previous month|Rate Type- Current month|Original Amount|Previous o/s amount|Current o/s amount|Interest rate previous month|Old benchmark|New benchmark|Old BM Spread|New BM Spread|Old BM Rate|New BM Rate|Interest Rate current month|Interest rate differential|Last Reset Date|Next Reset Date|ALM Line|IA Line|Concat|Division|NPA Type|Raw BM|Final BM|Old Rate Flag|New Rate Flag\n";
    let drilldown_header = "Account Number|Source system|Customer Id|Customer Name|Product code|Scheme ID|Booking Date|Validity Date|Maturity Date|CCY|MIS 1|MIS 2|MIS 3|Source GL|Rate Type- Previous month|Rate Type- Current month|Original Amount|Previous o/s amount|Current o/s amount|Interest rate previous month|Old benchmark|New benchmark|Old BM Spread|New BM Spread|Old BM Rate|New BM Rate|Interest Rate current month|Interest rate differential|Last Reset Date|Next Reset Date|FTM impact|Residual Tenor|Residual Tenor Impact|Present Value|ALM Line|IA Line|Concat|Division|NPA Type|Raw BM|Final BM|Old Rate Flag|New Rate Flag|Cash Flow Date\n";
    write_data(&mut summary_writer, summary_header.to_string(), logger);
    write_data(&mut drilldown_writer, drilldown_header.to_string(), logger);

    for file in files_config.input_files {
        let keys = AccFieldNames::new_from_path(&file.req_fields_file_path);
        let mut prev_mth_reader =
            reader::Reader::new_at_path(&file.account_metadata_file_path, &file.prev_mth_src_file);

        let mut prev_mth_map: HashMap<String, Data> = HashMap::new();
        for account in prev_mth_reader.iter() {
            let acc_no = account
                .get_string_for_key(&keys.acc_no)
                .unwrap_or(&"acc_no".to_string())
                .to_string();
            let int_rate = account.get_f64_for_key(&keys.int_rt).unwrap_or(0.0);
            let rate_flag = account
                .get_string_for_key(&keys.rate_flag)
                .unwrap_or(&"NA".to_string())
                .to_string();
            let benchmark = account
                .get_string_for_key(&keys.benchmark)
                .unwrap_or(&"NA".to_string())
                .to_string();
            let bm_spread = account.get_f64_for_key(&keys.bm_spread).unwrap_or(0.0);
            let os_amt = account.get_f64_for_key(&keys.cur_os_amt).unwrap_or(0.0);
            let rt_type = account
                .get_string_for_key(&keys.rt_type)
                .unwrap_or(&"NA".to_string())
                .to_string();
            prev_mth_map.insert(
                acc_no,
                Data {
                    int_rate,
                    rate_flag,
                    benchmark,
                    bm_spread,
                    os_amt,
                    rt_type,
                },
            );
        }

        let mut cur_mth_reader =
            reader::Reader::new_at_path(&file.account_metadata_file_path, &file.cur_mth_src_file);
        let field_reader =
            reader::Reader::new_at_path(&file.account_metadata_file_path, &file.cur_mth_src_file);

        for mut account in cur_mth_reader.iter() {
            acc_enc += 1;
            let acc_no = account
                .get_string_for_key(&keys.acc_no)
                .unwrap_or(&"acc_no".to_string())
                .to_string();
            if let Some(data) = prev_mth_map.get(&acc_no) {
                let old_benchmark = &data.benchmark;
                let new_benchmark = account
                    .get_string_for_key(&keys.benchmark)
                    .unwrap_or(&"NA".to_string())
                    .to_string();

                if old_benchmark.ne(&new_benchmark) {
                    acc_succ += 1;
                    let cust_id = match field_reader.get_field_type(&keys.cust_id) {
                        Some(typ) => match typ {
                            Type::I64 => account
                                .get_i64_for_key(&keys.cust_id)
                                .unwrap_or(0)
                                .to_string(),
                            Type::F64 => account
                                .get_f64_for_key(&keys.cust_id)
                                .unwrap_or(0.0)
                                .to_string(),
                            Type::String => account
                                .get_string_for_key(&keys.cust_id)
                                .unwrap_or(&"NA".to_string())
                                .to_string(),
                            _ => "NA".to_string(),
                        },
                        None => "NA".to_string(),
                    };
                    let cust_name = account
                        .get_string_for_key(&keys.cust_name)
                        .unwrap_or(&"NA".to_string())
                        .to_string();

                    let pdt_code = match field_reader.get_field_type(&keys.pdt_code) {
                        Some(typ) => match typ {
                            Type::I64 => account
                                .get_i64_for_key(&keys.pdt_code)
                                .unwrap_or(0)
                                .to_string(),
                            Type::F64 => account
                                .get_f64_for_key(&keys.pdt_code)
                                .unwrap_or(0.0)
                                .to_string(),
                            Type::String => account
                                .get_string_for_key(&keys.pdt_code)
                                .unwrap_or(&"NA".to_string())
                                .to_string(),
                            _ => "NA".to_string(),
                        },
                        None => "NA".to_string(),
                    };
                    let scheme_id = match field_reader.get_field_type(&keys.scheme_id) {
                        Some(typ) => match typ {
                            Type::I64 => account
                                .get_i64_for_key(&keys.scheme_id)
                                .unwrap_or(0)
                                .to_string(),
                            Type::F64 => account
                                .get_f64_for_key(&keys.scheme_id)
                                .unwrap_or(0.0)
                                .to_string(),
                            Type::String => account
                                .get_string_for_key(&keys.scheme_id)
                                .unwrap_or(&"NA".to_string())
                                .to_string(),
                            _ => "NA".to_string(),
                        },
                        None => "NA".to_string(),
                    };
                    let booking_dt = match account.get_i64_for_key(&keys.booking_dt).unwrap_or(0) {
                        0 => "NA".to_string(),
                        dt => naivedate_from_timestamp(dt).format("%d-%m-%Y").to_string(),
                    };
                    let validity_dt = match account.get_i64_for_key(&keys.validity_dt).unwrap_or(0)
                    {
                        0 => "NA".to_string(),
                        dt => naivedate_from_timestamp(dt).format("%d-%m-%Y").to_string(),
                    };
                    let mat_dt = match account.get_i64_for_key(&keys.mat_dt).unwrap_or(0) {
                        0 => "NA".to_string(),
                        dt => naivedate_from_timestamp(dt).format("%d-%m-%Y").to_string(),
                    };
                    let ccy = account
                        .get_string_for_key(&keys.ccy)
                        .unwrap_or(&"NA".to_string())
                        .to_string();
                    let mis1 = match field_reader.get_field_type(&keys.mis1) {
                        Some(typ) => match typ {
                            Type::I64 => {
                                account.get_i64_for_key(&keys.mis1).unwrap_or(0).to_string()
                            }
                            Type::F64 => account
                                .get_f64_for_key(&keys.mis1)
                                .unwrap_or(0.0)
                                .to_string(),
                            Type::String => account
                                .get_string_for_key(&keys.mis1)
                                .unwrap_or(&"NA".to_string())
                                .to_string(),
                            _ => "NA".to_string(),
                        },
                        None => "NA".to_string(),
                    };
                    let mis2 = match field_reader.get_field_type(&keys.mis2) {
                        Some(typ) => match typ {
                            Type::I64 => {
                                account.get_i64_for_key(&keys.mis2).unwrap_or(0).to_string()
                            }
                            Type::F64 => account
                                .get_f64_for_key(&keys.mis2)
                                .unwrap_or(0.0)
                                .to_string(),
                            Type::String => account
                                .get_string_for_key(&keys.mis2)
                                .unwrap_or(&"NA".to_string())
                                .to_string(),
                            _ => "NA".to_string(),
                        },
                        None => "NA".to_string(),
                    };
                    let mis3 = match field_reader.get_field_type(&keys.mis3) {
                        Some(typ) => match typ {
                            Type::I64 => {
                                account.get_i64_for_key(&keys.mis3).unwrap_or(0).to_string()
                            }
                            Type::F64 => account
                                .get_f64_for_key(&keys.mis3)
                                .unwrap_or(0.0)
                                .to_string(),
                            Type::String => account
                                .get_string_for_key(&keys.mis3)
                                .unwrap_or(&"NA".to_string())
                                .to_string(),
                            _ => "NA".to_string(),
                        },
                        None => "NA".to_string(),
                    };
                    let source_gl = match field_reader.get_field_type(&keys.source_gl) {
                        Some(typ) => match typ {
                            Type::I64 => account
                                .get_i64_for_key(&keys.source_gl)
                                .unwrap_or(0)
                                .to_string(),
                            Type::F64 => account
                                .get_f64_for_key(&keys.source_gl)
                                .unwrap_or(0.0)
                                .to_string(),
                            Type::String => account
                                .get_string_for_key(&keys.source_gl)
                                .unwrap_or(&"NA".to_string())
                                .to_string(),
                            _ => "NA".to_string(),
                        },
                        None => "NA".to_string(),
                    };
                    let prev_rt_type = &data.rt_type;
                    let cur_rt_type = account
                        .get_string_for_key(&keys.rt_type)
                        .unwrap_or(&"NA".to_string())
                        .to_string();
                    let org_amt = account
                        .get_f64_for_key(&keys.org_amt)
                        .unwrap_or(0.00)
                        .to_string();
                    let prev_os_amt = &data.os_amt.to_string();
                    let cur_os_amt = account.get_f64_for_key(&keys.cur_os_amt).unwrap_or(0.00);
                    let prev_int_rt = &data.int_rate;
                    let cur_int_rt = account.get_f64_for_key(&keys.int_rt).unwrap_or(0.0);
                    let int_diff = cur_int_rt - prev_int_rt;

                    let old_bm_spread = &data.bm_spread;
                    let old_bm_rt = prev_int_rt - old_bm_spread;
                    let new_bm_spread = account.get_f64_for_key(&keys.bm_spread).unwrap_or(0.0);
                    let new_bm_rt = cur_int_rt - new_bm_spread;
                    let last_reset_dt =
                        match account.get_i64_for_key(&keys.last_reset_dt).unwrap_or(0) {
                            0 => "NA".to_string(),
                            dt => naivedate_from_timestamp(dt).format("%d-%m-%Y").to_string(),
                        };
                    let next_reset_dt =
                        match account.get_i64_for_key(&keys.next_reset_dt).unwrap_or(0) {
                            0 => "NA".to_string(),
                            dt => naivedate_from_timestamp(dt).format("%d-%m-%Y").to_string(),
                        };

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
                        ccy,
                        mis1,
                        mis2,
                        mis3,
                        source_gl,
                        prev_rate_type: prev_rt_type.to_string(),
                        cur_rate_type: cur_rt_type,
                        original_amount: org_amt,
                        previous_os_amount: prev_os_amt.to_string(),
                        current_os_amount: cur_os_amt.to_string(),
                        int_rt_prev_mth: prev_int_rt.to_string(),
                        old_benchmark: old_benchmark.to_string(),
                        new_benchmark,
                        old_bm_spread: old_bm_spread.to_string(),
                        new_bm_spread: new_bm_spread.to_string(),
                        old_bm_rate: old_bm_rt.to_string(),
                        new_bm_rate: new_bm_rt.to_string(),
                        int_rt_cur_mth: cur_int_rt.to_string(),
                        int_rate_diff: int_diff.to_string(),
                        last_reset_dt,
                        next_reset_dt,
                    };

                    //Append trailing fields to the reports.
                    let alm_line = account
                        .get_string_for_key(&keys.alm_line)
                        .unwrap_or(&"NA".to_string())
                        .to_string();

                    let ia_line = account
                        .get_string_for_key(&keys.ia_line)
                        .unwrap_or(&"NA".to_string())
                        .to_string();

                    let concat = match field_reader.get_field_type(&keys.concat) {
                        Some(typ) => match typ {
                            Type::I64 => account
                                .get_i64_for_key(&keys.concat)
                                .unwrap_or(0)
                                .to_string(),
                            Type::F64 => account
                                .get_f64_for_key(&keys.concat)
                                .unwrap_or(0.0)
                                .to_string(),
                            Type::String => account
                                .get_string_for_key(&keys.concat)
                                .unwrap_or(&"NA".to_string())
                                .to_string(),
                            _ => "NA".to_string(),
                        },
                        None => "NA".to_string(),
                    };

                    let division = match field_reader.get_field_type(&keys.division) {
                        Some(typ) => match typ {
                            Type::I64 => account
                                .get_i64_for_key(&keys.division)
                                .unwrap_or(0)
                                .to_string(),
                            Type::F64 => account
                                .get_f64_for_key(&keys.division)
                                .unwrap_or(0.0)
                                .to_string(),
                            Type::String => account
                                .get_string_for_key(&keys.division)
                                .unwrap_or(&"NA".to_string())
                                .to_string(),
                            _ => "NA".to_string(),
                        },
                        None => "NA".to_string(),
                    };
                    let npa_type = account
                        .get_string_for_key(&keys.npa_type)
                        .unwrap_or(&"NA".to_string())
                        .to_string();

                    let raw_bm = match field_reader.get_field_type(&keys.raw_bm) {
                        Some(typ) => match typ {
                            Type::I64 => account
                                .get_i64_for_key(&keys.raw_bm)
                                .unwrap_or(0)
                                .to_string(),
                            Type::F64 => account
                                .get_f64_for_key(&keys.raw_bm)
                                .unwrap_or(0.0)
                                .to_string(),
                            Type::String => account
                                .get_string_for_key(&keys.raw_bm)
                                .unwrap_or(&"NA".to_string())
                                .to_string(),
                            _ => "NA".to_string(),
                        },
                        None => "NA".to_string(),
                    };

                    let final_bm = match field_reader.get_field_type(&keys.final_bm) {
                        Some(typ) => match typ {
                            Type::I64 => account
                                .get_i64_for_key(&keys.final_bm)
                                .unwrap_or(0)
                                .to_string(),
                            Type::F64 => account
                                .get_f64_for_key(&keys.final_bm)
                                .unwrap_or(0.0)
                                .to_string(),
                            Type::String => account
                                .get_string_for_key(&keys.final_bm)
                                .unwrap_or(&"NA".to_string())
                                .to_string(),
                            _ => "NA".to_string(),
                        },
                        None => "NA".to_string(),
                    };

                    let old_rate_flag = &data.rate_flag;
                    let new_rate_flag = account
                        .get_string_for_key(&keys.rate_flag)
                        .unwrap_or(&"NA".to_string())
                        .to_string();

                    let trailing_fields = OpTrailingFields {
                        alm_line,
                        ia_line,
                        concat,
                        division,
                        npa_type,
                        raw_bm,
                        final_bm,
                        old_rt_flag: old_rate_flag.to_string(),
                        new_rt_flag: new_rate_flag,
                    };

                    //write to summary report
                    let mut summary_op: String = leading_fields.print().to_owned();
                    summary_op.push_str(&trailing_fields.print());
                    summary_op.push('\n');
                    write_data(&mut summary_writer, summary_op, logger);
                    summary_rows += 1;
                    //calculate derived fields for drilldown report.
                    let mut prev_cf_dt: Option<NaiveDate> = None;

                    let mut cashflows = account
                        .remove_cfs_for_key(&keys.cashflows)
                        .expect("Error while removing cashflow from the pool of cashflows.");
                    tot_cfs += cashflows.len();
                    for cf in cashflows.iter_mut() {
                        let prin_amount = cf.get_principal_amount();
                        let cf_date = naivedate_from_timestamp(cf.get_date());
                        //Calculate no_of_days with default value 30.
                        let no_of_days: i64 = match prev_cf_dt {
                            Some(dt) => num_days_start_to_end(dt, cf_date),
                            None => 30,
                        };
                        //FTM impact
                        let ftm_impact =
                            cur_os_amt * int_diff / 100.0 * (no_of_days as f64) / 360.0;
                        prev_cf_dt = Some(cf_date);

                        //Residual Tenor
                        let res_tenure = num_days_start_to_end(as_on_date, cf_date);
                        //Residual tenor in years
                        let res_ten_yr = res_tenure as f64 / 365.0;
                        //Residual tenor impact
                        let res_ten_impact =
                            prin_amount * int_diff / 100.0 * (res_tenure as f64) / 360.0;
                        //Discounted Factor
                        let dis_pdt = 1.0 + &cur_int_rt / 100.0;
                        let disc_factor = 1.0 / f64::powf(dis_pdt, res_ten_yr as f64);
                        //Present value
                        let present_val = res_ten_impact * disc_factor;
                        let derived_fields_op = OpDrilldownReport {
                            ftm_impact,
                            res_tenor: res_tenure,
                            res_ten_impact,
                            present_val: present_val.to_string(),
                        };
                        //write to drilldown report
                        let mut drilldown_op: String = leading_fields.print().to_owned();
                        drilldown_op.push_str(&derived_fields_op.print());
                        drilldown_op.push_str(&trailing_fields.print());
                        let cf_date = format!("|{}\n", cf_date.format("%d-%m-%Y").to_string());
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

    pub fn naivedate_from_timestamp(t: i64) -> NaiveDate {
        let naive_date_time = rbdate::NaiveDateTime::from_timestamp(t, 0);
        naive_date_time.date()
    }
}