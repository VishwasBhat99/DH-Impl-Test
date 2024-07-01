use configuration_parameters::ConfigurationParameters;
use rbdate::NaiveDate;

#[derive(Debug)]
pub struct CRMData {
    aod: String,
    pub acc_id: String,
    col_id: String,
    claim_id: String,
    col_type: String,
    pub tot_col_amt_lcy: f64,
    tot_col_amt_ccy: f64,
    col_mat_dt: NaiveDate,
    acc_mat_dt: NaiveDate,
    acc_ccy: String,
    col_ccy: String,
    col_hc_perc: f64,
    col_hc_amt: f64,
    ccy_mismatch_perc: f64,
    mat_mismatch_perc: f64,
    pub crm_amt_lcy: f64,
    col_hc_perc_rule_id: String,
    ccy_mismatch_perc_rule_id: String,
    mat_mismatch_perc_rule_id: String,
}

impl CRMData {
    pub fn print(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.aod,
            self.acc_id,
            self.col_id,
            self.claim_id,
            self.col_type,
            self.tot_col_amt_lcy,
            self.tot_col_amt_ccy,
            self.col_mat_dt.format("%d-%m-%Y").to_string(),
            self.col_ccy,
            self.col_hc_perc,
            self.ccy_mismatch_perc,
            self.mat_mismatch_perc,
            self.crm_amt_lcy,
            self.col_hc_perc_rule_id,
            self.ccy_mismatch_perc_rule_id,
            self.mat_mismatch_perc_rule_id,
        )
    }
}

pub fn get_crm_data(aod: &NaiveDate, line: String, claim_info: &str) -> CRMData {
    let line_info: Vec<&str> = line.split('|').collect();

    CRMData {
        aod: aod.format("%d-%m-%Y").to_string(),
        acc_id: line_info[1].to_string(),
        col_id: line_info[0].to_string(),
        claim_id: claim_info.to_string(),
        col_type: line_info[3].to_string(),
        tot_col_amt_lcy: line_info[6].parse().unwrap_or(0.0),
        //TODO: Correct CCY Amount Calculation
        tot_col_amt_ccy: line_info[6].parse().unwrap_or(0.0),
        col_mat_dt: NaiveDate::parse_from_str(line_info[7], "%d-%m-%Y")
            .unwrap_or(NaiveDate::from_ymd(1970, 1, 1)),
        acc_mat_dt: NaiveDate::from_ymd(1970, 1, 1),
        acc_ccy: "".to_string(),
        col_ccy: line_info[5].to_string(),
        col_hc_perc: line_info[9].parse().unwrap_or(0.0),
        col_hc_amt: line_info[10].parse().unwrap_or(0.0),
        ccy_mismatch_perc: 0.0,
        mat_mismatch_perc: 0.0,
        crm_amt_lcy: 0.0,
        col_hc_perc_rule_id: "".to_string(),
        ccy_mismatch_perc_rule_id: "".to_string(),
        mat_mismatch_perc_rule_id: "".to_string(),
    }
}

pub fn update_data(
    col_data: &mut Vec<CRMData>,
    acc_ccy: String,
    acc_mat_date: NaiveDate,
    config_params: &ConfigurationParameters,
) {
    for mut data in col_data {
        data.acc_ccy = acc_ccy.to_string();
        data.acc_mat_dt = acc_mat_date;
        if data.col_ccy != data.acc_ccy {
            data.ccy_mismatch_perc = *config_params.ccy_mm_hc_prcnt();
        }
        if data.col_mat_dt != data.acc_mat_dt {
            data.mat_mismatch_perc = *config_params.mat_mm_hc_prcnt();
        }
        let ccy_hc_amt = (data.tot_col_amt_lcy * data.ccy_mismatch_perc) / 100.0;
        let mm_hc_amt = (data.tot_col_amt_lcy * data.mat_mismatch_perc) / 100.0;
        let final_crm_amt_lcy = data.tot_col_amt_lcy - data.col_hc_amt - ccy_hc_amt - mm_hc_amt;
        data.crm_amt_lcy = final_crm_amt_lcy;
    }
}
