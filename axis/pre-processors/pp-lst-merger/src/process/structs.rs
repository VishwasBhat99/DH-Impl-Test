#[derive(Clone, Debug)]
pub struct LstOp {
    pub first_to_amt: String,
    pub prin_amt: f64,
    pub int_amt: f64,
    pub ccyid: String,
    pub p_int_rate: f64,
    pub i_int_rate: f64,
    pub amt_to_last: String,
}

impl LstOp {
    pub fn append_principal_data(&mut self, amt: f64, int_rate: f64) {
        self.prin_amt += amt;
        self.p_int_rate += amt * int_rate;
    }
    pub fn append_interest_data(&mut self, amt: f64, int_rate: f64) {
        self.int_amt += amt;
        self.i_int_rate += amt * int_rate;
    }
}

pub fn get_new_value(fields: Vec<&str>, amt_field: &str) -> LstOp {
    let first_to_amt = format!(
        "{}|{}|{}",
        fields[0], //FlowID
        fields[1], //GrpID
        fields[2]  //LLGID
    );
    let reprice_freq = match fields[6] {
        "BLANK" => "".to_string(),
        _ => fields[6].to_string(),
    };
    let mat_dt = match fields[8] {
        "BLANK" => "31-12-2099".to_string(),
        _ => fields[8].to_string(),
    };
    let reprice_dt = match fields[7] {
        "BLANK" => "31-12-2099".to_string(),
        "01-01-1900" => mat_dt.to_owned(),
        _ => fields[7].to_string(),
    };
    let st_dt = match fields[10] {
        "BLANK" => "01-01-1900".to_string(),
        _ => fields[10].to_string(),
    };
    let mut amt_to_last = format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        reprice_freq,
        reprice_dt,
        mat_dt,
        fields[9], // AcctNum
        st_dt,
        fields[11], // IntrCalFreq
        fields[12], // IsFlotRate
        fields[13], // FlotRateBM
        fields[14], // BUID
        fields[15], // CustID
        fields[16], // CustName
        fields[17], // Sprd
        fields[18], // SchmCode
        fields[19], // MinIR
        fields[20], // MaxIR
    );
    //Since the lst files are varied in field length append additional fields to output if present.
    let mut additional_fields = String::new();
    if fields.len() > 21 {
        for field in fields.iter().skip(21) {
            additional_fields.push_str(field);
            additional_fields.push('|');
        }
    }
    //Finally append extra fields upto 40 positions. At 41th position is the concat value.
    let extra_count = 40 - fields.len();
    for _i in 1..extra_count {
        additional_fields.push('|');
    }
    amt_to_last.push_str(&additional_fields);
    let mut prin_amt = 0.0;
    let mut int_amt = 0.0;
    let mut p_int_rate = 0.0;
    let mut i_int_rate = 0.0;

    let amt_sum = fields[3].to_string().parse::<f64>().unwrap_or(0.0);
    let int_rt = fields[5].to_string().parse::<f64>().unwrap_or(0.0);

    match amt_field {
        "P" => {
            prin_amt = amt_sum;
            p_int_rate = amt_sum * int_rt;
        }
        "I" => {
            int_amt = amt_sum;
            i_int_rate = amt_sum * int_rt;
        }
        _ => {}
    };
    LstOp {
        first_to_amt,
        prin_amt,
        int_amt,
        ccyid: fields[4].to_string(),
        p_int_rate,
        i_int_rate,
        amt_to_last,
    }
}
