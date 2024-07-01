use std::collections::HashMap;

use rbdate::{DateParser, NaiveDate};
use slog::Logger;

use crate::configuration_parameters::ConfigurationParameters;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub deal_ref: String,
    pub security_name: String,
    pub portfolio_code: String,
    pub counterparty: String,
    pub currency: String,
    pub deal_date: Option<NaiveDate>,
    pub value_date: Option<NaiveDate>,
    pub maturity_date: Option<NaiveDate>,
    pub repo_period: String,
    pub repo_rate: f64,
    pub interest_practice: String,
    pub dealer_name: String,
    pub stock_exchange: String,
    pub sub_category: String,
    pub broker_name: String,
    pub broker_amount: f64,
    pub face_value: f64,
    pub repo_interest: f64,
    pub leg1_price: f64,
    pub leg2_price: f64,
    pub accrued_interest_leg1: f64,
    pub accrued_interest_leg2: f64,
    pub settlement_amt_leg1: f64,
    pub settlement_amt_leg2: f64,
    pub coupon_cashflow: f64,
    pub yield_leg1: f64,
    pub yield_leg2: f64,
    pub portfolio: String,
    pub instrument_type: String,
    pub instrument_name: String,
    pub repo_category: String,
    pub repo_type: String,
    pub gl_code: String,
    pub borrowing_type: String,
    pub interest_type: String,
}

