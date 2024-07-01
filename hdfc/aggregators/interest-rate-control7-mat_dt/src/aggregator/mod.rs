use aggregator::account_field_names::AccFieldNames;
use aggregator::structs::{OpDrilldownReport, OpLeadingFields, OpTrailingFields};
use chrono::Local;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use rbdate::{decr_dt_by_mon_presrv_eom, num_days_start_to_end, NaiveDate};
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

struct Data {
    mat_dt: String,
    benchmark: String,
    rate_type: String,
    bm_rate: f64,
    int_rate: f64,
    rate_flag: String,
    cf_date: Vec<NaiveDate>,
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
    let summary_header ="Account Number|Source system|Customer Id|Customer Name|Product code|Scheme ID|Booking Date|Value Date|Maturity Date – Original|Maturity Date - New|MIS 1|MIS 2|MIS 3|CCY|Original Amount|O/s amount|Old BM|New BM|Old Rate Type|New Rate Type|Old BM Rate|New BM Rate|Old Interest rate|New Interest rate |Old Spread|New Spread|ALM line|IA Line|Concat|Division|NPA Type|Raw BM|Final BM|Old Rate Flag|New Rate Flag\n";
    let drilldown_header ="Account Number|Source system|Customer Id|Customer Name|Product code|Scheme ID|Booking Date|Value Date|Maturity Date – Original|Maturity Date - New|MIS 1|MIS 2|MIS 3|CCY|Original Amount|O/s amount|Old BM|New BM|Old Rate Type|New Rate Type|Old BM Rate|New BM Rate|Old Interest rate|New Interest rate |Old Spread|New Spread|Residual Tenor-Old|Residual Tenor- New|Residual Tenor Impact|Present Value|ALM line|IA Line|Concat|Division|NPA Type|Raw BM|Final BM|Old Rate Flag|New Rate Flag|Cash Flow Date\n";
    write_data(&mut summary_writer, summary_header.to_string(), logger);
    write_data(&mut drilldown_writer, drilldown_header.to_string(), logger);

