use super::*;
use chrono::Datelike;
use sdb_day_convention::days_with_convn;
use sdb_dyn_proto_rdr::reader::account_with_cfs::Cashflow;
use std::fs;

pub struct StaticParams<'a> {
    pub config_params: &'a ConfigurationParameters,
    pub log: &'a Logger,
    pub diag_log: &'a Logger,
    pub old_acc_map: OldAccountMap,
    pub adj_rates: Adjustments,
    pub bal_slab: BalanceSlabs,
    pub spread_writer: BufWriter<File>,
    pub method_master: MethodMap,
    pub saved_bm_rates: IntermediateBMPointsMap,
    pub out_writer: BufWriter<File>,
    pub cf_det_writer: BufWriter<File>,
    pub no_of_days_in_year: f64,
}

impl<'a> StaticParams<'a> {
    pub fn new(
        log: &'a Logger,
        diag_log: &'a Logger,
        config_params: &'a ConfigurationParameters,
    ) -> Self {
        // Deriving spread file output path
        let spread_path = config_params
            .output_file_path()
            .to_string()
            .replace(".txt", "_spread.txt");

        // Deriving cf level detailed file output path
        let cf_dt_path = config_params
            .output_file_path()
            .to_string()
            .replace(".txt", "_cf_det.txt");
        let mut cf_det_writer = get_writer(&cf_dt_path);

        let start_date = NaiveDate::from_ymd(config_params.to_date().year(), 1, 1);
        let end_date = NaiveDate::from_ymd(config_params.to_date().year(), 12, 31);
        let no_of_days_in_year =
            (days_with_convn(start_date, end_date, config_params.day_count_basis())
                .unwrap()
                .day_in_yr) as f64;
        // Writing header in cf detailed file
        write!(
            cf_det_writer,
            "Acc_id|FTPMethod|Start_date|CF_Date|Maturity_date|Last_repr_date|Cf_Prin_amount|Cf_Int_amount|Base_rate|Adj1|Adj2|Adj3|Adj4|Adj5|Adj6|Ftp_rate|Residual_days|Yield_rate|Org_bal_tenor|Base_rate_prod|End_rate_prod\n",
        )
        .expect("Error while writing headers in cf_level detail file.");

        Self {
            config_params,
            log,
            diag_log,
            old_acc_map: SpreadReader::new(config_params.spread_file_path()),
            adj_rates: Adjustments::new(config_params.adj_rate_file_path()),
            bal_slab: BalanceSlabs::new(config_params.bal_slab_file()),
            spread_writer: get_writer(&spread_path),
            method_master: get_method_config(&MethodField::new_from_path(
                &config_params.method_req_fields_file_path(),
            )),
            saved_bm_rates: HashMap::new(),
            out_writer: get_writer(config_params.output_file_path()),
            cf_det_writer,
            no_of_days_in_year,
        }
    }
}

pub struct DynamicParams {
    pub is_consolidated: bool,
    pub m_rules: AggRules,
    pub bc_rules: AggRules,
    pub aorl_rules: AggRules,
    pub llg_rules: AggRules,
    pub fix_adj_rules: AggRulesAdj,
    pub var_adj_rules: AggRulesAdj,
    pub input_field_names: AccFieldNames,
    pub avg_bal: AverageBalance,
    pub exrt_map: ExchangeRates,
    pub out_writer: BufWriter<File>,
    pub is_cf_req: bool,
    pub is_int_amt_consolidated: bool,
}

impl<'a> DynamicParams {
    pub fn new(config_params: &'a ConfigurationParameters, file: &'a ConfigFile) -> Self {
        let input_data = Reader::new_at_path(
            &file.metadata_file_path,
            &get_file_path(file.input_file_path.to_string(), *config_params.to_date()),
        );

        let amb_file_path = get_file_path(file.amb_file_path.to_string(), *config_params.to_date());

        Self {
            is_consolidated: file.is_consolidated,
            m_rules: AggRules::new_from_path(config_params.method_rules_file_path(), &input_data),
            bc_rules: AggRules::new_from_path(config_params.bc_rule_file_path(), &input_data),
            aorl_rules: AggRules::new_from_path(config_params.aorl_rule_file_path(), &input_data),
            llg_rules: AggRules::new_from_path(config_params.llg_rule_file_path(), &input_data),
            fix_adj_rules: AggRulesAdj::new_from_path(
                config_params.fix_adj_rule_file_path(),
                &input_data,
            ),
            var_adj_rules: AggRulesAdj::new_from_path(
                config_params.var_adj_rule_file_path(),
                &input_data,
            ),
            input_field_names: AccFieldNames::new_from_path(&file.req_fields_file_path),
            avg_bal: AverageBalance::new(&amb_file_path, config_params.skip_amb_header()),
            exrt_map: ExchangeRates::new(&get_file_path(
                file.exrt_file_path.to_string(),
                *config_params.to_date(),
            )),
            out_writer: get_writer(config_params.output_file_path()),
            is_cf_req: file.is_cf_req,
            is_int_amt_consolidated: file.is_int_amt_consolidated,
        }
    }
}

pub type Cashflows = Vec<Cashflow>;
pub struct DerivedFields {
    pub method_id: i32,
    pub llg_id: String,
    pub a_or_l_value: String,
    pub parsed_method: ParsedMethod,
    pub basecurve: i32,
    pub fix_adjs: Vec<i32>,
    pub var_adjs: Vec<i32>,
    pub cashflows: Cashflows,
}

impl DerivedFields {
    pub fn new(
        acc_id: &str,
        acc_data: &mut AccountWithCFs,
        static_params: &StaticParams,
        dyn_params: &DynamicParams,
    ) -> Self {
        let default_llg = fs::read_to_string(static_params.config_params.default_llg_file_path())
            .expect("Unable to read default LLG file")
            .trim()
            .to_string();
        let default_method =
            fs::read_to_string(static_params.config_params.default_method_file_path())
                .expect("Unable to read default METHOD file")
                .trim()
                .to_string();
        let default_basecurve =
            fs::read_to_string(static_params.config_params.default_basecurve_file_path())
                .expect("Unable to read default BaseCurve file")
                .trim()
                .to_string();

        let method_id = get_method(
            acc_id,
            &acc_data,
            &dyn_params.m_rules,
            default_method,
            &static_params.log,
        );

        Self {
            method_id,
            llg_id: get_llg(
                acc_id,
                &acc_data,
                &dyn_params.llg_rules,
                &default_llg,
                &static_params.log,
            ),
            parsed_method: ParsedMethod::new(
                method_id,
                &static_params.method_master,
                acc_data,
                timestamp(*static_params.config_params.to_date()),
            ),
            a_or_l_value: get_aorl_value(
                acc_id,
                &acc_data,
                &dyn_params.aorl_rules,
                static_params.config_params.default_aorl_flag(),
                &static_params.log,
            ),
            basecurve: get_bc(
                acc_id,
                &acc_data,
                &dyn_params.bc_rules,
                default_basecurve,
                &static_params.log,
            ),
            fix_adjs: get_adj(
                acc_id,
                &acc_data,
                &dyn_params.fix_adj_rules,
                static_params.config_params.fixed_adj_count(),
                &static_params.log,
            ),
            var_adjs: get_adj(
                acc_id,
                &acc_data,
                &dyn_params.var_adj_rules,
                static_params.config_params.var_adj_count(),
                &static_params.log,
            ),
            cashflows: acc_data
                .remove_cfs_for_key(&dyn_params.input_field_names.cashflows)
                .unwrap_or_default(),
        }
    }
}