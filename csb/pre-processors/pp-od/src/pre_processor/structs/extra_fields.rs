use rbdate::NaiveDate;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraFieldData {
    pub acc_id: String,
    pub sanc_dt: String,
    pub occp_cd: String,
    pub sens_sec: String,
    pub prior_subtype: String,
    pub restruct_flag: String,
    pub restruct_dt: String,
    pub mor_prd: String,
    pub rating: String,
    pub consitin: String,
    pub pan: String,
    pub limit_amt: String,
    pub gross_adv: String,
    pub exp_amt: String,
    pub unvail_amt: String,
    pub gold_gram: String,
    pub fund_flag: String,
}

impl Default for ExtraFieldData {
    fn default() -> ExtraFieldData {
        ExtraFieldData {
            acc_id: "NA".to_string(),
            sanc_dt: "NA".to_string(),
            occp_cd: "NA".to_string(),
            sens_sec: "NA".to_string(),
            prior_subtype: "NA".to_string(),
            restruct_flag: "NA".to_string(),
            restruct_dt: "NA".to_string(),
            mor_prd: "NA".to_string(),
            rating: "NA".to_string(),
            consitin: "NA".to_string(),
            pan: "NA".to_string(),
            limit_amt: "NA".to_string(),
            gross_adv: "NA".to_string(),
            exp_amt: "NA".to_string(),
            unvail_amt: "NA".to_string(),
            gold_gram: "NA".to_string(),
            fund_flag: "NA".to_string(),
        }
    }
}

impl FromStr for ExtraFieldData {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<&str> = s
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .collect();
        let acc_id = values[0].to_string();
        let sanc_dt = values[1].to_string();
        let occp_cd = values[2].to_string();
        let sens_sec = values[3].to_string();
        let prior_subtype = values[4].to_string();
        let restruct_flag = values[5].to_string();
        let restruct_dt = values[6].to_string();
        let mor_prd = values[7].to_string();
        let rating = values[8].to_string();
        let consitin = values[9].to_string();
        let pan = values[10].to_string();
        let limit_amt = values[11].to_string();
        let gross_adv = values[12].to_string();
        let exp_amt = values[13].to_string();
        let unvail_amt = values[14].to_string();
        let gold_gram = values[15].to_string();
        let fund_flag = values[16].to_string();

        Ok(ExtraFieldData {
            acc_id,
            sanc_dt,
            occp_cd,
            sens_sec,
            prior_subtype,
            restruct_flag,
            restruct_dt,
            mor_prd,
            rating,
            consitin,
            pan,
            limit_amt,
            gross_adv,
            exp_amt,
            unvail_amt,
            gold_gram,
            fund_flag,
        })
    }
}

impl ExtraFieldData {
    pub fn print(&self) -> String {
        format!(
            "|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|",
            NaiveDate::parse_from_str(&self.sanc_dt, "%d-%m-%Y")
                .unwrap_or(NaiveDate::from_ymd(1970, 1, 1))
                .format("%d-%m-%Y")
                .to_string(),
            self.occp_cd,
            self.sens_sec,
            self.prior_subtype,
            self.restruct_flag,
            NaiveDate::parse_from_str(&self.restruct_dt, "%d-%m-%Y")
                .unwrap_or(NaiveDate::from_ymd(1970, 1, 1))
                .format("%d-%m-%Y")
                .to_string(),
            self.mor_prd,
            self.rating,
            self.consitin,
            self.pan,
            self.limit_amt,
            self.gross_adv,
            self.exp_amt,
            self.unvail_amt,
            self.gold_gram,
            self.fund_flag,
        )
    }
}
