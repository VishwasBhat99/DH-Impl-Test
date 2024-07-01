use crate::configuration_parameters::ConfigurationParameters;
use crate::process::input_account::*;
use crate::process::output_account::{format_output, get_writer, OutputField};
use calamine::{open_workbook_auto, Reader};
use health_report::HealthReport;
use slog::Logger;
use std::collections::{HashMap, HashSet};
use std::iter::Product;
use std::path::Component;
use std::{fs, io::Write};
mod input_account;
mod output_account;

pub fn process(config_params: &ConfigurationParameters, _logger: &Logger, _diag_logger: &Logger) {
    let mut acc_enc = 0;
    let mut acc_proc = 0;

    let comp_skip_vec: Vec<&str> = config_params.cashflow_component_skip().split(',').collect();
    let product_code_skip: Vec<&str> = config_params.product_code_skip().split(',').collect();

    //Reading NPA file
    let mut npa_map: HashMap<String, NpaData> = HashMap::new();
    let npa_file_reader =
        fs::read_to_string(config_params.npa_file_path()).expect("Could Not Read NPA file");
    for (line_no, line) in npa_file_reader.lines().enumerate() {
        let npa_vec: Vec<&str> = line.split(',').collect::<Vec<&str>>();
        let npa_data: NpaData = NpaData::new(
            config_params,
            config_params.npa_file_path(),
            &npa_vec,
            line_no + 1,
        );
        let key = npa_data.accref_num.clone();
        npa_map.insert(key, npa_data);
    }

    //Reading Ubs loan Cashflows file

    let mut cashflow_map: HashMap<String, CashflowData> = HashMap::new();
    let cashflow_file_reader = fs::read_to_string(config_params.ubs_loans_cashflow_file_path())
        .expect("Could Not Read UBS Cashflow file");
    for (line_no, line) in cashflow_file_reader.lines().enumerate() {
        let cashflow_vec: Vec<&str> = line.split('~').collect::<Vec<&str>>();
        let cashflow_data: CashflowData = CashflowData::new(
            config_params,
            config_params.ubs_loans_cashflow_file_path(),
            &cashflow_vec,
            line_no + 1,
        );
        let component = cashflow_data.component.as_str().clone();
        let amt_due = cashflow_data.amount_due.clone();
        let amt_settled = cashflow_data.amount_settled.clone();
        let tot_amt = amt_due - amt_settled.clone();
        let ref_key = cashflow_data.reference.clone();
        if comp_skip_vec.contains(&component.to_uppercase().as_str()) && tot_amt != 0.0 {
            cashflow_map.insert(ref_key, cashflow_data);
        }
    }
    //Reading Ubs loans master file
    let mut master_map: HashMap<String, Vec<OutputField>> = HashMap::new();
    let master_file_reader = fs::read_to_string(config_params.ubs_loans_master_file_path())
        .expect("Could Not Read UBS Master file");
    for (line_no, line) in master_file_reader.lines().enumerate() {
        acc_enc += 1;
        let master_vec: Vec<&str> = line.split('~').collect::<Vec<&str>>();
        let master_data: MasterData = MasterData::new(
            config_params,
            config_params.ubs_loans_master_file_path(),
            &master_vec,
            line_no + 1,
        );
        let ac_ccy_out_bal = master_data.ac_ccy_outstand_bal.clone();
        let product_code = master_data.product_code.clone();

        if !product_code_skip.contains(&product_code.to_uppercase().as_str())
            && ac_ccy_out_bal != 0.0
        {
            let counter_party = master_data.counter_party.clone();
            let cust_name1 = master_data.cust_name1;
            let branch = master_data.branch.replace("\"", "");
            let int_rate = master_data.int_rate;
            let accrual_freq = master_data.accrual_freq;
            let cont_ref_key = master_data.contract_ref_no;
            let default_1 = CashflowData::default();
            let cashflow_data = cashflow_map.get(&cont_ref_key).unwrap_or(&default_1);
            let default_2 = NpaData::default();
            let npa_data = npa_map.get(&cont_ref_key).unwrap_or(&default_2);

            //booking date
            let formatted_date = master_data
                .booking_date
                .format("%d-%b-%Y")
                .to_string()
                .to_lowercase();
            let formatted_date_temp = master_data.booking_date.format("%d-%m-").to_string();
            let year_last_two = &formatted_date[9..];
            let new_booking_date = format!("{}{}{}", formatted_date_temp, "20", year_last_two);
            //mat date
            let formatted_date_mat = master_data.mat_date.format("%d-%b-%Y").to_string();
            let formatted_date_mat_temp = master_data.mat_date.format("%d-%m-").to_string();
            let year_last_two_1 = &formatted_date_mat[9..];
            let new_mat_date = format!("{}{}{}", formatted_date_mat_temp, "20", year_last_two_1);

            //value date
            let formatted_date_val = master_data.value_dt.format("%d-%b-%Y").to_string();
            let formatted_date_val_temp = master_data.value_dt.format("%d-%m-").to_string();
            let year_last_two_2 = &formatted_date_val[9..];
            let new_val_date = format!("{}{}{}", formatted_date_val_temp, "20", year_last_two_2);

            //schedule due date
            let formatted_date_sche = cashflow_data.schedule_due_dt.format("%d-%b-%Y").to_string();
            let formatted_date_sche_temp =
                cashflow_data.schedule_due_dt.format("%d-%m-").to_string();
            let year_last_two_3 = &formatted_date_sche[9..];
            let new_sche_date = format!("{}{}{}", formatted_date_sche_temp, "20", year_last_two_3);

            let asst_class = &npa_data.asst_class;
            let product_code = master_data.product_code;
            let amt_due = cashflow_data.amount_due;
            let amt_settled = cashflow_data.amount_settled;
            let lcy_outstand_bal = master_data.lcy_outstand_bal;
            let ac_ccy_bal = master_data.ac_ccy_outstand_bal;
            let bal = (amt_due - amt_settled) * (lcy_outstand_bal / ac_ccy_out_bal);

            //next reset date
            let formatted_date_next_reset = master_data
                .new_last_reset_date
                .format("%d-%b-%Y")
                .to_string();
            let formatted_date_next_reset_temp =
                master_data.new_last_reset_date.format("%d-%m-").to_string();
            let year_last_two_5 = &formatted_date_next_reset[9..];
            let new_next_reset_date = format!(
                "{}{}{}",
                formatted_date_next_reset_temp, "20", year_last_two_5
            );

            //last reset date
            let formatted_date_last_reset = master_data
                .new_next_reset_date
                .format("%d-%b-%Y")
                .to_string();
            let formatted_date_last_reset_temp =
                master_data.new_next_reset_date.format("%d-%m-").to_string();
            let year_last_two_4 = &formatted_date_last_reset[9..];
            let new_last_reset_date = format!(
                "{}{}{}",
                formatted_date_last_reset_temp, "20", year_last_two_4
            );

            let output_data = OutputField {
                counter_party_1: counter_party.clone(),
                contract_ref_no: cont_ref_key,
                cust_name1: cust_name1,
                branch: branch,
                int_rate: int_rate,
                accural_freq: accrual_freq,
                booking_date: new_booking_date,
                value_dt: new_val_date,
                mat_date: new_mat_date,
                schedule_due_date: new_sche_date,
                asst_class: asst_class.to_string(),
                product_code: product_code,
                gl: master_data.gl,
                contract_ccy: master_data.contract_ccy,
                lcy_outstand_bal: master_data.lcy_outstand_bal,
                component: cashflow_data.component.clone(),
                amount_due: cashflow_data.amount_due,
                amount_settled: cashflow_data.amount_settled,
                balance: bal,
                rate_spread: master_data.rate_spread,
                comp_mis_1: master_data.comp_mis_1,
                comp_mis_2: master_data.comp_mis_2,
                comp_mis_3: master_data.comp_mis_3,
                rate_code: master_data.rate_code,
                rate_type: master_data.rate_typ,
                benchmark_rate: master_data.benchmark_rate,
                new_next_reset_date: new_next_reset_date,
                new_last_reset_date: new_last_reset_date,
                weaker_section: master_data.weaker_section,
                msme: master_data.msme,
                call: master_data.call,
                call_option_date: master_data.call_option_date.format("%d-%m-%Y").to_string(),
                put: master_data.put,
                put_option_date: master_data.put_option_date.format("%d-%m-%Y").to_string(),
                last_reset_date: master_data
                    .new_last_reset_date
                    .format("%d-%m-%Y")
                    .to_string(),
                next_reset_date: master_data
                    .new_next_reset_date
                    .format("%d-%m-%Y")
                    .to_string(),
                counter_party: master_data.counter_party,
                lcy_amt: master_data.lcy_amt,
                frequency: master_data.frequency,
                desc: master_data.description,
                prod_desc: master_data.product_desc,
            };
            master_map
                .entry(counter_party)
                .and_modify(|prev_data| prev_data.push(output_data.clone()))
                .or_insert(vec![output_data]);
        }
    }
    let mut sorted_map: Vec<_> = master_map.into_iter().collect();
    sorted_map.sort_by_key(|(key, _)| key.clone());

    // Reconstruct the sorted HashMap
    let sorted_master_map: HashMap<_, _> = sorted_map.into_iter().collect();

    let mut op_writer = get_writer(config_params.output_file());
    for (_key, val) in sorted_master_map {
        acc_proc += 1;
        for ele in val {
            writeln!(op_writer, "{}", format_output(ele.clone())).expect("Error in Writing Output");
        }
    }
    let health_report = HealthReport::new(acc_enc, acc_proc, 0, 0.0, 0.0, 0);
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_params.output_file());
}
