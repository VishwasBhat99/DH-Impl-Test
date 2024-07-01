use crate::configuration_parameters::ConfigurationParameters;
use cond_utils::In;

#[derive(Debug, Clone, Default)]
/// The structure in which MasterMapping Data is Read
pub struct MasterMapping {
    pub gl_acct_no: String,
    pub description: String,
    pub classification: String,
    pub group: String,
    pub llg: String,
    pub other_llg_class: String,
    pub logic: String,
}

impl MasterMapping {
    pub fn new(master_map: &[calamine::DataType]) -> MasterMapping {
        MasterMapping {
            gl_acct_no: master_map[0].to_string(),
            description: master_map[1].to_string(),
            classification: master_map[2].to_string(),
            group: master_map[3].to_string(),
            llg: master_map[4].to_string(),
            other_llg_class: master_map[5].to_string(),
            logic: master_map[6].to_string(),
        }
    }
    pub fn def() -> MasterMapping {
        ::std::default::Default::default()
    }
}

#[derive(Debug, Clone, Default)]
/// The structure in which Input is Read And Output is Written
pub struct Account {
    pub key_1: String,
    pub gl_class_code: String,
    pub status: String,
    pub balance: f64,
    pub old_bad_debt_ind: i64,
    pub i_or_b: String,
    pub crm_flag: String,
    pub app_amt: f64,
    pub lmt1: f64,
    pub lmt2: f64,
    pub lmt3: f64,
    pub lmt4: f64,
    pub od_lmt: f64,
    pub adv_val: f64,
    pub basel_class: String,
    pub limit_exp_date: String,
    pub lending_status: i64,
    pub dp: String,
    pub drawing_amt: f64,
    pub od_multi_lim_allow: i64,

    //Derived Fields to Output (Not present in Input)
    pub ccy: String,
    pub group: String,
    pub llg: String,
    pub limit_amt: f64,
    pub dp_amt: f64,
    pub undrawn_sls_amt: f64,
    pub undrawn_lcr_amt: f64,
    pub undrawn_nsfr_amt: f64,
}

impl Account {
    pub fn new(input_acc: Vec<&str>) -> Account {
        Account {
            key_1: get_str(input_acc[0]),
            gl_class_code: get_str(input_acc[1]),
            status: get_str(input_acc[2]),
            //Default value in case of i_or_b = "B"
            balance: input_acc[3].to_string().parse::<f64>().unwrap_or(0.0),
            old_bad_debt_ind: input_acc[4].to_string().parse::<i64>().unwrap_or(0),
            i_or_b: get_str(input_acc[5]),
            crm_flag: get_str(input_acc[6]),
            app_amt: input_acc[7].to_string().parse::<f64>().unwrap_or(0.0),
            lmt1: input_acc[8].to_string().parse::<f64>().unwrap_or(0.0),
            lmt2: input_acc[9].to_string().parse::<f64>().unwrap_or(0.0),
            lmt3: input_acc[10].to_string().parse::<f64>().unwrap_or(0.0),
            lmt4: input_acc[11].to_string().parse::<f64>().unwrap_or(0.0),
            od_lmt: input_acc[12].to_string().parse::<f64>().unwrap_or(0.0),
            adv_val: input_acc[13].to_string().parse::<f64>().unwrap_or(0.0),
            basel_class: get_str(input_acc[14]),
            limit_exp_date: if input_acc[15].is_empty() {
                chrono::NaiveDate::from_ymd(1999, 12, 31)
                    .format("%d-%m-%Y")
                    .to_string()
            } else {
                input_acc[15].to_string()
            },
            lending_status: input_acc[16].to_string().parse::<i64>().unwrap_or(0),
            dp: get_str(input_acc[17]),
            drawing_amt: input_acc[18].to_string().parse::<f64>().unwrap_or(0.0),
            od_multi_lim_allow: input_acc[19].to_string().parse::<i64>().unwrap_or(0),
            ccy: "INR".to_string(),
            group: "NA".to_string(),
            llg: "NA".to_string(),
            limit_amt: 0.0,
            dp_amt: 0.0,
            undrawn_sls_amt: 0.0,
            undrawn_lcr_amt: 0.0,
            undrawn_nsfr_amt: 0.0,
        }
    }
}

