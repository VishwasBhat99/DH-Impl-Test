use self::io::*;
use configuration_parameters::ConfigurationParameters;
use macros;
use process::get_fields::*;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::hash::Hash;
use std::time::SystemTime;
mod get_fields;
mod io;
mod structs;
use self::structs::*;
use health_report::HealthReport;
use rbdate::DateParser;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;

pub fn process(config_params: &ConfigurationParameters, log: &Logger, _diag_logger: &Logger) {
    let mut op_writer = get_writer(config_params.output_file_path());
    let date_parser: DateParser = DateParser::new("%d-%m-%Y".to_string(), true);
    //let as_on_date = date_parser.parse(config_params.as_on_date());
    let mut tot_records_read = 0;
    let mut tot_records_success = 0;

    let lam_file = match new_buf_rdr(config_params.input_file_lam()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` : {}.",
            config_params.input_file_lam(),
            error
        ),
    };
    let od_file = match new_buf_rdr(config_params.input_file_od_int()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` : {}.",
            config_params.input_file_od_int(),
            error
        ),
    };
    let mut tbl_code_file = match new_buf_rdr(config_params.tbl_code_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` : {}.",
            config_params.tbl_code_file_path(),
            error
        ),
    };
    let int_file = match new_buf_rdr(config_params.input_file_int_rate()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` : {}.",
            config_params.input_file_int_rate(),
            error
        ),
    };
    let lrs_file = match new_buf_rdr(config_params.input_file_lrs()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` : {}.",
            config_params.input_file_lrs(),
            error
        ),
    };
    let lsp_file = match new_buf_rdr(config_params.input_file_lsp()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` : {}.",
            config_params.input_file_lsp(),
            error
        ),
    };
    let benchmark_file = match new_buf_rdr(config_params.input_file_benchmark()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` : {}.",
            config_params.input_file_benchmark(),
            error
        ),
    };
    let npa_file = match new_buf_rdr(config_params.input_file_npa()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` : {}.",
            config_params.input_file_npa(),
            error
        ),
    };
    let gam_input = File::open(&config_params.input_file_gam()).expect("Could Not Read File GAM.");
    //TBL_CODES
    let mut tblcodes_set: HashSet<String> = HashSet::new();
    for (line_num, lines) in tbl_code_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.tbl_code_file_path(),
                line_num + 1,
                error
            ),
        };
        tblcodes_set.insert(line.trim().to_string());
    }
    //LAM
    let start_lam_read_timer = SystemTime::now();
    let mut lam_map: HashMap<String, LamData> = HashMap::new();
    for (line_num, lines) in lam_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.input_file_lam(),
                line_num + 1,
                error
            ),
        };
        let input_fields: Vec<&str> = line.split('|').collect();
        let lam_value = match config_params.finacle_finlite_flg() {
            "FINACLE" => LamData {
                dis_amt: input_fields[6].trim().parse::<f64>().unwrap_or(0.0),
                dis_shdl_date: date_parser.parse(input_fields[5].trim()),
                dis_shdl_num: input_fields[4].trim().to_string(),
                rep_shdl_date: date_parser.parse(input_fields[13].trim()),
                rep_shdl_num: input_fields[3].trim().parse::<i64>().unwrap_or(0),
                rephasement_principal: input_fields[14].trim().to_string(),
                ei_schm_flg: input_fields[18].trim().to_string(),
            },
            _ => LamData {
                dis_amt: input_fields[6].trim().parse::<f64>().unwrap_or(0.0),
                dis_shdl_date: date_parser.parse(input_fields[5].trim()),
                dis_shdl_num: input_fields[4].trim().to_string(),
                rep_shdl_date: date_parser.parse(input_fields[13].trim()),
                rep_shdl_num: input_fields[3].trim().parse::<i64>().unwrap_or(0),
                rephasement_principal: input_fields[14].trim().to_string(),
                ei_schm_flg: "NA".to_string(),
            },
        };
        lam_map.insert(input_fields[0].trim().to_string(), lam_value);
    }
    let end_lam_read_timer = SystemTime::now();
    let duration = end_lam_read_timer
        .duration_since(start_lam_read_timer)
        .expect("Could not calculate total duration for LAM read timer.");
    debug!(log, "Readings LAM File, Total Duration: {:?}.", duration);

    //OD INT
    let start_od_read_timer = SystemTime::now();
    let mut od_int_map: HashMap<String, String> = HashMap::new();
    for (line_num, lines) in od_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.input_file_od_int(),
                line_num + 1,
                error
            ),
        };
        let input_fields: Vec<&str> = line.split('|').collect();
        od_int_map.insert(
            input_fields[0].trim().to_string(),
            input_fields[5].trim().to_string(),
        );
    }
    let end_od_read_timer = SystemTime::now();
    let duration = end_od_read_timer
        .duration_since(start_od_read_timer)
        .expect("Could not calculate total duration for OD Int read timer.");
    debug!(log, "Readings OD Int File, Total Duration: {:?}.", duration);

    //LOAN INT
    let start_int_read_timer = SystemTime::now();
    let mut der_pegged_map: HashMap<String, String> = HashMap::new();
    let mut loan_int_map: HashMap<String, IntRateData> = HashMap::new();
    for (line_num, lines) in int_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.input_file_int_rate(),
                line_num + 1,
                error
            ),
        };
        let input_fields: Vec<&str> = line.split('|').collect();
        let interest_sum = input_fields[13].trim().parse::<f64>().unwrap_or(0.0)
            + input_fields[11].trim().parse::<f64>().unwrap_or(0.0)
            + input_fields[12].trim().parse::<f64>().unwrap_or(0.0)
            + input_fields[15].trim().parse::<f64>().unwrap_or(0.0);
        let pegged_flg = match od_int_map.get(input_fields[0].trim()) {
            Some(_data) => "N1".to_string(),
            None => input_fields[19].trim().to_string(),
        };
        der_pegged_map.insert(
            input_fields[0].trim().to_string(),
            input_fields[19].trim().to_string(),
        );
        let int_tbl_code = input_fields[5].trim().to_string();
        let int_value = IntRateData {
            interest_rate: interest_sum,
            end_date: match input_fields[18].trim() {
                "Null" => date_parser.parse("31-12-2099"),
                _ => date_parser.parse(input_fields[18].trim()),
            },
            pegged_flg,
            int_tbl_code,
        };

        loan_int_map.insert(input_fields[0].trim().to_string(), int_value);
    }
    drop(od_int_map);

    let end_int_read_timer = SystemTime::now();
    let duration = end_int_read_timer
        .duration_since(start_int_read_timer)
        .expect("Could not calculate total duration for Loan Int read timer.");
    debug!(
        log,
        "Readings Loan Int File, Total Duration: {:?}.", duration
    );

    //LRS
    let start_lrs_read_timer = SystemTime::now();
    let mut lrs_map: HashMap<String, Vec<LrsData>> = HashMap::new();
    for (line_num, lines) in lrs_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.input_file_lrs(),
                line_num + 1,
                error
            ),
        };
        let input_fields: Vec<&str> = line.split('|').collect();
        if input_fields[13].trim() == "EIDEM" || input_fields[13].trim() == "PRDEM" {
            lrs_map
                .entry(input_fields[0].trim().to_string())
                .and_modify(|val| {
                    val.push(LrsData {
                        num_of_flows: input_fields[1].trim().parse::<f64>().unwrap_or(0.0),
                        flow_start_date: date_parser.parse(input_fields[3].trim()),
                        flow_amt: input_fields[4].trim().parse::<f64>().unwrap_or(0.0),
                        lr_freq_type: input_fields[5].trim().to_string(),
                        num_of_dmds: input_fields[2].trim().parse::<f64>().unwrap_or(0.0),
                        cashflow_code: input_fields[13].trim().to_string(),
                    })
                })
                .or_insert_with(|| {
                    vec![LrsData {
                        num_of_flows: input_fields[1].trim().parse::<f64>().unwrap_or(0.0),
                        flow_start_date: date_parser.parse(input_fields[3].trim()),
                        flow_amt: input_fields[4].trim().parse::<f64>().unwrap_or(0.0),
                        lr_freq_type: input_fields[5].trim().to_string(),
                        num_of_dmds: input_fields[2].trim().parse::<f64>().unwrap_or(0.0),
                        cashflow_code: input_fields[13].trim().to_string(),
                    }]
                });
        }
    }

    let end_lrs_read_timer = SystemTime::now();
    let duration = end_lrs_read_timer
        .duration_since(start_lrs_read_timer)
        .expect("Could not calculate total duration for lrs read timer.");
    debug!(log, "Readings LRS File, Total Duration: {:?}.", duration);

    // LSP
    let start_lsp_read_timer = SystemTime::now();
    let mut lsp_map: HashMap<String, String> = HashMap::new();

    for (line_num, lines) in lsp_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.input_file_lsp(),
                line_num + 1,
                error
            ),
        };
        let input_fields: Vec<&str> = line.split('|').collect();
        lsp_map.insert(
            input_fields[0].trim().to_string(),
            input_fields[1].trim().to_string(),
        );
    }
    let end_lsp_read_timer = SystemTime::now();
    let duration = end_lsp_read_timer
        .duration_since(start_lsp_read_timer)
        .expect("Could not calculate total duration for lsp read timer.");
    debug!(log, "Readings lsp File, Total Duration: {:?}.", duration);

    //BENCHMARK
    let start_benchmark_read_timer = SystemTime::now();
    let mut benchmark_map: HashMap<String, BenchmarkData> = HashMap::new();
    for (line_num, lines) in benchmark_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.input_file_benchmark(),
                line_num + 1,
                error
            ),
        };
        let input_fields: Vec<&str> = line.split('|').collect();
        //Lookup is foracid.
        benchmark_map.insert(
            input_fields[1].to_string(),
            BenchmarkData {
                floating_type: input_fields[3].to_string(),
                repricing_plan: input_fields[2].to_string(),
                peg_review_date: date_parser.parse(input_fields[4]),
            },
        );
    }

    let end_benchmark_read_timer = SystemTime::now();
    let duration = end_benchmark_read_timer
        .duration_since(start_benchmark_read_timer)
        .expect("Could not calculate total duration for benchmark read timer.");
    debug!(
        log,
        "Readings benchmark File, Total Duration: {:?}.", duration
    );

    //NPA
    let start_npa_read_timer = SystemTime::now();
    let mut npa_map: HashMap<String, NpaData> = HashMap::new();

    for (line_num, lines) in npa_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.input_file_npa(),
                line_num + 1,
                error
            ),
        };
        let input_fields: Vec<&str> = line.split('|').collect();
        //Lookup is foracid.
        npa_map.insert(
            input_fields[1].trim().to_string(),
            NpaData {
                npa_amount: input_fields[10].to_string(),
                npa_classification: input_fields[8].to_string(),
                cust_hlth_code: input_fields[12].to_string(),
                cust_npa_class: input_fields[17].to_string(),
                final_npa_class: input_fields[18].to_string(),
            },
        );
    }
    let end_npa_read_timer = SystemTime::now();
    let duration = end_npa_read_timer
        .duration_since(start_npa_read_timer)
        .expect("Could not calculate total duration for npa read timer.");
    debug!(log, "Readings npa File, Total Duration: {:?}.", duration);

    let gam_input_reader = BufReader::new(gam_input);
    //Adding header to the output file.
    // writeln!(
    //     op_writer,
    //     "NA|NA|DIS_AMT|REPHASEMENT_PRINCIPAL|REP_SHDL_NUM|REP_SHDL_DATE|ACID|BACID|CLR_BAL_AMT|SANCT_LIM|GL_SUB_HEAD_CODE|SCHM_CODE|SCHM_TYPE|ACCT_CRNCY_CODE|ACCT_OPN_DATE|DIS_SHDL_NUM|DIS_SHDL_DATE|SOL_ID|CUSTNAME|INTEREST_RATE|END_DATE|PEGGED_FLG|DER_PEGGED_FLG|CUST_ID|FORACID|INTEREST_RATE|EI_SCHM_FLG|O|LR_FREQ_TYPE||TRUE|M003|5|LIBOR|N|N|NUM_OF_FLOWS|FLOW_START_DATE|FLOW_AMT|LR_FREQ_TYPE|NUM_OF_DMDS|CASHFLOW_CODE|NA|||||EXRATE|SEGMENT_CODE||NPA_CLASSIFICATION|FLOATING_TYPE|OUT_BAL_AMT|CUST_HLTH_CODE|CUST_NPA_CLASS|FINAL_NPA_CLASS|REPRICING_PLAN|NEXT_REPRICING_DATE|INT_TBL_CODE",
    // ).expect("Could not write to output file.");
    //GAM
    for (_index, line) in gam_input_reader.lines().enumerate() {
        let line = line.expect("Could Not Read Line in GAM file.").to_string();
        let input_fields: Vec<&str> = line.split('|').collect();
        tot_records_read += 1;
        match lam_map.get(input_fields[0]) {
            //Implement inner join.
            Some(data) => match lrs_map.get(input_fields[0]) {
                Some(val) => {
                    for acc in val {
                        let mut new_acc = LoanAccount::new();
                        get_gam_fields(&mut new_acc, input_fields.to_owned());
                        new_acc.dis_amt = data.dis_amt.to_string();
                        new_acc.dis_shdl_date = data.dis_shdl_date;
                        new_acc.dis_shdl_num = data.dis_shdl_num.to_string();
                        new_acc.rep_shdl_date = data.rep_shdl_date;
                        new_acc.rep_shdl_num = data.rep_shdl_num;
                        new_acc.rephasement_principal =
                            data.rephasement_principal.trim().to_string();
                        new_acc.num_of_flows = acc.num_of_flows;
                        new_acc.flow_start_date = acc.flow_start_date;
                        new_acc.flow_amt = acc.flow_amt;
                        new_acc.lr_freq_type = acc.lr_freq_type.to_string();
                        new_acc.num_of_dmds = acc.num_of_dmds;
                        new_acc.cashflow_code = acc.cashflow_code.to_string();
                        let mut der_pegged_flg = "".to_string();
                        let mut int_tbl_code = "NA".to_string();
                        let mut npa_amount = 0.0;
                        match loan_int_map.get(input_fields[0]) {
                            Some(int_val) => {
                                new_acc.interest_rate = int_val.interest_rate;
                                new_acc.pegged_flg = int_val.pegged_flg.to_string();
                                int_tbl_code = int_val.int_tbl_code.to_string();
                                der_pegged_flg = if tblcodes_set.contains(&int_val.int_tbl_code) {
                                    "N1".to_string()
                                } else if benchmark_map.contains_key(&new_acc.foracid) {
                                    "N".to_string()
                                } else if int_val.pegged_flg.to_owned().is_empty()
                                    || int_val.pegged_flg.to_owned() == "NULL"
                                {
                                    "N".to_string()
                                } else {
                                    der_pegged_map
                                        .get(input_fields[0])
                                        .unwrap_or(&"N1".to_string())
                                        .to_owned()
                                };
                                match benchmark_map.get(input_fields[1]) {
                                    Some(bm_val) => {
                                        new_acc.end_date = bm_val.peg_review_date.to_owned();
                                        new_acc.floating_type = bm_val.floating_type.to_owned();
                                        new_acc.repricing_plan = bm_val.repricing_plan.to_owned();
                                        new_acc.next_repricing_date =
                                            bm_val.peg_review_date.to_owned();
                                    }
                                    None => {}
                                };
                                match npa_map.get(input_fields[1]) {
                                    Some(npa_val) => {
                                        new_acc.npa_classification =
                                            npa_val.npa_classification.to_owned();
                                        new_acc.cust_hlth_code = npa_val.cust_hlth_code.to_owned();
                                        new_acc.cust_npa_class = npa_val.cust_npa_class.to_owned();
                                        new_acc.final_npa_class =
                                            npa_val.final_npa_class.to_owned();
                                        npa_amount = npa_val.npa_amount.parse().unwrap_or(0.0);
                                    }
                                    None => {}
                                }
                            }
                            None => {}
                        };
                        if config_params.finacle_finlite_flg() == "FINACLE" {
                            new_acc.ei_schm_flg = data.ei_schm_flg.to_owned();
                        } else {
                            match lsp_map.get(input_fields[14]) {
                                Some(ei_schm_flg) => new_acc.ei_schm_flg = ei_schm_flg.to_string(),
                                None => {}
                            };
                        }
                        if new_acc.npa_classification == "0" {
                            writeln!(
                                op_writer,
                                "NA|NA|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|O|{}|{}|TRUE|M003|5|LIBOR|N|N|{}|{}|{}|{}|{}|{}|NA|||||{}|{}||{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
                                if new_acc.final_npa_class != "0".to_string() {
                                    "0".to_string()
                                } else {
                                    new_acc.dis_amt
                                },
                                new_acc.rephasement_principal,
                                if new_acc.final_npa_class != "0".to_string() {
                                    1
                                } else {
                                    new_acc.rep_shdl_num
                                },
                                if new_acc.final_npa_class != "0".to_string() {
                                    "31-01-2099".to_string()
                                } else {
                                    new_acc.rep_shdl_date.format("%d-%m-%Y").to_string()
                                },
                                new_acc.acid,
                                new_acc.bacid,
                                new_acc.clr_bal_amt,
                                new_acc.sanct_lim,
                                new_acc.gl_sub_head_code,
                                new_acc.schm_code,
                                new_acc.schm_type,
                                new_acc.acct_crncy_code,
                                new_acc.acct_opn_date.format("%d-%m-%Y"),
                                new_acc.dis_shdl_num,
                                if new_acc.final_npa_class != "0".to_string() {
                                    "31-01-2099".to_string()
                                } else {
                                    new_acc.dis_shdl_date.format("%d-%m-%Y").to_string()
                                },
                                new_acc.sol_id,
                                new_acc.custname,
                                new_acc.interest_rate,
                                new_acc.end_date.format("%d-%m-%Y"),
                                new_acc.pegged_flg,
                                new_acc.cust_id,
                                new_acc.foracid,
                                new_acc.interest_rate,
                                if new_acc.final_npa_class != "0".to_string() {
                                    "P".to_string()
                                } else {
                                    new_acc.ei_schm_flg
                                },
                                new_acc.lr_freq_type,
                                if new_acc.sol_id.contains("NFS") {
                                    "NFS"
                                } else {
                                    "0"
                                },
                                new_acc.num_of_flows,
                                new_acc.flow_start_date.format("%d-%m-%Y"),
                                new_acc.flow_amt,
                                new_acc.lr_freq_type,
                                new_acc.num_of_dmds,
                                new_acc.cashflow_code,
                                new_acc.exrate,
                                new_acc.segment_code,
                                new_acc.npa_classification,
                                new_acc.floating_type,
                                new_acc.out_bal_amt,
                                new_acc.cust_hlth_code,
                                new_acc.cust_npa_class,
                                new_acc.final_npa_class,
                                new_acc.repricing_plan,
                                new_acc.next_repricing_date.format("%d-%m-%Y"),
                                der_pegged_flg,
                                int_tbl_code,
                            )
                            .expect("Could not write to output file.");
                        } else {
                            writeln!(
                                op_writer,
                                "NA|NA|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|O|{}|{}|TRUE|M003|5|LIBOR|N|N|{}|{}|{}|{}|{}|{}|NA|||||{}|{}||{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
                                if new_acc.final_npa_class != "0".to_string() {
                                    "0".to_string()
                                } else {
                                    new_acc.dis_amt
                                },
                                new_acc.rephasement_principal,
                                if new_acc.final_npa_class != "0".to_string() {
                                    1
                                } else {
                                    new_acc.rep_shdl_num
                                },
                                if new_acc.final_npa_class != "0".to_string() {
                                    "31-01-2099".to_string()
                                } else {
                                    new_acc.rep_shdl_date.format("%d-%m-%Y").to_string()
                                },
                                new_acc.acid,
                                new_acc.bacid,
                                npa_amount,
                                new_acc.sanct_lim,
                                new_acc.gl_sub_head_code,
                                new_acc.schm_code,
                                new_acc.schm_type,
                                new_acc.acct_crncy_code,
                                new_acc.acct_opn_date.format("%d-%m-%Y"),
                                new_acc.dis_shdl_num,
                                if new_acc.final_npa_class != "0".to_string() {
                                    "31-01-2099".to_string()
                                } else {
                                    new_acc.dis_shdl_date.format("%d-%m-%Y").to_string()
                                },
                                new_acc.sol_id,
                                new_acc.custname,
                                new_acc.interest_rate,
                                new_acc.end_date.format("%d-%m-%Y"),
                                new_acc.pegged_flg,
                                new_acc.cust_id,
                                new_acc.foracid,
                                new_acc.interest_rate,
                                if new_acc.final_npa_class != "0".to_string() {
                                    "P".to_string()
                                } else {
                                    new_acc.ei_schm_flg
                                },
                                new_acc.lr_freq_type,
                                if new_acc.sol_id.contains("NFS") {
                                    "NFS"
                                } else {
                                    "0"
                                },
                                new_acc.num_of_flows,
                                new_acc.flow_start_date.format("%d-%m-%Y"),
                                new_acc.flow_amt,
                                new_acc.lr_freq_type,
                                new_acc.num_of_dmds,
                                new_acc.cashflow_code,
                                new_acc.exrate,
                                new_acc.segment_code,
                                new_acc.npa_classification,
                                new_acc.floating_type,
                                npa_amount,
                                new_acc.cust_hlth_code,
                                new_acc.cust_npa_class,
                                new_acc.final_npa_class,
                                new_acc.repricing_plan,
                                new_acc.next_repricing_date.format("%d-%m-%Y"),
                                der_pegged_flg,
                                int_tbl_code,
                            )
                            .expect("Could not write to output file.");
                        }
                    }
                    tot_records_success += 1;
                }
                None => {
                    log_debug!(log, "Could not find LRS data for acid:{}", input_fields[0])
                }
            },
            None => {
                log_debug!(log, "Cannot find LAM data for acid:{}", input_fields[0])
            }
        };
    }

    let health_report = HealthReport::new(
        tot_records_read,
        tot_records_success,
        tot_records_read - tot_records_success,
        0.0,
        0.0,
        0,
    );
    health_report.gen_health_rpt(config_params.output_file_path());
}
