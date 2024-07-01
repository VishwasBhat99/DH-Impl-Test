use rbdate::{DateParser, NaiveDate};
use statics::*;

#[derive(Debug, Clone)]
pub struct InputAccount {
    pub portfolio: String,
    pub security_name: String,
    pub instrument_id: String,
    pub instrument_type: String,
    pub isin_code: String,
    pub slr_nslr: String,
    pub category: String,
    pub category_grp: String,
    pub sub_category: String,
    pub fa_classification: String,
    pub maturity_date: Option<NaiveDate>,
    pub coupon: f64,
    pub discount_intr_rate: f64,
    pub face_val_per_units: f64,
    pub quantity: f64,
    pub face_value: f64,
    pub wap: f64,
    pub yield_value: f64,
    pub book_value: f64,
    pub mvmnt_amount: f64,
    pub market_price: f64,
    pub market_yield: f64,
    pub market_value: f64,
    pub duration: f64,
    pub m_duration: f64,
    pub appreciation: f64,
    pub depreciation: f64,
    pub net_app_dep: f64,
    pub convexity: f64,
    pub pvbp: f64,
    pub absolute_pvbp: f64,
    pub amortization_asondate: f64,
    pub accounted_amortization: f64,
    pub unaccounted_amortization: f64,
    pub accrued_interest: f64,
    pub no_of_ca_skipped: String,
    pub ca_interest_not_receieved: String,
    pub total_interest: f64,
    pub encumbered_since_repo: String,
    pub amount_repo: f64,
    pub encumbered_since_ccil: String,
    pub amount_ccil: f64,
    pub encumbered_since_treps: String,
    pub amount_treps: f64,
    pub encumbered_since_mcxs: String,
    pub amount_mcxs: f64,
    pub encumbered_since_others: String,
    pub amount_others: f64,
    pub custody_pos_number: String,
    pub custody_pos_type: String,
    pub interest_frequency: String,
    pub interest_practice: String,
    pub put_date: Option<NaiveDate>,
    pub call_date: Option<NaiveDate>,
    pub last_coupon_date: Option<NaiveDate>,
    pub next_coupon_date: Option<NaiveDate>,
    pub issue_date: Option<NaiveDate>,
    pub place: String,
    pub country: String,
    pub booking_basis: String,
    pub residual_maturity: f64,
    pub issuer_name: String,
    pub market: String,
    pub gurantor: String,
    pub industry: String,
    pub sub_industry: String,
    pub borrower_category: String,
    pub asset_classification: String,
    pub asset_type: String,
    pub asset_category: String,
    pub old_security_id: String,
    pub curve1: String,
    pub listed: String,
    pub secured: String,
    pub quoted: String,
    pub borrower: String,
    pub extbank_ref: String,
    pub pan: String,
    pub intr_rating_agency: String,
    pub internal_rating: String,
    pub intr_rating_valid_from: Option<NaiveDate>,
    pub intr_rating_valid_till: Option<NaiveDate>,
    pub extrn_rating_agency: String,
    pub external_rating: String,
    pub extrn_rating_valid_from: Option<NaiveDate>,
    pub extrn_rating_valid_till: Option<NaiveDate>,
    pub liquid_status: String,
    pub asset_sub_class: String,
    pub hurdle_rating: String,
    pub external_rating_vs_hurdle: String,
    pub internal_rating_vs_hurdle: String,
    pub fsu: String,
    pub equity_seg: String,
    pub issuer_segr: String,
    pub restructuring: String,
    pub paid_up_share_captial: String,
    pub exempted_amount: f64,
    pub issuer_group: String,
    pub murram_market_value: f64,
    pub murram_depr: f64,
    pub var_settled_bv: f64,
    pub var_unsettled_bv: f64,
    pub var_settled_amount: f64,
    pub var_unsettled_amount: f64,
    pub kri_settled_qtd_fv: f64,
    pub basel_group: String,
    pub basel_sub_group: String,
    pub time_band: String,
    pub capital_charge_market_risk_rate: String,
    pub capital_charge_market_risk_amount: f64,
    pub trading_specif_risk_rate: f64,
    pub banking_specif_risk_rate: f64,
    pub trading_specif_risk_captial_charge: f64,
    pub banking_specif_risk_captial_charge: f64,
    pub mode_of_holding: String,
    pub issuer_rating_agency: String,
    pub issuer_rating: String,
    pub issuer_rating_valid_from: Option<NaiveDate>,
    pub issuer_rating_valid_till: Option<NaiveDate>,
    pub issuer_sub_industry: String,
    pub gl_code: String,
    pub interest_type: String,
    pub computed_mat_date: Option<NaiveDate>,
    pub cgl: String,
    pub group: String,
    pub llg: String,
    pub currency: String,
    pub mat_date_flag: String,
    pub concat_deal_id: String,
    pub concat_inst_id: String,
    pub concat_deal_n_slr_id: String,
    pub ftp_coupon_rate: f64    
}

