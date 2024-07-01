use super::manual_handler::remove_comma;
use rbdate::DateParser;
use rbdate::NaiveDate;
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub entity: String,
    pub trade_id: String,
    pub et_trade_id: String,
    pub contractid: String,
    pub root_contract_id: String,
    pub package_id: String,
    pub contract_typology: String,
    pub trade_typology: String,
    pub usage: String,
    pub desk: String,
    pub book: String,
    pub folder: String,
    pub trading_banking: String,
    pub internal_external: String,
    pub inter_entity: String,
    pub counterparty_group_code: String,
    pub counterparty_parent_code: String,
    pub counterparty_child_code: String,
    pub counterparty_long_name: String,
    pub cp_category_level_1: String,
    pub cp_category_level_2: String,
    pub cp_category_level_3: String,
    pub near_leg_far_leg: String,
    pub insertion_date: Option<NaiveDate>,
    pub trade_date: Option<NaiveDate>,
    pub option_start_date: Option<NaiveDate>,
    pub option_end_date: Option<NaiveDate>,
    pub effective_date: Option<NaiveDate>,
    pub mat_date_of_contract: Option<NaiveDate>,
    pub mat_date_of_trade: Option<NaiveDate>,
    pub ccil_guranteed: String,
    pub currency_pair: String,
    pub buy_currency: String,
    pub buy_current_notional: f64,
    pub sell_currency: String,
    pub sell_current_notional: f64,
    pub deal_rate: f64,
    pub buy_original_notional: f64,
    pub sell_original_notional: f64,
    pub deal_status: String,
    pub last_event: String,
    pub last_event_date: Option<NaiveDate>,
    pub country_of_residence: String,
    pub country_of_ultimate_risk: String,
    pub spot_rate_leg_1_inr_usd: f64,
    pub spot_rate_leg_2_inr_usd: f64,
    pub mtm_currency_pl_currency: String,
    pub forward_rate: f64,
    pub forward_mtm: f64,
    pub mtm_in_inr_forward: f64,
    pub mtm_in_usd_forward: f64,
    pub pv_mtm_in_inr: f64,
    pub pv_mtm_in_usd: f64,
    pub pv01_in_inr: f64,
    pub mduriation_ccy1_inr: f64,
    pub mduriation_ccy2_inr: f64,
    pub contingent_notional_in_inr: f64,
    pub cva: f64,
    pub dva: f64,
    pub bcva: f64,
    pub bcva_adjusted_mtm: f64,
    pub npa: String,
    pub bank_nonbank: String,
    pub original_tenor: i64,
    pub residual_tenor: i64,
    pub deal_underlying: String,
    pub input_id: String,
    pub authoriser_id: String,
    pub comment: String,
    pub insdustry_type: String,
    pub type_b_s: String,
    pub broker: String,
    pub brokerage_amount: f64,
    pub deal_mode: String,
    pub settlement_mode: String,
    pub lcy_equivalent: f64,
    pub cover_rate: f64,
    pub instrument_id_portfolio_id: String,
    pub lei_no: String,
    pub lei_next_renewal_date: Option<NaiveDate>,
    pub shf_udf: String,
    pub uti_name_space_issuer_code: String,
    pub uti_unique_transaction_identifier: String,
    pub upi_unique_product_identifier: String,
    pub effective_date_maturity_date: i64,
    pub reporting_date: Option<NaiveDate>,
    pub fcnr_name: String,
    pub fcnr_fd1: String,
    pub fcnr_fd2: String,
    pub fcnr_fd3: String,
    pub fcnr_fd4: String,
    pub fcnr_fd5: String,
    pub fcnr_fd6: String,
    pub fcnr_fd7: String,
    pub fcnr_fd8: String,
    pub fcnr_fd9: String,
    pub fcnr_fd10: String,
    pub fcnr_csm: String,
    pub arbfund_ln: String,
    pub res_roll: String,
    pub wr_ctpy: String,
    pub conso_ref: String,
    pub purpose: String,
    pub rev_orig: String,
    pub swap_cost: String,
    pub fedai_ext: String,
    pub rl_orig_dt: String,
    pub fedai_dt: String,
    pub npv1_currency: String,
    pub npv2_currency: String,
    pub npv1: f64,
    pub npv2: f64,
    pub mlc_ntgcnt: i64,
    pub outst_amt: f64,
    pub outst_amtr: f64,
    pub package_typology: String,
    pub ae_id_field: String,
    pub ccy_waiver: String,
    pub usd_10_mn_client: String,
    pub ndf_settlementcurrency: String,
    pub ndf_settlementamount: f64,
    pub ndf_fixingdate: Option<NaiveDate>,
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split(';');
        let input_account = InputAccount {
            entity: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `entity`.");
                }
            },
            trade_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `trade_id`.");
                }
            },
            et_trade_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `contract_id`.");
                }
            },
            contractid: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `contractid`.");
                }
            },
            root_contract_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `root_contract_id`.");
                }
            },
            package_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `package_id`.");
                }
            },
            contract_typology: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `contract_typology`.");
                }
            },
            trade_typology: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `trade_typology`.");
                }
            },
            usage: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `usage`.");
                }
            },
            desk: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `desk`.");
                }
            },
            book: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `book`.");
                }
            },
            folder: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `folder`.");
                }
            },
            trading_banking: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `trading_banking`.");
                }
            },
            internal_external: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `internal_external`.");
                }
            },
            inter_entity: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `inter_entity`.");
                }
            },
            counterparty_group_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `counterparty_group_code`.");
                }
            },
            counterparty_parent_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `counterparty_parent_code`.");
                }
            },
            counterparty_child_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `counterparty_child_code`.");
                }
            },
            counterparty_long_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `counterparty_long_name`.");
                }
            },
            cp_category_level_1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cp_category_level_1`.");
                }
            },
            cp_category_level_2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cp_category_level_2`.");
                }
            },
            cp_category_level_3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `cp_category_level_3`.");
                }
            },
            near_leg_far_leg: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `near_leg_far_leg`.");
                }
            },
            insertion_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `insertion_date`.");
                }
            },
            trade_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `trade_date`.");
                }
            },
            option_start_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `option_start_date`.");
                }
            },
            option_end_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `option_end_date`.");
                }
            },
            effective_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `effective_date`.");
                }
            },
            mat_date_of_contract: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `mat_date_of_contract`.");
                }
            },
            mat_date_of_trade: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `mat_date_of_trade`.");
                }
            },
            ccil_guranteed: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ccil_guranteed`.");
                }
            },
            currency_pair: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `currency_pair`.");
                }
            },
            buy_currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `buy_currency`.");
                }
            },
            buy_current_notional: match value_iterator.next() {
                Some(val) => remove_comma(val).parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `buy_current_notional`.");
                }
            },
            sell_currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sell_currency`.");
                }
            },
            sell_current_notional: match value_iterator.next() {
                Some(val) => remove_comma(val).parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `sell_current_notional`.");
                }
            },
            deal_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `deal_rate`.");
                }
            },
            buy_original_notional: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `buy_original_notional`.");
                }
            },
            sell_original_notional: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `sell_original_notional`.");
                }
            },
            deal_status: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deal_status`.");
                }
            },
            last_event: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `last_event`.");
                }
            },
            last_event_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `last_event_date`.");
                }
            },
            country_of_residence: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `country_of_residence`.");
                }
            },
            country_of_ultimate_risk: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `country_of_ultimate_risk`.");
                }
            },
            spot_rate_leg_1_inr_usd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `spot_rate_leg_1_inr_usd`.");
                }
            },
            spot_rate_leg_2_inr_usd: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `spot_rate_leg_2_inr_usd`.");
                }
            },
            mtm_currency_pl_currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `mtm_currency_pl_currency`.");
                }
            },
            forward_rate: match value_iterator.next() {
                Some(val) => remove_comma(val).parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `forward_rate`.");
                }
            },
            forward_mtm: match value_iterator.next() {
                Some(val) => remove_comma(val).parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `forward_mtm`.");
                }
            },
            mtm_in_inr_forward: match value_iterator.next() {
                Some(val) => remove_comma(val).parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `mtm_in_inr_forward`.");
                }
            },
            mtm_in_usd_forward: match value_iterator.next() {
                Some(val) => remove_comma(val).parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `mtm_in_usd_forward`.");
                }
            },
            pv_mtm_in_inr: match value_iterator.next() {
                Some(val) => remove_comma(val).parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `pv_mtm_in_inr`.");
                }
            },
            pv_mtm_in_usd: match value_iterator.next() {
                Some(val) => remove_comma(val).parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `pv_mtm_in_usd`.");
                }
            },
            pv01_in_inr: match value_iterator.next() {
                Some(val) => remove_comma(val).parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `pv01_in_inr`.");
                }
            },
            mduriation_ccy1_inr: match value_iterator.next() {
                Some(val) => remove_comma(val).parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `mduriation_ccy1_inr`.");
                }
            },
            mduriation_ccy2_inr: match value_iterator.next() {
                Some(val) => remove_comma(val).parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `mduriation_ccy2_inr`.");
                }
            },
            contingent_notional_in_inr: match value_iterator.next() {
                Some(val) => remove_comma(val).parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `contingent_notional_in_inr`.");
                }
            },
            cva: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `cva`.");
                }
            },
            dva: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `dva`.");
                }
            },
            bcva: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `bcva`.");
                }
            },
            bcva_adjusted_mtm: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `bcva_adjusted_mtm`.");
                }
            },
            npa: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `npa`.");
                }
            },
            bank_nonbank: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `bank_nonbank`.");
                }
            },
            original_tenor: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `original_tenor`.");
                }
            },
            residual_tenor: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `residual_tenor`.");
                }
            },
            deal_underlying: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deal_underlying`.");
                }
            },
            input_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `input_id`.");
                }
            },
            authoriser_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `authoriser_id`.");
                }
            },
            comment: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `comment`.");
                }
            },
            insdustry_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `insdustry_type`.");
                }
            },
            type_b_s: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `type_b_s`.");
                }
            },
            broker: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `broker`.");
                }
            },
            brokerage_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `brokerage_amount`.");
                }
            },
            deal_mode: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `deal_mode`.");
                }
            },
            settlement_mode: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `settlement_mode`.");
                }
            },
            lcy_equivalent: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `lcy_equivalent`.");
                }
            },
            cover_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `cover_rate`.");
                }
            },
            instrument_id_portfolio_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `instrument_id_portfolio_id`.");
                }
            },
            lei_no: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `lei_no`.");
                }
            },
            lei_next_renewal_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `lei_next_renewal_date`.");
                }
            },
            shf_udf: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `shf_udf`.");
                }
            },
            uti_name_space_issuer_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `uti_name_space_issuer_code`.");
                }
            },
            uti_unique_transaction_identifier: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `uti_unique_transaction_identifier`.");
                }
            },
            upi_unique_product_identifier: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `upi_unique_product_identifier`.");
                }
            },
            effective_date_maturity_date: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `effective_date_maturity_date`.");
                }
            },
            reporting_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `reporting_date`.");
                }
            },
            fcnr_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `fcnr_name`.");
                }
            },
            fcnr_fd1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `fcnr_fd1`.");
                }
            },
            fcnr_fd2: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `fcnr_fd2`.");
                }
            },
            fcnr_fd3: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `fcnr_fd3`.");
                }
            },
            fcnr_fd4: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `fcnr_fd4`.");
                }
            },
            fcnr_fd5: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `fcnr_fd5`.");
                }
            },
            fcnr_fd6: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `fcnr_fd6`.");
                }
            },
            fcnr_fd7: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `fcnr_fd7`.");
                }
            },
            fcnr_fd8: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `fcnr_fd8`.");
                }
            },
            fcnr_fd9: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `fcnr_fd9`.");
                }
            },
            fcnr_fd10: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `fcnr_fd10`.");
                }
            },
            fcnr_csm: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `fcnr_csm`.");
                }
            },
            arbfund_ln: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `arbfund_ln`.");
                }
            },
            res_roll: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `res_roll`.");
                }
            },
            wr_ctpy: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `wr_ctpy`.");
                }
            },
            conso_ref: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `conso_ref`.");
                }
            },
            purpose: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `purpose`.");
                }
            },
            rev_orig: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rev_orig`.");
                }
            },
            swap_cost: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `swap_cost`.");
                }
            },
            fedai_ext: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `fedai_ext`.");
                }
            },
            rl_orig_dt: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `rl_orig_dt`.");
                }
            },
            fedai_dt: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `fedai_dt`.");
                }
            },
            npv1_currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `npv1_currency`.");
                }
            },
            npv2_currency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `npv2_currency`.");
                }
            },
            npv1: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `npv1`.");
                }
            },
            npv2: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `npv2`.");
                }
            },
            mlc_ntgcnt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_INT),
                None => {
                    return Err("Could not parse property `mlc_ntgcnt`.");
                }
            },
            outst_amt: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `outst_amt`.");
                }
            },
            outst_amtr: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `outst_amtr`.");
                }
            },
            package_typology: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `package_typology`.");
                }
            },
            ae_id_field: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ae_id_field`.");
                }
            },
            ccy_waiver: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ccy_waiver`.");
                }
            },
            usd_10_mn_client: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `usd_10_mn_client`.");
                }
            },
            ndf_settlementcurrency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ndf_settlementcurrency`.");
                }
            },
            ndf_settlementamount: match value_iterator.next() {
                Some(val) => remove_comma(val).parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ndf_settlementamount`.");
                }
            },
            ndf_fixingdate: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `ndf_fixingdate`.");
                }
            },
        };
        Ok(input_account)
    }
}