    for file in files_config.input_files {
        let keys = AccFieldNames::new_from_path(&file.req_fields_file_path);
        let mut prev_mth_reader =
            reader::Reader::new_at_path(&file.account_metadata_file_path, &file.prev_mth_src_file);

        let ason: NaiveDate = NaiveDate::parse_from_str(&config_params.as_on_date(), "%Y%m%d")
            .expect("Could not parse as on date");
        let prev_ason =
            decr_dt_by_mon_presrv_eom(ason, 1).expect("Could not parse previous month as on date");

        let mut prev_mth_map: HashMap<String, Data> = HashMap::new();
        for mut account in prev_mth_reader.iter() {
            let acc_no = account
                .get_string_for_key(&keys.acc_no)
                .unwrap_or(&"NA".to_string())
                .to_string();

            let maturity_dt = account.get_i64_for_key(&keys.mat_dt).unwrap_or(0);
            let maturity_dt = naivedate_from_timestamp(maturity_dt)
                .format("%d-%m-%Y")
                .to_string();
            let old_benchmark = account
                .get_string_for_key(&keys.benchmark)
                .unwrap_or(&"NA".to_string())
                .to_string();
            let old_rate_type = account
                .get_string_for_key(&keys.rt_type)
                .unwrap_or(&"NA".to_string())
                .to_string();
            let bm_rate = account.get_f64_for_key(&keys.bm_rate).unwrap_or(0.0);
            let old_int_rt = account.get_f64_for_key(&keys.int_rt).unwrap_or(0.0);

            let rt_flag = account
                .get_string_for_key(&keys.rate_flag)
                .unwrap_or(&"NA".to_string())
                .to_string();

            let mut old_cf_dates = Vec::new();
            for cf in account
                .remove_cfs_for_key(&keys.cashflows)
                .expect("Error while removing cashflow from the pool of cashflows.")
                .iter_mut()
            {
                let cf_date = naivedate_from_timestamp(cf.get_date());
                old_cf_dates.push(cf_date);
            }
            prev_mth_map.insert(
                acc_no,
                Data {
                    mat_dt: maturity_dt,
                    benchmark: old_benchmark,
                    rate_type: old_rate_type,
                    bm_rate,
                    int_rate: old_int_rt,
                    rate_flag: rt_flag,
                    cf_date: old_cf_dates,
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
                .unwrap_or(&"NA".to_string())
                .to_string();
            if let Some(data) = prev_mth_map.get(&acc_no) {
                let org_mat_dt = &data.mat_dt;
                let new_mat_dt = account.get_i64_for_key(&keys.mat_dt).unwrap_or(0);
                let new_mat_dt = naivedate_from_timestamp(new_mat_dt)
                    .format("%d-%m-%Y")
                    .to_string();
                if org_mat_dt.ne(&new_mat_dt) {
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
                    let booking_dt = account.get_i64_for_key(&keys.booking_dt).unwrap_or(0);
                    let booking_dt = naivedate_from_timestamp(booking_dt)
                        .format("%d-%m-%Y")
                        .to_string();
                    let value_dt = account.get_i64_for_key(&keys.value_dt).unwrap_or(0);
                    let value_dt = naivedate_from_timestamp(value_dt)
                        .format("%d-%m-%Y")
                        .to_string();
                    let ccy = account
                        .get_string_for_key(&keys.ccy)
                        .unwrap_or(&"NA".to_string())
                        .to_string();
                    let mis1 = account
                        .get_string_for_key(&keys.mis1)
                        .unwrap_or(&"NA".to_string())
                        .to_string();
                    let mis2 = account
                        .get_string_for_key(&keys.mis2)
                        .unwrap_or(&"NA".to_string())
                        .to_string();
                    let mis3 = account
                        .get_string_for_key(&keys.mis3)
                        .unwrap_or(&"NA".to_string())
                        .to_string();
                    let org_amt = account
                        .get_f64_for_key(&keys.org_amt)
                        .unwrap_or(0.00)
                        .to_string();
                    let os_amt = account.get_f64_for_key(&keys.cur_os_amt).unwrap_or(0.00);
                    let old_benchmark = &data.benchmark;
                    let benchmark = account
                        .get_string_for_key(&keys.benchmark)
                        .unwrap_or(&"NA".to_string())
                        .to_string();
                    let old_rt_type = &data.rate_type;
                    let new_rt_type = account
                        .get_string_for_key(&keys.rt_type)
                        .unwrap_or(&"NA".to_string())
                        .to_string();
                    let old_bm_rt = &data.bm_rate;
                    let new_bm_rt = account.get_f64_for_key(&keys.bm_rate).unwrap_or(0.0);
                    let old_int_rt = &data.int_rate;
                    let cur_int_rt = account.get_f64_for_key(&keys.int_rt).unwrap_or(0.0);
                    let spread_old = old_bm_rt - old_int_rt;
                    let spread_new = new_bm_rt - cur_int_rt;

                    let leading_fields = OpLeadingFields {
                        account_number: acc_no,
                        source_system: file.source_system.to_string(),
                        customer_id: cust_id,
                        customer_name: cust_name,
                        product_code: pdt_code,
                        scheme_id,
                        booking_date: booking_dt,
                        value_date: value_dt,
                        maturity_dt_org: org_mat_dt.to_string(),
                        maturity_dt_new: new_mat_dt,
                        mis1,
                        mis2,
                        mis3,
                        ccy,
                        original_amount: org_amt.to_string(),
                        current_os_amount: os_amt.to_string(),
                        old_bm: old_benchmark.to_string(),
                        new_bm: benchmark,
                        old_rate_type: old_rt_type.to_string(),
                        new_rate_type: new_rt_type,
                        old_bm_rate: old_bm_rt.to_string(),
                        new_bm_rate: new_bm_rt.to_string(),
                        int_rt_prev_mth: old_int_rt.to_string(),
                        int_rt_cur_mth: cur_int_rt.to_string(),
                        spread_old: spread_old.to_string(),
                        spread_new: spread_new.to_string(),
                    };

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

                    let division = account
                        .get_string_for_key(&keys.division)
                        .unwrap_or(&"NA".to_string())
                        .to_string();

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
                    let new_rt_flag = account
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
                        new_rt_flag,
                    };
                    //Write to summary report
                    let mut summary_op: String = leading_fields.print().to_owned();
                    summary_op.push_str(&trailing_fields.print());
                    summary_op.push('\n');
                    write_data(&mut summary_writer, summary_op, logger);
                    summary_rows += 1;
                    let old_cf_dates = &data.cf_date;
                    //Iterator for cashflow dates of previous month file.
                    let mut cf_date_iter = old_cf_dates.iter();
                    let mut cashflows = account
                        .remove_cfs_for_key(&keys.cashflows)
                        .expect("Error while removing cashflow from the pool of cashflows.");
                    tot_cfs += cashflows.len();
                    for cf in cashflows.iter_mut() {
                        let prin_amount = cf.get_principal_amount();
                        let cf_date = naivedate_from_timestamp(cf.get_date());
                        //Residual Tenor
                        let res_tenor_old = match cf_date_iter.next() {
                            Some(date) => num_days_start_to_end(prev_ason, *date),
                            None => 0,
                        };
                        let res_tenor_new = num_days_start_to_end(ason, cf_date);
                        //Residual tenor in years
                        let res_ten_yr = res_tenor_new as f64 / 365.0;
                        //Residual tenor impact
                        let res_ten_impact =
                            prin_amount * spread_new / 100.0 * (res_tenor_new as f64) / 360.0;
                        //Discounted Factor
                        let dis_pdt = 1.0 + &cur_int_rt / 100.0;
                        let disc_factor = 1.0 / f64::powf(dis_pdt, res_ten_yr as f64);
                        //Present value
                        let present_val = res_ten_impact * disc_factor;

                        let derived_fields_op = OpDrilldownReport {
                            res_tenor_old,
                            res_tenor_new,
                            res_tenor_impact: res_ten_impact,
                            present_val,
                        };
                        //Write to drilldown report.
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
    let as_on_date: NaiveDate = NaiveDate::parse_from_str(&config_params.as_on_date(), "%Y%m%d")
        .expect("Could not parse as on date");
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
