use account_writer::AccountWithCashflows;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use rbdate::{timestamp, DateParser, NaiveDate};
use slog::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;
pub mod structs;
use account::{Account, Cashflow};
use account_appender::account_appender::create_acc_wt_cfs;
use process::structs::OPFields;

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let mut writer = AccountWithCashflows::new(config_params.output_file_path(), logger);
    let mut accounts: Vec<Account> = Vec::new();
    let mut tot_acc_encntrd = 0;
    let mut acc_pro_suc = 0;

    let cp_date_parser = DateParser::new("%d %b %Y".to_string(), false);
    let mm_date_parser = DateParser::new("%d %b %Y".to_string(), false);

    //Counter Party file
    let counter_party =
        File::open(&config_params.counter_party_file()).expect("Could Not Read Counter party File");
    let cp_reader = BufReader::new(counter_party);
    let mut counter_party: HashMap<String, NaiveDate> = HashMap::new();
    for (index, line) in cp_reader.lines().enumerate().skip(1) {
        tot_acc_encntrd += 1;
        acc_pro_suc += 1;
        let line = line.expect("Could Not Read Line").to_string();
        let cp_fields: Vec<&str> = line.split('|').collect();
        counter_party.insert(
            cp_fields[3].to_string(),
            cp_date_parser.parse(&cp_fields[18].to_string()),
        );

        //Inflow Account
        let mut new_cf = Cashflow::new();
        new_cf.int_amt = 0.0;
        new_cf.prin_amt = cp_fields[11].parse::<f64>().unwrap_or(0.0);
        new_cf.date = timestamp(cp_date_parser.parse(&cp_fields[18].to_string()));
        let mut new_acc = OPFields {
            entity: cp_fields[0].to_string(),
            source: "COUNTER PARTY".to_string(),
            in_out: "INFLOW".to_string(),
            sub_type: cp_fields[1].to_string(),
            counter_party: cp_fields[3].to_string(),
            currency: cp_fields[9].to_string(),
            avaliabile_limit: cp_fields[17].parse::<f64>().unwrap_or(0.0),
            deal_amount_lcy: cp_fields[17].parse::<f64>().unwrap_or(0.0),
            cf_date: new_cf.date,
            cp_parent_id: cp_fields[3].to_string(),
            cashflows: new_cf,
        };
        accounts.push(create_acc_wt_cfs(&new_acc));

        //Outflow Account
        new_acc.in_out = "OUTFLOW".to_string();
        let mut new_cf = Cashflow::new();
        new_cf.int_amt = 0.0;
        new_cf.prin_amt = cp_fields[17].parse::<f64>().unwrap_or(0.0);
        new_cf.date = timestamp(cp_date_parser.parse(&cp_fields[18].to_string()));
        new_acc.cashflows = new_cf;
        accounts.push(create_acc_wt_cfs(&new_acc));
    }

    //Master MM Accounts
    let master_mm =
        File::open(&config_params.master_mm_file()).expect("Could Not Read  Master MM File");
    let master_mm_reader = BufReader::new(master_mm);

    for (index, line) in master_mm_reader.lines().enumerate().skip(1) {
        tot_acc_encntrd += 1;
        acc_pro_suc += 1;
        let line = line.expect("Could Not Read Line").to_string();
        let mm_fields: Vec<&str> = line.split("|").collect();
        if !counter_party.contains_key(mm_fields[68]) {
            continue;
        }
        //Outflow account
        let mut deal_amount_lcy = 0.0;
        if mm_fields[34].to_string() != "".to_string() {
            if &mm_date_parser.parse(&mm_fields[34].to_string())
                < counter_party
                    .get(mm_fields[68])
                    .expect("Could Not find Counter Party")
            {
                deal_amount_lcy = mm_fields[24].parse::<f64>().unwrap_or(0.0)
            }
        }
        let mut new_cf = Cashflow::new();
        new_cf.int_amt = 0.0;
        new_cf.prin_amt = deal_amount_lcy;
        new_cf.date = if mm_fields[34].to_string() != "".to_string() {
            timestamp(mm_date_parser.parse(&mm_fields[34].to_string()))
        } else {
            timestamp(
                *counter_party
                    .get(mm_fields[68])
                    .expect("Could Not find Counter Party"),
            )
        };
        let mut new_acc = OPFields {
            entity: mm_fields[1].to_string(),
            source: "MM OUTSTANDING".to_string(),
            in_out: "OUTFLOW".to_string(),
            sub_type: mm_fields[6].to_string(),
            counter_party: mm_fields[11].to_string(),
            currency: mm_fields[20].to_string(),
            avaliabile_limit: deal_amount_lcy,
            deal_amount_lcy: deal_amount_lcy,
            cf_date: new_cf.date,
            cp_parent_id: mm_fields[68].to_string(),
            cashflows: new_cf,
        };
        accounts.push(create_acc_wt_cfs(&new_acc));

        //Inflow account
        new_acc.deal_amount_lcy = mm_fields[24].parse::<f64>().unwrap_or(0.0);
        new_acc.avaliabile_limit = mm_fields[24].parse::<f64>().unwrap_or(0.0);
        let mut new_cf = Cashflow::new();
        new_cf.int_amt = 0.0;
        new_cf.prin_amt = mm_fields[24].parse::<f64>().unwrap_or(0.0);
        new_cf.date = if mm_fields[34].to_string() != "".to_string() {
            timestamp(mm_date_parser.parse(&mm_fields[34].to_string()))
        } else {
            timestamp(
                *counter_party
                    .get(mm_fields[68])
                    .expect("Could Not find Counter Party"),
            )
        };
        new_acc.cf_date = new_cf.date;
        new_acc.cashflows = new_cf;
        new_acc.in_out = "INFLOW".to_string();
        accounts.push(create_acc_wt_cfs(&new_acc));
    }

    for acc in accounts.iter() {
        writer.write(acc.clone());
    }

    let health_report = HealthReport::new(
        tot_acc_encntrd,
        acc_pro_suc,
        tot_acc_encntrd - acc_pro_suc,
        0.0,
        0.0,
        0,
    );
    health_report.gen_health_rpt(&config_params.output_file_path());
}
