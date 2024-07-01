use rbdate::{DateParser, NaiveDate};
use statics::DEFAULT_FLOAT;
use statics::DEFAULT_INT;

#[derive(Debug)]
pub struct InputAccount {
    pub org: String,
    pub logo: String,
    pub acct: String,
    pub loan_nbr: String,
    pub tenure: String,
    pub prin_amt_1: f64,
    pub int_amt_1: f64,
    pub emi_amt_1: f64,
    pub emi_date_1: Option<NaiveDate>,
    pub prin_amt_2: f64,
    pub int_amt_2: f64,
    pub emi_amt_2: f64,
    pub emi_date_2: Option<NaiveDate>,
    pub prin_amt_3: f64,
    pub int_amt_3: f64,
    pub emi_amt_3: f64,
    pub emi_date_3: Option<NaiveDate>,
    pub prin_amt_4: f64,
    pub int_amt_4: f64,
    pub emi_amt_4: f64,
    pub emi_date_4: Option<NaiveDate>,
    pub prin_amt_5: f64,
    pub int_amt_5: f64,
    pub emi_amt_5: f64,
    pub emi_date_5: Option<NaiveDate>,
    pub prin_amt_6: f64,
    pub int_amt_6: f64,
    pub emi_amt_6: f64,
    pub emi_date_6: Option<NaiveDate>,
    pub prin_amt_7: f64,
    pub int_amt_7: f64,
    pub emi_amt_7: f64,
    pub emi_date_7: Option<NaiveDate>,
    pub prin_amt_8: f64,
    pub int_amt_8: f64,
    pub emi_amt_8: f64,
    pub emi_date_8: Option<NaiveDate>,
    pub prin_amt_9: f64,
    pub int_amt_9: f64,
    pub emi_amt_9: f64,
    pub emi_date_9: Option<NaiveDate>,
    pub prin_amt_10: f64,
    pub int_amt_10: f64,
    pub emi_amt_10: f64,
    pub emi_date_10: Option<NaiveDate>,
    pub prin_amt_11: f64,
    pub int_amt_11: f64,
    pub emi_amt_11: f64,
    pub emi_date_11: Option<NaiveDate>,
    pub prin_amt_12: f64,
    pub int_amt_12: f64,
    pub emi_amt_12: f64,
    pub emi_date_12: Option<NaiveDate>,
    pub prin_amt_13: f64,
    pub int_amt_13: f64,
    pub emi_amt_13: f64,
    pub emi_date_13: Option<NaiveDate>,
    pub prin_amt_14: f64,
    pub int_amt_14: f64,
    pub emi_amt_14: f64,
    pub emi_date_14: Option<NaiveDate>,
    pub prin_amt_15: f64,
    pub int_amt_15: f64,
    pub emi_amt_15: f64,
    pub emi_date_15: Option<NaiveDate>,
    pub prin_amt_16: f64,
    pub int_amt_16: f64,
    pub emi_amt_16: f64,
    pub emi_date_16: Option<NaiveDate>,
    pub prin_amt_17: f64,
    pub int_amt_17: f64,
    pub emi_amt_17: f64,
    pub emi_date_17: Option<NaiveDate>,
    pub prin_amt_18: f64,
    pub int_amt_18: f64,
    pub emi_amt_18: f64,
    pub emi_date_18: Option<NaiveDate>,
    pub prin_amt_19: f64,
    pub int_amt_19: f64,
    pub emi_amt_19: f64,
    pub emi_date_19: Option<NaiveDate>,
    pub prin_amt_20: f64,
    pub int_amt_20: f64,
    pub emi_amt_20: f64,
    pub emi_date_20: Option<NaiveDate>,
    pub prin_amt_21: f64,
    pub int_amt_21: f64,
    pub emi_amt_21: f64,
    pub emi_date_21: Option<NaiveDate>,
    pub prin_amt_22: f64,
    pub int_amt_22: f64,
    pub emi_amt_22: f64,
    pub emi_date_22: Option<NaiveDate>,
    pub prin_amt_23: f64,
    pub int_amt_23: f64,
    pub emi_amt_23: f64,
    pub emi_date_23: Option<NaiveDate>,
    pub prin_amt_24: f64,
    pub int_amt_24: f64,
    pub emi_amt_24: f64,
    pub emi_date_24: Option<NaiveDate>,
    pub prin_amt_25: f64,
    pub int_amt_25: f64,
    pub emi_amt_25: f64,
    pub emi_date_25: Option<NaiveDate>,
    pub prin_amt_26: f64,
    pub int_amt_26: f64,
    pub emi_amt_26: f64,
    pub emi_date_26: Option<NaiveDate>,
    pub prin_amt_27: f64,
    pub int_amt_27: f64,
    pub emi_amt_27: f64,
    pub emi_date_27: Option<NaiveDate>,
    pub prin_amt_28: f64,
    pub int_amt_28: f64,
    pub emi_amt_28: f64,
    pub emi_date_28: Option<NaiveDate>,
    pub prin_amt_29: f64,
    pub int_amt_29: f64,
    pub emi_amt_29: f64,
    pub emi_date_29: Option<NaiveDate>,
    pub prin_amt_30: f64,
    pub int_amt_30: f64,
    pub emi_amt_30: f64,
    pub emi_date_30: Option<NaiveDate>,
    pub prin_amt_31: f64,
    pub int_amt_31: f64,
    pub emi_amt_31: f64,
    pub emi_date_31: Option<NaiveDate>,
    pub prin_amt_32: f64,
    pub int_amt_32: f64,
    pub emi_amt_32: f64,
    pub emi_date_32: Option<NaiveDate>,
    pub prin_amt_33: f64,
    pub int_amt_33: f64,
    pub emi_amt_33: f64,
    pub emi_date_33: Option<NaiveDate>,
    pub prin_amt_34: f64,
    pub int_amt_34: f64,
    pub emi_amt_34: f64,
    pub emi_date_34: Option<NaiveDate>,
    pub prin_amt_35: f64,
    pub int_amt_35: f64,
    pub emi_amt_35: f64,
    pub emi_date_35: Option<NaiveDate>,
    pub prin_amt_36: f64,
    pub int_amt_36: f64,
    pub emi_amt_36: f64,
    pub emi_date_36: Option<NaiveDate>,
    pub prin_amt_37: f64,
    pub int_amt_37: f64,
    pub emi_amt_37: f64,
    pub emi_date_37: Option<NaiveDate>,
    pub prin_amt_38: f64,
    pub int_amt_38: f64,
    pub emi_amt_38: f64,
    pub emi_date_38: Option<NaiveDate>,
    pub prin_amt_39: f64,
    pub int_amt_39: f64,
    pub emi_amt_39: f64,
    pub emi_date_39: Option<NaiveDate>,
    pub prin_amt_40: f64,
    pub int_amt_40: f64,
    pub emi_amt_40: f64,
    pub emi_date_40: Option<NaiveDate>,
    pub prin_amt_41: f64,
    pub int_amt_41: f64,
    pub emi_amt_41: f64,
    pub emi_date_41: Option<NaiveDate>,
    pub prin_amt_42: f64,
    pub int_amt_42: f64,
    pub emi_amt_42: f64,
    pub emi_date_42: Option<NaiveDate>,
    pub prin_amt_43: f64,
    pub int_amt_43: f64,
    pub emi_amt_43: f64,
    pub emi_date_43: Option<NaiveDate>,
    pub prin_amt_44: f64,
    pub int_amt_44: f64,
    pub emi_amt_44: f64,
    pub emi_date_44: Option<NaiveDate>,
    pub prin_amt_45: f64,
    pub int_amt_45: f64,
    pub emi_amt_45: f64,
    pub emi_date_45: Option<NaiveDate>,
    pub prin_amt_46: f64,
    pub int_amt_46: f64,
    pub emi_amt_46: f64,
    pub emi_date_46: Option<NaiveDate>,
    pub prin_amt_47: f64,
    pub int_amt_47: f64,
    pub emi_amt_47: f64,
    pub emi_date_47: Option<NaiveDate>,
    pub prin_amt_48: f64,
    pub int_amt_48: f64,
    pub emi_amt_48: f64,
    pub emi_date_48: Option<NaiveDate>,
    pub prin_amt_49: f64,
    pub int_amt_49: f64,
    pub emi_amt_49: f64,
    pub emi_date_49: Option<NaiveDate>,
    pub prin_amt_50: f64,
    pub int_amt_50: f64,
    pub emi_amt_50: f64,
    pub emi_date_50: Option<NaiveDate>,
    pub prin_amt_51: f64,
    pub int_amt_51: f64,
    pub emi_amt_51: f64,
    pub emi_date_51: Option<NaiveDate>,
    pub prin_amt_52: f64,
    pub int_amt_52: f64,
    pub emi_amt_52: f64,
    pub emi_date_52: Option<NaiveDate>,
    pub prin_amt_53: f64,
    pub int_amt_53: f64,
    pub emi_amt_53: f64,
    pub emi_date_53: Option<NaiveDate>,
    pub prin_amt_54: f64,
    pub int_amt_54: f64,
    pub emi_amt_54: f64,
    pub emi_date_54: Option<NaiveDate>,
    pub prin_amt_55: f64,
    pub int_amt_55: f64,
    pub emi_amt_55: f64,
    pub emi_date_55: Option<NaiveDate>,
    pub prin_amt_56: f64,
    pub int_amt_56: f64,
    pub emi_amt_56: f64,
    pub emi_date_56: Option<NaiveDate>,
    pub prin_amt_57: f64,
    pub int_amt_57: f64,
    pub emi_amt_57: f64,
    pub emi_date_57: Option<NaiveDate>,
    pub prin_amt_58: f64,
    pub int_amt_58: f64,
    pub emi_amt_58: f64,
    pub emi_date_58: Option<NaiveDate>,
    pub prin_amt_59: f64,
    pub int_amt_59: f64,
    pub emi_amt_59: f64,
    pub emi_date_59: Option<NaiveDate>,
    pub prin_amt_60: f64,
    pub int_amt_60: f64,
    pub emi_amt_60: f64,
    pub emi_date_60: Option<NaiveDate>,
    pub prin_amt_61: f64,
    pub int_amt_61: f64,
    pub emi_amt_61: f64,
    pub emi_date_61: Option<NaiveDate>,
    pub prin_amt_62: f64,
    pub int_amt_62: f64,
    pub emi_amt_62: f64,
    pub emi_date_62: Option<NaiveDate>,
    pub prin_amt_63: f64,
    pub int_amt_63: f64,
    pub emi_amt_63: f64,
    pub emi_date_63: Option<NaiveDate>,
    pub prin_amt_64: f64,
    pub int_amt_64: f64,
    pub emi_amt_64: f64,
    pub emi_date_64: Option<NaiveDate>,
    pub prin_amt_65: f64,
    pub int_amt_65: f64,
    pub emi_amt_65: f64,
    pub emi_date_65: Option<NaiveDate>,
    pub prin_amt_66: f64,
    pub int_amt_66: f64,
    pub emi_amt_66: f64,
    pub emi_date_66: Option<NaiveDate>,
    pub prin_amt_67: f64,
    pub int_amt_67: f64,
    pub emi_amt_67: f64,
    pub emi_date_67: Option<NaiveDate>,
    pub prin_amt_68: f64,
    pub int_amt_68: f64,
    pub emi_amt_68: f64,
    pub emi_date_68: Option<NaiveDate>,
    pub prin_amt_69: f64,
    pub int_amt_69: f64,
    pub emi_amt_69: f64,
    pub emi_date_69: Option<NaiveDate>,
    pub prin_amt_70: f64,
    pub int_amt_70: f64,
    pub emi_amt_70: f64,
    pub emi_date_70: Option<NaiveDate>,
    pub prin_amt_71: f64,
    pub int_amt_71: f64,
    pub emi_amt_71: f64,
    pub emi_date_71: Option<NaiveDate>,
    pub prin_amt_72: f64,
    pub int_amt_72: f64,
    pub emi_amt_72: f64,
    pub emi_date_72: Option<NaiveDate>,
    pub prin_amt_73: f64,
    pub int_amt_73: f64,
    pub emi_amt_73: f64,
    pub emi_date_73: Option<NaiveDate>,
    pub prin_amt_74: f64,
    pub int_amt_74: f64,
    pub emi_amt_74: f64,
    pub emi_date_74: Option<NaiveDate>,
    pub prin_amt_75: f64,
    pub int_amt_75: f64,
    pub emi_amt_75: f64,
    pub emi_date_75: Option<NaiveDate>,
    pub prin_amt_76: f64,
    pub int_amt_76: f64,
    pub emi_amt_76: f64,
    pub emi_date_76: Option<NaiveDate>,
    pub prin_amt_77: f64,
    pub int_amt_77: f64,
    pub emi_amt_77: f64,
    pub emi_date_77: Option<NaiveDate>,
    pub prin_amt_78: f64,
    pub int_amt_78: f64,
    pub emi_amt_78: f64,
    pub emi_date_78: Option<NaiveDate>,
    pub prin_amt_79: f64,
    pub int_amt_79: f64,
    pub emi_amt_79: f64,
    pub emi_date_79: Option<NaiveDate>,
    pub prin_amt_80: f64,
    pub int_amt_80: f64,
    pub emi_amt_80: f64,
    pub emi_date_80: Option<NaiveDate>,
    pub prin_amt_81: f64,
    pub int_amt_81: f64,
    pub emi_amt_81: f64,
    pub emi_date_81: Option<NaiveDate>,
    pub prin_amt_82: f64,
    pub int_amt_82: f64,
    pub emi_amt_82: f64,
    pub emi_date_82: Option<NaiveDate>,
    pub prin_amt_83: f64,
    pub int_amt_83: f64,
    pub emi_amt_83: f64,
    pub emi_date_83: Option<NaiveDate>,
    pub prin_amt_84: f64,
    pub int_amt_84: f64,
    pub emi_amt_84: f64,
    pub emi_date_84: Option<NaiveDate>,
    pub prin_amt_85: f64,
    pub int_amt_85: f64,
    pub emi_amt_85: f64,
    pub emi_date_85: Option<NaiveDate>,
    pub prin_amt_86: f64,
    pub int_amt_86: f64,
    pub emi_amt_86: f64,
    pub emi_date_86: Option<NaiveDate>,
    pub prin_amt_87: f64,
    pub int_amt_87: f64,
    pub emi_amt_87: f64,
    pub emi_date_87: Option<NaiveDate>,
    pub prin_amt_88: f64,
    pub int_amt_88: f64,
    pub emi_amt_88: f64,
    pub emi_date_88: Option<NaiveDate>,
    pub prin_amt_89: f64,
    pub int_amt_89: f64,
    pub emi_amt_89: f64,
    pub emi_date_89: Option<NaiveDate>,
    pub prin_amt_90: f64,
    pub int_amt_90: f64,
    pub emi_amt_90: f64,
    pub emi_date_90: Option<NaiveDate>,
    pub prin_amt_91: f64,
    pub int_amt_91: f64,
    pub emi_amt_91: f64,
    pub emi_date_91: Option<NaiveDate>,
    pub prin_amt_92: f64,
    pub int_amt_92: f64,
    pub emi_amt_92: f64,
    pub emi_date_92: Option<NaiveDate>,
    pub prin_amt_93: f64,
    pub int_amt_93: f64,
    pub emi_amt_93: f64,
    pub emi_date_93: Option<NaiveDate>,
    pub prin_amt_94: f64,
    pub int_amt_94: f64,
    pub emi_amt_94: f64,
    pub emi_date_94: Option<NaiveDate>,
    pub prin_amt_95: f64,
    pub int_amt_95: f64,
    pub emi_amt_95: f64,
    pub emi_date_95: Option<NaiveDate>,
    pub prin_amt_96: f64,
    pub int_amt_96: f64,
    pub emi_amt_96: f64,
    pub emi_date_96: Option<NaiveDate>,
    pub prin_amt_97: f64,
    pub int_amt_97: f64,
    pub emi_amt_97: f64,
    pub emi_date_97: Option<NaiveDate>,
    pub prin_amt_98: f64,
    pub int_amt_98: f64,
    pub emi_amt_98: f64,
    pub emi_date_98: Option<NaiveDate>,
    pub prin_amt_99: f64,
    pub int_amt_99: f64,
    pub emi_amt_99: f64,
    pub emi_date_99: Option<NaiveDate>,
    pub prin_amt_100: f64,
    pub int_amt_100: f64,
    pub emi_amt_100: f64,
    pub emi_date_100: Option<NaiveDate>,
    pub prin_amt_101: f64,
    pub int_amt_101: f64,
    pub emi_amt_101: f64,
    pub emi_date_101: Option<NaiveDate>,
    pub prin_amt_102: f64,
    pub int_amt_102: f64,
    pub emi_amt_102: f64,
    pub emi_date_102: Option<NaiveDate>,
    pub prin_amt_103: f64,
    pub int_amt_103: f64,
    pub emi_amt_103: f64,
    pub emi_date_103: Option<NaiveDate>,
    pub prin_amt_104: f64,
    pub int_amt_104: f64,
    pub emi_amt_104: f64,
    pub emi_date_104: Option<NaiveDate>,
    pub prin_amt_105: f64,
    pub int_amt_105: f64,
    pub emi_amt_105: f64,
    pub emi_date_105: Option<NaiveDate>,
    pub prin_amt_106: f64,
    pub int_amt_106: f64,
    pub emi_amt_106: f64,
    pub emi_date_106: Option<NaiveDate>,
    pub prin_amt_107: f64,
    pub int_amt_107: f64,
    pub emi_amt_107: f64,
    pub emi_date_107: Option<NaiveDate>,
    pub prin_amt_108: f64,
    pub int_amt_108: f64,
    pub emi_amt_108: f64,
    pub emi_date_108: Option<NaiveDate>,
    pub prin_amt_109: f64,
    pub int_amt_109: f64,
    pub emi_amt_109: f64,
    pub emi_date_109: Option<NaiveDate>,
    pub prin_amt_110: f64,
    pub int_amt_110: f64,
    pub emi_amt_110: f64,
    pub emi_date_110: Option<NaiveDate>,
    pub prin_amt_111: f64,
    pub int_amt_111: f64,
    pub emi_amt_111: f64,
    pub emi_date_111: Option<NaiveDate>,
    pub prin_amt_112: f64,
    pub int_amt_112: f64,
    pub emi_amt_112: f64,
    pub emi_date_112: Option<NaiveDate>,
    pub prin_amt_113: f64,
    pub int_amt_113: f64,
    pub emi_amt_113: f64,
    pub emi_date_113: Option<NaiveDate>,
    pub prin_amt_114: f64,
    pub int_amt_114: f64,
    pub emi_amt_114: f64,
    pub emi_date_114: Option<NaiveDate>,
    pub prin_amt_115: f64,
    pub int_amt_115: f64,
    pub emi_amt_115: f64,
    pub emi_date_115: Option<NaiveDate>,
    pub prin_amt_116: f64,
    pub int_amt_116: f64,
    pub emi_amt_116: f64,
    pub emi_date_116: Option<NaiveDate>,
    pub prin_amt_117: f64,
    pub int_amt_117: f64,
    pub emi_amt_117: f64,
    pub emi_date_117: Option<NaiveDate>,
    pub prin_amt_118: f64,
    pub int_amt_118: f64,
    pub emi_amt_118: f64,
    pub emi_date_118: Option<NaiveDate>,
    pub prin_amt_119: f64,
    pub int_amt_119: f64,
    pub emi_amt_119: f64,
    pub emi_date_119: Option<NaiveDate>,
    pub prin_amt_120: f64,
    pub int_amt_120: f64,
    pub emi_amt_120: f64,
    pub emi_date_120: Option<NaiveDate>,
    pub d_f_coc_emi_date_loan_booked: Option<NaiveDate>,
    pub v_f_coc_emi_loan_src: String,
    pub v_f_coc_emi_tenure: String,
    pub n_f_coc_emi_loan_amount: f64,
    pub v_f_coc_emi_loan_nbr: String,
    pub n_f_coc_emi_out_prin: f64,
    pub n_f_coc_emi_out_int: f64,
    pub total_outstanding: f64,
    pub v_f_coc_emi_rate_of_int: f64,
    pub n_f_coc_emi_org: String,
    pub v_f_coc_emi_chg_type: String,
    pub filler: Option<NaiveDate>,
}