pub fn write_output(output_rec: &Account) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
        output_rec.key_1,
        output_rec.gl_class_code.trim(),
        output_rec.status,
        output_rec.balance,
        output_rec.old_bad_debt_ind,
        output_rec.i_or_b,
        output_rec.crm_flag,
        output_rec.app_amt,
        output_rec.lmt1,
        output_rec.lmt2,
        output_rec.lmt3,
        output_rec.lmt4,
        output_rec.od_lmt,
        output_rec.adv_val,
        output_rec.basel_class,
        output_rec.limit_exp_date,
        output_rec.lending_status,
        output_rec.dp,
        output_rec.drawing_amt,
        output_rec.od_multi_lim_allow,
        output_rec.ccy,
        output_rec.group,
        output_rec.llg,
        output_rec.limit_amt,
        output_rec.dp_amt,
        output_rec.undrawn_sls_amt,
        output_rec.undrawn_lcr_amt,
        output_rec.undrawn_nsfr_amt,
    )
}

pub fn get_amts(inp_acc: &mut Account, config_params: &ConfigurationParameters) {
    //Logic for Balance, Limit-Amount, DP-Amount and Undrawn-SLS-Amount when i_or_b = I
    if inp_acc.i_or_b == "I" {
        inp_acc.limit_amt = if inp_acc.od_multi_lim_allow == 2 {
            inp_acc.lmt1 + inp_acc.lmt2 + inp_acc.lmt3 + inp_acc.lmt4
        } else {
            inp_acc.lmt1
        };
        inp_acc.dp_amt = if inp_acc.lending_status.is_in(&[3, 4, 5, 6, 7, 8]) {
            if inp_acc.dp.trim().to_uppercase() == *"Y" {
                min(inp_acc.limit_amt, inp_acc.drawing_amt)
            } else {
                inp_acc.limit_amt
            }
        } else {
            0.0
        };
        if inp_acc.balance < 0.0 {
            inp_acc.balance *= -1.00;
        } else {
            inp_acc.balance = 0.0;
        }
        if rbdate::NaiveDate::parse_from_str(&inp_acc.limit_exp_date, "%d-%m-%Y")
            .unwrap_or(*config_params.as_on_date())
            < *config_params.as_on_date()
        {
            inp_acc.undrawn_sls_amt = 0.0;
        } else if inp_acc.old_bad_debt_ind < 4 && inp_acc.balance <= 0.0 && inp_acc.limit_amt > 0.0
        {
            inp_acc.undrawn_sls_amt = inp_acc.limit_amt;
        } else if inp_acc.old_bad_debt_ind < 4
            && inp_acc.balance > 0.0
            && inp_acc.limit_amt > 0.0
            && inp_acc.limit_amt > inp_acc.balance
        {
            inp_acc.undrawn_sls_amt = inp_acc.limit_amt - inp_acc.balance;
        }
    }
    //Logic for Limit-Amount, DP-Amount and Undrawn-SLS-Amount when i_or_b = B
    if inp_acc.i_or_b == "B" {
        inp_acc.limit_amt = inp_acc.app_amt;
        inp_acc.dp_amt = min(inp_acc.app_amt, inp_acc.drawing_amt);
        inp_acc.undrawn_sls_amt = if inp_acc.old_bad_debt_ind < 4 {
            if inp_acc.app_amt > 0.0 && inp_acc.app_amt <= inp_acc.adv_val {
                0.0
            } else {
                inp_acc.app_amt - inp_acc.adv_val
            }
        } else {
            0.0
        };
    }
}

pub fn get_str(field_val: &str) -> String {
    if field_val.is_empty() {
        "NA".to_string()
    } else {
        field_val.to_string()
    }
}

pub fn min(amt1: f64, amt2: f64) -> f64 {
    if amt1 < amt2 {
        amt1
    } else {
        amt2
    }
}