impl<'a> InputAccount {
    pub fn new_from_line(
        line: String,
        dmy_date_parser: &DateParser,
    ) -> Result<InputAccount, &'a str> {
        let mut value_iterator = line.split('|');
        let input_account = InputAccount {
            portfolio: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `portfolio`.");
                }
            },
            security_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `security_name`.");
                }
            },
            instrument_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `instrument_id`.");
                }
            },
            instrument_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `instrument_type`.");
                }
            },
            isin_code: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `isin_code`.");
                }
            },
            slr_nslr: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `slr_nslr`.");
                }
            },
            category: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `category`.");
                }
            },
            category_grp: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `category_grp`.");
                }
            },
            sub_category: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sub_category`.");
                }
            },
            fa_classification: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `fa_classification`.");
                }
            },
            maturity_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `maturity_date`.");
                }
            },
            coupon: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `coupon`.");
                }
            },
            discount_intr_rate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `discount_intr_rate`.");
                }
            },
            face_val_per_units: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `face_val_per_units`.");
                }
            },
            quantity: match value_iterator.next() {
                Some(val) => val.parse::<f64>().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `quantity`.");
                }
            },
            face_value: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `face_value`.");
                }
            },
            wap: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `wap`.");
                }
            },
            yield_value: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `yield_value`.");
                }
            },
            book_value: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `book_value`.");
                }
            },
            mvmnt_amount: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `mvmnt_amount`.");
                }
            },
            market_price: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `market_price`.");
                }
            },
            market_yield: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `market_yield`.");
                }
            },
            market_value: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `market_value`.");
                }
            },
            duration: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `duration`.");
                }
            },
            m_duration: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `m_duration`.");
                }
            },
            appreciation: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `appreciation`.");
                }
            },
            depreciation: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `depreciation`.");
                }
            },
            net_app_dep: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `net_app_dep`.");
                }
            },
            convexity: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `convexity`.");
                }
            },
            pvbp: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `pvbp`.");
                }
            },
            absolute_pvbp: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `absolute_pvbp`.");
                }
            },
            amortization_asondate: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `amortization_asondate`.");
                }
            },
            accounted_amortization: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `accounted_amortization`.");
                }
            },
            unaccounted_amortization: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `unaccounted_amortization`.");
                }
            },
            accrued_interest: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `accrued_interest`.");
                }
            },
            no_of_ca_skipped: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `no_of_ca_skipped`.");
                }
            },
            ca_interest_not_receieved: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `ca_interest_not_received`.");
                }
            },
            total_interest: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `total_interest`.");
                }
            },
            encumbered_since_repo: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `encumbered_since_repo`.");
                }
            },
            amount_repo: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `amount_repo`.");
                }
            },
            encumbered_since_ccil: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `encumbered_since_ccil`.");
                }
            },
            amount_ccil: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `amount_ccil`.");
                }
            },
            encumbered_since_treps: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `encumbered_since_treps`.");
                }
            },
            amount_treps: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `amount_treps`.");
                }
            },
            encumbered_since_mcxs: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `encumbered_since_mcxs`.");
                }
            },
            amount_mcxs: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `amount_mcxs`.");
                }
            },
            encumbered_since_others: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `encumbered_since_others`.");
                }
            },
            amount_others: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `amount_others`.");
                }
            },
            custody_pos_number: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `custory_pos_number`.");
                }
            },
            custody_pos_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `custody_pos_type`.");
                }
            },
            interest_frequency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `interest_frequence`.");
                }
            },
            interest_practice: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `interest_practice`.");
                }
            },
            put_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `put_date`.");
                }
            },
            call_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `call_date`.");
                }
            },
            last_coupon_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `last_coupon_date`.");
                }
            },
            next_coupon_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `next_coupon_date`.");
                }
            },
            issue_date: match value_iterator.next() {
                Some(val) => dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `issue_date`.");
                }
            },
            place: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `place`.");
                }
            },
            country: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `country`.");
                }
            },
            booking_basis: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `booking_basis`.");
                }
            },
            residual_maturity: match value_iterator.next() {
                Some(val) => val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `residual_maturity`.");
                }
            },
            issuer_name: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `issuer_name`.");
                }
            },
            market: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `market`.");
                }
            },
            gurantor: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `gurantor`.");
                }
            },
            industry: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `industry`.");
                }
            },
            sub_industry: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `sub_industry`.");
                }
            },
            borrower_category: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `borrower_category`.");
                }
            },
            asset_classification: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `asset_classification`.");
                }
            },
            asset_type: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `asset_type`.");
                }
            },
            asset_category: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `asset_category`.");
                }
            },
            old_security_id: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `old_security_id`.");
                }
            },
            curve1: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `curve1`.");
                }
            },
            listed: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `listed`.");
                }
            },
            secured: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `secured`.");
                }
            },
            quoted: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `quoted`.");
                }
            },
            borrower: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `borrower`.");
                }
            },
            extbank_ref: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `extbank_ref`.");
                }
            },
            pan: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `pan`.");
                }
            },
            intr_rating_agency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `intr_rating_agency`.");
                }
            },
            internal_rating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `internal_rating`.");
                }
            },
            intr_rating_valid_from: match value_iterator.next() {
                Some(val) =>  dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `intr_rating_valid_from`.");
                }
            },
            intr_rating_valid_till: match value_iterator.next() {
                Some(val) =>  dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `intr_rating_valid_till`.");
                }
            },
            extrn_rating_agency: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `extrn_rating_agency`.");
                }
            },
            external_rating: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `external_rating`.");
                }
            },
            extrn_rating_valid_from: match value_iterator.next() {
                Some(val) =>  dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `external_rating_valid_from`.");
                }
            },
            extrn_rating_valid_till: match value_iterator.next() {
                Some(val) =>  dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `extrn_rating_valid_till`.");
                }
            },
            liquid_status: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `liquid_status`.");
                }
            },
            asset_sub_class: match value_iterator.next() {
                Some(val) => val.to_string(),
                None => {
                    return Err("Could not parse property `asset_sub_class`.");
                }
            },
            hurdle_rating: match value_iterator.next() {
                Some(val) =>  val.to_string(),
                None => {
                    return Err("Could not parse property `hurdle_rating`.");
                }
            },
            external_rating_vs_hurdle: match value_iterator.next() {
                Some(val) =>  val.to_string(),
                None => {
                    return Err("Could not parse property `external_rating_vs_hurdle`.");
                }
            },
            internal_rating_vs_hurdle: match value_iterator.next() {
                Some(val) =>  val.to_string(),
                None => {
                    return Err("Could not parse property `internal_rating_vs_hurdle`.");
                }
            },
            fsu: match value_iterator.next() {
                Some(val) =>  val.to_string(),
                None => {
                    return Err("Could not parse property `fsu`.");
                }
            },
            equity_seg: match value_iterator.next() {
                Some(val) =>  val.to_string(),
                None => {
                    return Err("Could not parse property `equity_seg`.");
                }
            },
            issuer_segr: match value_iterator.next() {
                Some(val) =>  val.to_string(),
                None => {
                    return Err("Could not parse property `issuer_segr`.");
                }
            },
            restructuring: match value_iterator.next() {
                Some(val) =>  val.to_string(),
                None => {
                    return Err("Could not parse property `restructuring`.");
                }
            },
            paid_up_share_captial: match value_iterator.next() {
                Some(val) =>  val.to_string(),
                None => {
                    return Err("Could not parse property `paid_up_share_capital`.");
                }
            },
            exempted_amount: match value_iterator.next() {
                Some(val) =>  val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `exempted_amount`.");
                }
            },
            issuer_group: match value_iterator.next() {
                Some(val) =>  val.to_string(),
                None => {
                    return Err("Could not parse property `issuer_group`.");
                }
            },
            murram_market_value: match value_iterator.next() {
                Some(val) =>  val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `murram_market_value`.");
                }
            },
            murram_depr: match value_iterator.next() {
                Some(val) =>  val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `murram_depr`.");
                }
            },
            var_settled_bv: match value_iterator.next() {
                Some(val) =>  val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `var_settled_bv`.");
                }
            },
            var_unsettled_bv: match value_iterator.next() {
                Some(val) =>  val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `var_unsettled_bv`.");
                }
            },
            var_settled_amount: match value_iterator.next() {
                Some(val) =>  val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `var_settled_amount`.");
                }
            },
            var_unsettled_amount: match value_iterator.next() {
                Some(val) =>  val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `var_unsettled_amount`.");
                }
            },
            kri_settled_qtd_fv: match value_iterator.next() {
                Some(val) =>  val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `kri_settled_qtd_fv`.");
                }
            },
            basel_group: match value_iterator.next() {
                Some(val) =>  val.to_string(),
                None => {
                    return Err("Could not parse property `basel_group`.");
                }
            },
            basel_sub_group: match value_iterator.next() {
                Some(val) =>  val.to_string(),
                None => {
                    return Err("Could not parse property `basel_sub_group`.");
                }
            },
            time_band: match value_iterator.next() {
                Some(val) =>  val.to_string(),
                None => {
                    return Err("Could not parse property `time_band`.");
                }
            },
            capital_charge_market_risk_rate: match value_iterator.next() {
                Some(val) =>  val.to_string(),
                None => {
                    return Err("Could not parse property `capital_charge_market_risk_rate`.");
                }
            },
            capital_charge_market_risk_amount: match value_iterator.next() {
                Some(val) =>  val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `capital_charge_market_risk_amount`.");
                }
            },
            trading_specif_risk_rate: match value_iterator.next() {
                Some(val) =>  val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `trading_specif_risk_rate`.");
                }
            },
            banking_specif_risk_rate: match value_iterator.next() {
                Some(val) =>  val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `banking_specif_risk_rate`.");
                }
            },
            trading_specif_risk_captial_charge: match value_iterator.next() {
                Some(val) =>  val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `trading_specif_risk_capital_charge`.");
                }
            },
            banking_specif_risk_captial_charge: match value_iterator.next() {
                Some(val) =>  val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `banking_specif_risk_capital_charge`.");
                }
            },
            mode_of_holding: match value_iterator.next() {
                Some(val) =>  val.to_string(),
                None => {
                    return Err("Could not parse property `mode_of_holding`.");
                }
            },
            issuer_rating_agency: match value_iterator.next() {
                Some(val) =>  val.to_string(),
                None => {
                    return Err("Could not parse property `issuer_rating_agency`.");
                }
            },
            issuer_rating: match value_iterator.next() {
                Some(val) =>  val.to_string(),
                None => {
                    return Err("Could not parse property `issuer_rating`.");
                }
            },
            issuer_rating_valid_from: match value_iterator.next() {
                Some(val) =>  dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `issuer_rating_valid_from`.");
                }
            },
            issuer_rating_valid_till: match value_iterator.next() {
                Some(val) =>  dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `issuer_rating_valid_till`.");
                }
            },
            issuer_sub_industry: match value_iterator.next() {
                Some(val) =>  val.to_string(),
                None => {
                    return Err("Could not parse property `issuer_sub_industry`.");
                }
            },
            gl_code: match value_iterator.next() {
                Some(val) =>  val.to_string(),
                None => {
                    return Err("Could not parse property `gl_code`.");
                }
            },
            interest_type: match value_iterator.next() {
                Some(val) =>  val.to_string(),
                None => {
                    return Err("Could not parse property `interest_type`.");
                }
            },
            computed_mat_date: match value_iterator.next() {
                Some(val) =>  dmy_date_parser.parse_opt(val),
                None => {
                    return Err("Could not parse property `computed_mat_date`.");
                }
            },
            cgl: match value_iterator.next() {
                Some(val) =>  val.to_string(),
                None => {
                    return Err("Could not parse property `cgl`.");
                }
            },
            group: match value_iterator.next() {
                Some(val) =>  val.to_string(),
                None => {
                    return Err("Could not parse property `group`.");
                }
            },
            llg: match value_iterator.next() {
                Some(val) =>  val.to_string(),
                None => {
                    return Err("Could not parse property `llg`.");
                }
            },
            currency: match value_iterator.next() {
                Some(val) =>  val.to_string(),
                None => {
                    return Err("Could not parse property `currency`.");
                }
            },
            mat_date_flag: match value_iterator.next() {
                Some(val) =>  val.to_string(),
                None => {
                    return Err("Could not parse property `mat_date_flag`.");
                }
            },
            concat_deal_id: match value_iterator.next() {
                Some(val) =>  val.to_string(),
                None => {
                    return Err("Could not parse property `concat_deal_id`.");
                }
            },
            concat_inst_id: match value_iterator.next() {
                Some(val) =>  val.to_string(),
                None => {
                    return Err("Could not parse property `concat_inst_id`.");
                }
            },
            concat_deal_n_slr_id: match value_iterator.next() {
                Some(val) =>  val.to_string(),
                None => {
                    return Err("Could not parse property `concat_deal_n_slr_id`.");
                }
            },
            ftp_coupon_rate: match value_iterator.next() {
                Some(val) =>  val.parse().unwrap_or(DEFAULT_FLOAT),
                None => {
                    return Err("Could not parse property `ftp_coupon_rate`.");
                }
            }
        };
        Ok(input_account)
    }
}