impl<'a> InputAccount {
    pub fn new_from_line(line: &str) -> Result<InputAccount, &'a str> {
        let vect: Vec<&str> = line.split("~#~").collect();
        let mut value_iterator = line.split("~#~");
        let dmy_date_parser = DateParser::new("%d%m%Y".to_string(), false);

        let input_account = InputAccount {
            org: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `hamort_org`.");
                }
            },
            logo: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property 'hamort_logo'."),
            },
            acct: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `hamort_acct`.");
                }
            },
            loan_nbr: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `loan_nbr`.");
                }
            },
            tenure: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `hamort_tenure`."),
            },
            prin_amt_1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_1`.");
                }
            },
            int_amt_1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_1`.");
                }
            },
            emi_amt_1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_1`.");
                }
            },
            emi_date_1: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_1`.");
                }
            },
            prin_amt_2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_2`.");
                }
            },
            int_amt_2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_2`.");
                }
            },
            emi_amt_2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_2`.");
                }
            },
            emi_date_2: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_2`.");
                }
            },
            prin_amt_3: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_3`.");
                }
            },
            int_amt_3: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_3`.");
                }
            },
            emi_amt_3: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_3`.");
                }
            },
            emi_date_3: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_3`.");
                }
            },
            prin_amt_4: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_4`.");
                }
            },
            int_amt_4: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_4`.");
                }
            },
            emi_amt_4: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_4`.");
                }
            },
            emi_date_4: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_4`.");
                }
            },
            prin_amt_5: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_5`.");
                }
            },
            int_amt_5: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_5`.");
                }
            },
            emi_amt_5: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_5`.");
                }
            },
            emi_date_5: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_5`.");
                }
            },
            prin_amt_6: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_6`.");
                }
            },
            int_amt_6: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_6`.");
                }
            },
            emi_amt_6: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_6`.");
                }
            },
            emi_date_6: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_6`.");
                }
            },
            prin_amt_7: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_7`.");
                }
            },
            int_amt_7: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_7`.");
                }
            },
            emi_amt_7: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_7`.");
                }
            },
            emi_date_7: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_7`.");
                }
            },
            prin_amt_8: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_8`.");
                }
            },
            int_amt_8: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_8`.");
                }
            },
            emi_amt_8: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_8`.");
                }
            },
            emi_date_8: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_8`.");
                }
            },
            prin_amt_9: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_9`.");
                }
            },
            int_amt_9: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_9`.");
                }
            },
            emi_amt_9: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_9`.");
                }
            },
            emi_date_9: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_9`.");
                }
            },
            prin_amt_10: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_10`.");
                }
            },
            int_amt_10: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_10`.");
                }
            },
            emi_amt_10: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_10`.");
                }
            },
            emi_date_10: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_10`.");
                }
            },
            prin_amt_11: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_11`.");
                }
            },
            int_amt_11: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_11`.");
                }
            },
            emi_amt_11: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_11`.");
                }
            },
            emi_date_11: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_11`.");
                }
            },
            prin_amt_12: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_12`.");
                }
            },
            int_amt_12: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_12`.");
                }
            },
            emi_amt_12: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_12`.");
                }
            },
            emi_date_12: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_12`.");
                }
            },
            prin_amt_13: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_13`.");
                }
            },
            int_amt_13: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_13`.");
                }
            },
            emi_amt_13: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_13`.");
                }
            },
            emi_date_13: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_13`.");
                }
            },
            prin_amt_14: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_14`.");
                }
            },
            int_amt_14: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_14`.");
                }
            },
            emi_amt_14: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_14`.");
                }
            },
            emi_date_14: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_14`.");
                }
            },
            prin_amt_15: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_14`.");
                }
            },
            int_amt_15: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_15`.");
                }
            },
            emi_amt_15: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_15`.");
                }
            },
            emi_date_15: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_15`.");
                }
            },
            prin_amt_16: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_16`.");
                }
            },
            int_amt_16: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_16`.");
                }
            },
            emi_amt_16: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_16`.");
                }
            },
            emi_date_16: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_16`.");
                }
            },
            prin_amt_17: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_17`.");
                }
            },
            int_amt_17: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_17`.");
                }
            },
            emi_amt_17: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_17`.");
                }
            },
            emi_date_17: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_17`.");
                }
            },
            prin_amt_18: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_18`.");
                }
            },
            int_amt_18: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_18`.");
                }
            },
            emi_amt_18: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_18`.");
                }
            },
            emi_date_18: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_18`.");
                }
            },
            prin_amt_19: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_19`.");
                }
            },
            int_amt_19: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_19`.");
                }
            },
            emi_amt_19: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_19`.");
                }
            },
            emi_date_19: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_19`.");
                }
            },
            prin_amt_20: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_20`.");
                }
            },
            int_amt_20: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_20`.");
                }
            },
            emi_amt_20: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_20`.");
                }
            },
            emi_date_20: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_20`.");
                }
            },
            prin_amt_21: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_21`.");
                }
            },
            int_amt_21: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_21`.");
                }
            },
            emi_amt_21: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_21`.");
                }
            },
            emi_date_21: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_21`.");
                }
            },
            prin_amt_22: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_22`.");
                }
            },
            int_amt_22: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_22`.");
                }
            },
            emi_amt_22: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_22`.");
                }
            },
            emi_date_22: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_22`.");
                }
            },
            prin_amt_23: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_23`.");
                }
            },
            int_amt_23: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_23`.");
                }
            },
            emi_amt_23: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_23`.");
                }
            },
            emi_date_23: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_23`.");
                }
            },
            prin_amt_24: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_24`.");
                }
            },
            int_amt_24: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_24`.");
                }
            },
            emi_amt_24: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_24`.");
                }
            },
            emi_date_24: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_24`.");
                }
            },
            prin_amt_25: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_25`.");
                }
            },
            int_amt_25: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_25`.");
                }
            },
            emi_amt_25: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_25`.");
                }
            },
            emi_date_25: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_25`.");
                }
            },
            prin_amt_26: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_26`.");
                }
            },
            int_amt_26: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_26`.");
                }
            },
            emi_amt_26: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_26`.");
                }
            },
            emi_date_26: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_26`.");
                }
            },
            prin_amt_27: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_27`.");
                }
            },
            int_amt_27: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_27`.");
                }
            },
            emi_amt_27: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_27`.");
                }
            },
            emi_date_27: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_27`.");
                }
            },
            prin_amt_28: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_28`.");
                }
            },
            int_amt_28: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_28`.");
                }
            },
            emi_amt_28: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_28`.");
                }
            },
            emi_date_28: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_28`.");
                }
            },
            prin_amt_29: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_29`.");
                }
            },
            int_amt_29: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_29`.");
                }
            },
            emi_amt_29: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_29`.");
                }
            },
            emi_date_29: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_29`.");
                }
            },
            prin_amt_30: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_30`.");
                }
            },
            int_amt_30: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_30`.");
                }
            },
            emi_amt_30: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_30`.");
                }
            },
            emi_date_30: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_30`.");
                }
            },
            prin_amt_31: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_31`.");
                }
            },
            int_amt_31: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_31`.");
                }
            },
            emi_amt_31: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_31`.");
                }
            },
            emi_date_31: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_31`.");
                }
            },
            prin_amt_32: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_32`.");
                }
            },
            int_amt_32: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_32`.");
                }
            },
            emi_amt_32: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_32`.");
                }
            },
            emi_date_32: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_32`.");
                }
            },
            prin_amt_33: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_33`.");
                }
            },
            int_amt_33: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_33`.");
                }
            },
            emi_amt_33: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_33`.");
                }
            },
            emi_date_33: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_33`.");
                }
            },
            prin_amt_34: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_34`.");
                }
            },
            int_amt_34: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_34`.");
                }
            },
            emi_amt_34: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_34`.");
                }
            },
            emi_date_34: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_34`.");
                }
            },
            prin_amt_35: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_35`.");
                }
            },
            int_amt_35: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_35`.");
                }
            },
            emi_amt_35: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_35`.");
                }
            },
            emi_date_35: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_35`.");
                }
            },
            prin_amt_36: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_36`.");
                }
            },
            int_amt_36: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_36`.");
                }
            },
            emi_amt_36: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_36`.");
                }
            },
            emi_date_36: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_36`.");
                }
            },
            prin_amt_37: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_37`.");
                }
            },
            int_amt_37: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_37`.");
                }
            },
            emi_amt_37: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_3`.");
                }
            },
            emi_date_37: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_37`.");
                }
            },
            prin_amt_38: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_38`.");
                }
            },
            int_amt_38: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_38`.");
                }
            },
            emi_amt_38: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_38`.");
                }
            },
            emi_date_38: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_38`.");
                }
            },
            prin_amt_39: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_39`.");
                }
            },
            int_amt_39: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_39`.");
                }
            },
            emi_amt_39: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_39`.");
                }
            },
            emi_date_39: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_39`.");
                }
            },
            prin_amt_40: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_40`.");
                }
            },
            int_amt_40: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_40`.");
                }
            },
            emi_amt_40: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_40`.");
                }
            },
            emi_date_40: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_40`.");
                }
            },
            prin_amt_41: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_41`.");
                }
            },
            int_amt_41: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_41`.");
                }
            },
            emi_amt_41: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_41`.");
                }
            },
            emi_date_41: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_41`.");
                }
            },
            prin_amt_42: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_42`.");
                }
            },
            int_amt_42: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_42`.");
                }
            },
            emi_amt_42: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_42`.");
                }
            },
            emi_date_42: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_42`.");
                }
            },
            prin_amt_43: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_43`.");
                }
            },
            int_amt_43: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_43`.");
                }
            },
            emi_amt_43: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_43`.");
                }
            },
            emi_date_43: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_43`.");
                }
            },
            prin_amt_44: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_44`.");
                }
            },
            int_amt_44: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_44`.");
                }
            },
            emi_amt_44: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_44`.");
                }
            },
            emi_date_44: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_44`.");
                }
            },
            prin_amt_45: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_45`.");
                }
            },
            int_amt_45: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_45`.");
                }
            },
            emi_amt_45: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_45`.");
                }
            },
            emi_date_45: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_45`.");
                }
            },
            prin_amt_46: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_46`.");
                }
            },
            int_amt_46: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_46`.");
                }
            },
            emi_amt_46: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_46`.");
                }
            },
            emi_date_46: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_46`.");
                }
            },
            prin_amt_47: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_47`.");
                }
            },
            int_amt_47: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_47`.");
                }
            },
            emi_amt_47: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_47`.");
                }
            },
            emi_date_47: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_47`.");
                }
            },
            prin_amt_48: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_48`.");
                }
            },
            int_amt_48: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_48`.");
                }
            },
            emi_amt_48: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_48`.");
                }
            },
            emi_date_48: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_48`.");
                }
            },
            prin_amt_49: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_49`.");
                }
            },
            int_amt_49: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_49`.");
                }
            },
            emi_amt_49: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_49`.");
                }
            },
            emi_date_49: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_49`.");
                }
            },
            prin_amt_50: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_50`.");
                }
            },
            int_amt_50: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_50`.");
                }
            },
            emi_amt_50: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_50`.");
                }
            },
            emi_date_50: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_50`.");
                }
            },
            prin_amt_51: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_51`.");
                }
            },
            int_amt_51: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_51`.");
                }
            },
            emi_amt_51: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_51`.");
                }
            },
            emi_date_51: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_51`.");
                }
            },
            prin_amt_52: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_52`.");
                }
            },
            int_amt_52: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_52`.");
                }
            },
            emi_amt_52: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_52`.");
                }
            },
            emi_date_52: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_52`.");
                }
            },
            prin_amt_53: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_53`.");
                }
            },
            int_amt_53: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_53`.");
                }
            },
            emi_amt_53: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_53`.");
                }
            },
            emi_date_53: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_53`.");
                }
            },
            prin_amt_54: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_54`.");
                }
            },
            int_amt_54: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_54`.");
                }
            },
            emi_amt_54: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_54`.");
                }
            },
            emi_date_54: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_54`.");
                }
            },
            prin_amt_55: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_55`.");
                }
            },
            int_amt_55: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_55`.");
                }
            },
            emi_amt_55: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_55`.");
                }
            },
            emi_date_55: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_55`.");
                }
            },
            prin_amt_56: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_56`.");
                }
            },
            int_amt_56: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_56`.");
                }
            },
            emi_amt_56: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_56`.");
                }
            },
            emi_date_56: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_56`.");
                }
            },
            prin_amt_57: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_57`.");
                }
            },
            int_amt_57: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_57`.");
                }
            },
            emi_amt_57: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_57`.");
                }
            },
            emi_date_57: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_57`.");
                }
            },
            prin_amt_58: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_58`.");
                }
            },
            int_amt_58: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_58`.");
                }
            },
            emi_amt_58: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_58`.");
                }
            },
            emi_date_58: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_58`.");
                }
            },
            prin_amt_59: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_59`.");
                }
            },
            int_amt_59: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_59`.");
                }
            },
            emi_amt_59: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_59`.");
                }
            },
            emi_date_59: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_59`.");
                }
            },
            prin_amt_60: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_60`.");
                }
            },
            int_amt_60: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_60`.");
                }
            },
            emi_amt_60: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_60`.");
                }
            },
            emi_date_60: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_60`.");
                }
            },
            prin_amt_61: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_61`.");
                }
            },
            int_amt_61: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_61`.");
                }
            },
            emi_amt_61: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_61`.");
                }
            },
            emi_date_61: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_61`.");
                }
            },
            prin_amt_62: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_62`.");
                }
            },
            int_amt_62: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_62`.");
                }
            },
            emi_amt_62: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_62`.");
                }
            },
            emi_date_62: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_62`.");
                }
            },
            prin_amt_63: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_63`.");
                }
            },
            int_amt_63: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_63`.");
                }
            },
            emi_amt_63: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_63`.");
                }
            },
            emi_date_63: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_63`.");
                }
            },
            prin_amt_64: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_64`.");
                }
            },
            int_amt_64: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_64`.");
                }
            },
            emi_amt_64: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_64`.");
                }
            },
            emi_date_64: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_64`.");
                }
            },
            prin_amt_65: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_65`.");
                }
            },
            int_amt_65: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_65`.");
                }
            },
            emi_amt_65: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_65`.");
                }
            },
            emi_date_65: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_65`.");
                }
            },
            prin_amt_66: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_66`.");
                }
            },
            int_amt_66: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_66`.");
                }
            },
            emi_amt_66: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_66`.");
                }
            },
            emi_date_66: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_66`.");
                }
            },
            prin_amt_67: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_67`.");
                }
            },
            int_amt_67: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_67`.");
                }
            },
            emi_amt_67: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_67`.");
                }
            },
            emi_date_67: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_67`.");
                }
            },
            prin_amt_68: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_68`.");
                }
            },
            int_amt_68: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_68`.");
                }
            },
            emi_amt_68: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_68`.");
                }
            },
            emi_date_68: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_68`.");
                }
            },
            prin_amt_69: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_69`.");
                }
            },
            int_amt_69: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_69`.");
                }
            },
            emi_amt_69: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_69`.");
                }
            },
            emi_date_69: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_69`.");
                }
            },
            prin_amt_70: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_70`.");
                }
            },
            int_amt_70: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_70`.");
                }
            },
            emi_amt_70: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_70`.");
                }
            },
            emi_date_70: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_70`.");
                }
            },
            prin_amt_71: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_71`.");
                }
            },
            int_amt_71: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_71`.");
                }
            },
            emi_amt_71: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_71`.");
                }
            },
            emi_date_71: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_71`.");
                }
            },
            prin_amt_72: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_72`.");
                }
            },
            int_amt_72: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_72`.");
                }
            },
            emi_amt_72: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_72`.");
                }
            },
            emi_date_72: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_72`.");
                }
            },
            prin_amt_73: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_73`.");
                }
            },
            int_amt_73: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_73`.");
                }
            },
            emi_amt_73: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_73`.");
                }
            },
            emi_date_73: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_73`.");
                }
            },
            prin_amt_74: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_74`.");
                }
            },
            int_amt_74: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_74`.");
                }
            },
            emi_amt_74: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_74`.");
                }
            },
            emi_date_74: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_74`.");
                }
            },
            prin_amt_75: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_75`.");
                }
            },
            int_amt_75: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_75`.");
                }
            },
            emi_amt_75: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_75`.");
                }
            },
            emi_date_75: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_75`.");
                }
            },
            prin_amt_76: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_76`.");
                }
            },
            int_amt_76: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_76`.");
                }
            },
            emi_amt_76: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_76`.");
                }
            },
            emi_date_76: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_76`.");
                }
            },
            prin_amt_77: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_77`.");
                }
            },
            int_amt_77: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_77`.");
                }
            },
            emi_amt_77: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_77`.");
                }
            },
            emi_date_77: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_77`.");
                }
            },
            prin_amt_78: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_78`.");
                }
            },
            int_amt_78: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_78`.");
                }
            },
            emi_amt_78: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_78`.");
                }
            },
            emi_date_78: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_78`.");
                }
            },
            prin_amt_79: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_79`.");
                }
            },
            int_amt_79: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_79`.");
                }
            },
            emi_amt_79: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_79`.");
                }
            },
            emi_date_79: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_79`.");
                }
            },
            prin_amt_80: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_80`.");
                }
            },
            int_amt_80: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_80`.");
                }
            },
            emi_amt_80: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_80`.");
                }
            },
            emi_date_80: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_80`.");
                }
            },
            prin_amt_81: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_81`.");
                }
            },
            int_amt_81: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_81`.");
                }
            },
            emi_amt_81: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_81`.");
                }
            },
            emi_date_81: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_81`.");
                }
            },
            prin_amt_82: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_82`.");
                }
            },
            int_amt_82: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_82`.");
                }
            },
            emi_amt_82: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_82`.");
                }
            },
            emi_date_82: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_82`.");
                }
            },
            prin_amt_83: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_83`.");
                }
            },
            int_amt_83: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_83`.");
                }
            },
            emi_amt_83: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_83`.");
                }
            },
            emi_date_83: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_83`.");
                }
            },
            prin_amt_84: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_84`.");
                }
            },
            int_amt_84: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_84`.");
                }
            },
            emi_amt_84: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_84`.");
                }
            },
            emi_date_84: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_84`.");
                }
            },
            prin_amt_85: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_85`.");
                }
            },
            int_amt_85: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_85`.");
                }
            },
            emi_amt_85: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_85`.");
                }
            },
            emi_date_85: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_85`.");
                }
            },
            prin_amt_86: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_86`.");
                }
            },
            int_amt_86: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_86`.");
                }
            },
            emi_amt_86: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_86`.");
                }
            },
            emi_date_86: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_86`.");
                }
            },
            prin_amt_87: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_87`.");
                }
            },
            int_amt_87: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_87`.");
                }
            },
            emi_amt_87: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_87`.");
                }
            },
            emi_date_87: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_87`.");
                }
            },
            prin_amt_88: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_88`.");
                }
            },
            int_amt_88: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_88`.");
                }
            },
            emi_amt_88: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_88`.");
                }
            },
            emi_date_88: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_88`.");
                }
            },
            prin_amt_89: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_89`.");
                }
            },
            int_amt_89: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_89`.");
                }
            },
            emi_amt_89: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_89`.");
                }
            },
            emi_date_89: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_89`.");
                }
            },
            prin_amt_90: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_90`.");
                }
            },
            int_amt_90: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_90`.");
                }
            },
            emi_amt_90: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_90`.");
                }
            },
            emi_date_90: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_90`.");
                }
            },
            prin_amt_91: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_91`.");
                }
            },
            int_amt_91: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_91`.");
                }
            },
            emi_amt_91: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_91`.");
                }
            },
            emi_date_91: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_91`.");
                }
            },
            prin_amt_92: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_92`.");
                }
            },
            int_amt_92: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_92`.");
                }
            },
            emi_amt_92: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_92`.");
                }
            },
            emi_date_92: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_92`.");
                }
            },
            prin_amt_93: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_93`.");
                }
            },
            int_amt_93: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_93`.");
                }
            },
            emi_amt_93: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_93`.");
                }
            },
            emi_date_93: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_93`.");
                }
            },
            prin_amt_94: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_94`.");
                }
            },
            int_amt_94: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_94`.");
                }
            },
            emi_amt_94: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_94`.");
                }
            },
            emi_date_94: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_94`.");
                }
            },
            prin_amt_95: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_95`.");
                }
            },
            int_amt_95: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_95`.");
                }
            },
            emi_amt_95: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_95`.");
                }
            },
            emi_date_95: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_95`.");
                }
            },
            prin_amt_96: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_96`.");
                }
            },
            int_amt_96: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_96`.");
                }
            },
            emi_amt_96: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_96`.");
                }
            },
            emi_date_96: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_96`.");
                }
            },
            prin_amt_97: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_97`.");
                }
            },
            int_amt_97: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_97`.");
                }
            },
            emi_amt_97: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_97`.");
                }
            },
            emi_date_97: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_97`.");
                }
            },
            prin_amt_98: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_98`.");
                }
            },
            int_amt_98: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_98`.");
                }
            },
            emi_amt_98: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_98`.");
                }
            },
            emi_date_98: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_98`.");
                }
            },
            prin_amt_99: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_99`.");
                }
            },
            int_amt_99: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_99`.");
                }
            },
            emi_amt_99: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_99`.");
                }
            },
            emi_date_99: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_99`.");
                }
            },
            prin_amt_100: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_100`.");
                }
            },
            int_amt_100: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_100`.");
                }
            },
            emi_amt_100: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_100`.");
                }
            },
            emi_date_100: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_100`.");
                }
            },
            prin_amt_101: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_101`.");
                }
            },
            int_amt_101: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_101`.");
                }
            },
            emi_amt_101: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_101`.");
                }
            },
            emi_date_101: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_101`.");
                }
            },
            prin_amt_102: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_102`.");
                }
            },
            int_amt_102: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_102`.");
                }
            },
            emi_amt_102: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_102`.");
                }
            },
            emi_date_102: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_102`.");
                }
            },
            prin_amt_103: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_103`.");
                }
            },
            int_amt_103: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_103`.");
                }
            },
            emi_amt_103: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_103`.");
                }
            },
            emi_date_103: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_103`.");
                }
            },
            prin_amt_104: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_104`.");
                }
            },
            int_amt_104: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_104`.");
                }
            },
            emi_amt_104: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_104`.");
                }
            },
            emi_date_104: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_104`.");
                }
            },
            prin_amt_105: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_105`.");
                }
            },
            int_amt_105: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_105`.");
                }
            },
            emi_amt_105: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_105`.");
                }
            },
            emi_date_105: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_105`.");
                }
            },
            prin_amt_106: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_106`.");
                }
            },
            int_amt_106: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_106`.");
                }
            },
            emi_amt_106: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_106`.");
                }
            },
            emi_date_106: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_106`.");
                }
            },
            prin_amt_107: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_107`.");
                }
            },
            int_amt_107: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_107`.");
                }
            },
            emi_amt_107: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_107`.");
                }
            },
            emi_date_107: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_107`.");
                }
            },
            prin_amt_108: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_108`.");
                }
            },
            int_amt_108: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_108`.");
                }
            },
            emi_amt_108: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_108`.");
                }
            },
            emi_date_108: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_108`.");
                }
            },
            prin_amt_109: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_109`.");
                }
            },
            int_amt_109: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_109`.");
                }
            },
            emi_amt_109: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_109`.");
                }
            },
            emi_date_109: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_109`.");
                }
            },
            prin_amt_110: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_110`.");
                }
            },
            int_amt_110: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_110`.");
                }
            },
            emi_amt_110: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_110`.");
                }
            },
            emi_date_110: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_110`.");
                }
            },
            prin_amt_111: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_111`.");
                }
            },
            int_amt_111: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_111`.");
                }
            },
            emi_amt_111: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_111`.");
                }
            },
            emi_date_111: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_111`.");
                }
            },
            prin_amt_112: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_112`.");
                }
            },
            int_amt_112: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_112`.");
                }
            },
            emi_amt_112: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_112`.");
                }
            },
            emi_date_112: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_112`.");
                }
            },
            prin_amt_113: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_113`.");
                }
            },
            int_amt_113: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_113`.");
                }
            },
            emi_amt_113: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_113`.");
                }
            },
            emi_date_113: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_113`.");
                }
            },
            prin_amt_114: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_114`.");
                }
            },
            int_amt_114: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_114`.");
                }
            },
            emi_amt_114: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_114`.");
                }
            },
            emi_date_114: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_114`.");
                }
            },
            prin_amt_115: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_115`.");
                }
            },
            int_amt_115: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_115`.");
                }
            },
            emi_amt_115: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_115`.");
                }
            },
            emi_date_115: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_115`.");
                }
            },
            prin_amt_116: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_116`.");
                }
            },
            int_amt_116: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_116`.");
                }
            },
            emi_amt_116: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_116`.");
                }
            },
            emi_date_116: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_116`.");
                }
            },
            prin_amt_117: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_117`.");
                }
            },
            int_amt_117: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_117`.");
                }
            },
            emi_amt_117: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_117`.");
                }
            },
            emi_date_117: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_117`.");
                }
            },
            prin_amt_118: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_118`.");
                }
            },
            int_amt_118: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_118`.");
                }
            },
            emi_amt_118: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_118`.");
                }
            },
            emi_date_118: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_118`.");
                }
            },
            prin_amt_119: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_119`.");
                }
            },
            int_amt_119: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_119`.");
                }
            },
            emi_amt_119: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_119`.");
                }
            },
            emi_date_119: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_119`.");
                }
            },
            prin_amt_120: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `prin_amt_120`.");
                }
            },
            int_amt_120: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `int_amt_120`.");
                }
            },
            emi_amt_120: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `emi_amt_120`.");
                }
            },
            emi_date_120: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `emi_date_120`.");
                }
            },
            d_f_coc_emi_date_loan_booked: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `d_f_coc_emi_date_loan_booked`.");
                }
            },
            v_f_coc_emi_loan_src: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `v_f_coc_emi_loan_src`."),
            },
            v_f_coc_emi_tenure: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `hamort_tenure`."),
            },
            n_f_coc_emi_loan_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `n_f_coc_emi_loan_amount`.");
                }
            },
            v_f_coc_emi_loan_nbr: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property `v_f_coc_emi_loan_nbr`."),
            },
            n_f_coc_emi_out_prin: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `n_f_coc_emi_out_prin`.");
                }
            },
            n_f_coc_emi_out_int: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `n_f_coc_emi_out_int`.");
                }
            },
            total_outstanding: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `total_outstanding`.");
                }
            },
            n_f_coc_emi_org: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `n_f_coc_emi_org`.");
                }
            },
            v_f_coc_emi_chg_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => return Err("Could not parse property 'v_f_coc_emi_chg_type'."),
            },
            v_f_coc_emi_rate_of_int: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `v_f_coc_emi_rate_of_int`.");
                }
            },
            filler: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `filler`.");
                }
            },
        };

        Ok(input_account)
    }
}