impl InputAccount {
    pub fn new(input_acc: Vec<&str>) -> InputAccount {
        InputAccount {
            deal_ref: input_acc[0].to_string(),
            security_name: input_acc[1].to_string(),
            portfolio_code: input_acc[2].to_string(),
            counterparty: input_acc[3].to_string(),
            currency: input_acc[4].to_string(),
            deal_date: DateParser::new("%d-%b-%Y".to_string(), false).parse_opt(input_acc[5]),
            value_date: DateParser::new("%d-%b-%Y".to_string(), false).parse_opt(input_acc[5]),
            maturity_date: DateParser::new("%d-%b-%Y".to_string(), false).parse_opt(input_acc[5]),
            repo_period: input_acc[8].to_string(),
            repo_rate: input_acc[9].parse().unwrap_or(0.0),
            interest_practice: input_acc[10].to_string(),
            dealer_name: input_acc[11].to_string(),
            stock_exchange: input_acc[12].to_string(),
            sub_category: input_acc[13].to_string(),
            broker_name: input_acc[14].to_string(),
            broker_amount: input_acc[15].parse().unwrap_or(0.0),
            face_value: input_acc[16].parse().unwrap_or(0.0),
            repo_interest: input_acc[17].parse().unwrap_or(0.0),
            leg1_price: input_acc[18].parse().unwrap_or(0.0),
            leg2_price: input_acc[19].parse().unwrap_or(0.0),
            accrued_interest_leg1: input_acc[20].parse().unwrap_or(0.0),
            accrued_interest_leg2: input_acc[21].parse().unwrap_or(0.0),
            settlement_amt_leg1: input_acc[22].parse().unwrap_or(0.0),
            settlement_amt_leg2: input_acc[23].parse().unwrap_or(0.0),
            coupon_cashflow: input_acc[24].parse().unwrap_or(0.0),
            yield_leg1: input_acc[25].parse().unwrap_or(0.0),
            yield_leg2: input_acc[26].parse().unwrap_or(0.0),
            portfolio: input_acc[27].to_string(),
            instrument_type: input_acc[28].to_string(),
            instrument_name: input_acc[29].to_string(),
            repo_category: input_acc[30].to_string(),
            repo_type: input_acc[31].to_string(),
            gl_code: input_acc[32].to_string(),
            borrowing_type: input_acc[33].to_string(),
            interest_type: input_acc[34].to_string(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct OutputAccount {
    pub deal_ref: String,
    pub security_name: String,
    pub portfolio_code: String,
    pub counterparty: String,
    pub currency: String,
    pub deal_date: String,
    pub value_date: String,
    pub maturity_date: String,
    pub repo_period: String,
    pub repo_rate: String,
    pub interest_practice: String,
    pub dealer_name: String,
    pub stock_exchange: String,
    pub sub_category: String,
    pub broker_name: String,
    pub broker_amount: String,
    pub face_value: String,
    pub repo_interest: String,
    pub leg1_price: String,
    pub leg2_price: String,
    pub accrued_interest_leg1: String,
    pub accrued_interest_leg2: String,
    pub settlement_amt_leg1: String,
    pub settlement_amt_leg2: String,
    pub coupon_cashflow: String,
    pub yield_leg1: String,
    pub yield_leg2: String,
    pub portfolio: String,
    pub instrument_type: String,
    pub instrument_name: String,
    pub repo_category: String,
    pub repo_type: String,
    pub gl_code: String,
    pub borrowing_type: String,
    pub interest_type: String,
    pub cgl: String,
    pub grp: String,
    pub llg: String,
}

impl OutputAccount {
    pub fn new(input_acc: InputAccount, config_params: &ConfigurationParameters, bgl_cgl_map: &HashMap<String, String>, log: &Logger, line_num: &usize, master_map: &HashMap<String, master_val>) -> OutputAccount {
        let mut cgl = "NA";
        if input_acc.gl_code.chars().count() >= 10 {
            if bgl_cgl_map.contains_key(&input_acc.gl_code[..10].to_string()) {
                cgl = bgl_cgl_map
                    .get(&input_acc.gl_code[..10].to_string())
                    .unwrap();
            } else {
                info!(
                    log,
                    "Can not find the value from bgl_cgl_map for: {} in line no: {}",
                    &input_acc.gl_code,
                    line_num
                );
            }
        } else {
            info!(
                log,
                "length of 33th field(val: '{}') is less than ten in line no: {}",
                &input_acc.gl_code,
                line_num
            );
        }

        let mut grp = "NA";
        let mut llg = "NA";
        if master_map.contains_key(cgl) {
            let val = master_map.get(cgl).unwrap();
            grp = &val.grp;
            llg = &val.llg;
        } else {
            info!(
                log,
                "Can not find the value from master_map for: {} in line no: {}", cgl, line_num
            );
        }
        
        OutputAccount {
            deal_ref: input_acc.deal_ref,
            security_name: input_acc.security_name,
            portfolio_code: input_acc.portfolio_code,
            counterparty: input_acc.counterparty,
            currency: input_acc.currency,
            deal_date: input_acc
                .deal_date
                .unwrap_or(*config_params.as_on_date())
                .format("%d-%m-%Y")
                .to_string(),
            value_date: input_acc
                .value_date
                .unwrap_or(*config_params.as_on_date())
                .format("%d-%m-%Y")
                .to_string(),
            maturity_date: input_acc
                .maturity_date
                .unwrap_or(*config_params.as_on_date())
                .format("%d-%m-%Y")
                .to_string(),
            repo_period: input_acc.repo_period,
            repo_rate: input_acc.repo_rate.to_string(),
            interest_practice: input_acc.interest_practice,
            dealer_name: input_acc.dealer_name,
            stock_exchange: input_acc.stock_exchange,
            sub_category: input_acc.sub_category,
            broker_name: input_acc.broker_name,
            broker_amount: input_acc.broker_amount.to_string(),
            face_value: input_acc.face_value.to_string(),
            repo_interest: input_acc.repo_interest.to_string(),
            leg1_price: input_acc.leg1_price.to_string(),
            leg2_price: input_acc.leg2_price.to_string(),
            accrued_interest_leg1: input_acc.accrued_interest_leg1.to_string(),
            accrued_interest_leg2: input_acc.accrued_interest_leg2.to_string(),
            settlement_amt_leg1: input_acc.settlement_amt_leg1.to_string(),
            settlement_amt_leg2: input_acc.settlement_amt_leg2.to_string(),
            coupon_cashflow: input_acc.coupon_cashflow.to_string(),
            yield_leg1: input_acc.yield_leg1.to_string(),
            yield_leg2: input_acc.yield_leg2.to_string(),
            portfolio: input_acc.portfolio,
            instrument_type: input_acc.instrument_type,
            instrument_name: input_acc.instrument_name,
            repo_category: input_acc.repo_category,
            repo_type: input_acc.repo_type,
            gl_code: input_acc.gl_code,
            borrowing_type: input_acc.borrowing_type,
            interest_type: input_acc.interest_type,
            cgl: cgl.to_string(),
            grp: grp.to_string(),
            llg: llg.to_string(),
        }
    }
}

pub fn format_output(output_rec: &OutputAccount) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        output_rec.deal_ref,
        output_rec.security_name,
        output_rec.portfolio_code,
        output_rec.counterparty,
        output_rec.currency,
        output_rec.deal_date,
        output_rec.value_date,
        output_rec.maturity_date,
        output_rec.repo_period,
        output_rec.repo_rate,
        output_rec.interest_practice,
        output_rec.dealer_name,
        output_rec.stock_exchange,
        output_rec.sub_category,
        output_rec.broker_name,
        output_rec.broker_amount,
        output_rec.face_value,
        output_rec.repo_interest,
        output_rec.leg1_price,
        output_rec.leg2_price,
        output_rec.accrued_interest_leg1,
        output_rec.accrued_interest_leg2,
        output_rec.settlement_amt_leg1,
        output_rec.settlement_amt_leg2,
        output_rec.coupon_cashflow,
        output_rec.yield_leg1,
        output_rec.yield_leg2,
        output_rec.portfolio,
        output_rec.instrument_type,
        output_rec.instrument_name,
        output_rec.repo_category,
        output_rec.repo_type,
        output_rec.gl_code,
        output_rec.borrowing_type,
        output_rec.interest_type,
        output_rec.cgl,
        output_rec.grp,
        output_rec.llg,
    )
}

pub struct master_val {
    pub grp: String,
    pub llg: String,
}

impl master_val {
    pub fn new(grp: String, llg: String) -> Self {
        Self { grp, llg }
    }
}
