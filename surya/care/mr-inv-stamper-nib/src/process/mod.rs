use self::account_field_names::AccFieldNames;
use self::get_prcnt::*;
use self::output_data::*;
use self::writer::write_data;
use configuration_parameters::ConfigurationParameters;
use currency;
use currency::currency_converter::CurrencyConverter;
use currency::CurrencyExchange;
use health_report::HealthReport;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use sdb_dyn_proto_rdr::reader::Reader;
use slog::Logger;

mod account_field_names;
mod get_prcnt;
mod output_data;
mod writer;

#[derive(Debug)]
pub struct ExtraData {
    yield_val: f64,
    mdur: f64,
}

pub fn process(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let mut tot_amt = 0.0;
    // init output writer
    let mut output_file = match sdb_io::buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(create) => create,
        Err(_) => {
            panic!(
                "Could not create output file: `{}`.",
                config_params.output_file_path(),
            );
        }
    };
    let acc_keys = AccFieldNames::new_from_path(config_params.req_fields_file_path());

    // Read currency exchange rate file
    let currency_converter = currency::create_currency_converter(
        config_params.base_currency(),
        config_params.exchange_rate_file(),
    );

    let mut file_rdr: Reader = reader::Reader::new_at_path(
        config_params.metadata_file_path(),
        config_params.input_file_path(),
    );
    let spec_risk_rules_file_path = format!("{}-spec-risk.txt", config_params.rules_file_path());
    let gen_mr_rules_file_path = format!("{}-gen-mr.txt", config_params.rules_file_path());
    let llg_file_path = format!("{}-llg.txt", config_params.rules_file_path());
    let spec_risk_rules = AggRules::new_from_path(&spec_risk_rules_file_path, &file_rdr);
    let gen_mr_rules = AggRules::new_from_path(&gen_mr_rules_file_path, &file_rdr);
    let llg_rules = AggRules::new_from_path(&llg_file_path, &file_rdr);

    let mut acc_enc = 0;
    let mut acc_succ = 0;
    for account in file_rdr.iter() {
        acc_enc += 1;
        // get percentage from rules file
        let spec_risk_prcnt = get_spec_risk_cap_prcnt(&account, &spec_risk_rules, &config_params);
        let gen_mr_prcnt = get_gen_mr_rule_prcnt(&account, &gen_mr_rules, &config_params);
        let llg_id = get_llg(&account, &llg_rules, &config_params);
        let currency = match account.get_string_for_key(&acc_keys.ccy_id) {
            Ok(val) => val.to_string(),
            Err(_) => format!(""),
        };
        let (face_val_hcy, face_val_ccy) = get_bal(
            &account,
            &acc_keys.face_val_hcy,
            currency.to_string(),
            &currency_converter,
            config_params,
            logger,
        );
        let (book_val_hcy, book_val_ccy) = get_bal(
            &account,
            &acc_keys.book_val_hcy,
            currency.to_string(),
            &currency_converter,
            config_params,
            logger,
        );
        tot_amt += book_val_hcy;
        let (market_val_hcy, _market_val_ccy) = get_bal(
            &account,
            &acc_keys.market_val_hcy,
            currency.to_string(),
            &currency_converter,
            config_params,
            logger,
        );
        let target_currency = CurrencyExchange {
            from_ccy: currency,
            to_ccy: config_params.base_currency().to_string(),
        };
        let exchange_rate = currency_converter.exchange_rate(&target_currency, logger);
        let op_data = get_op_data(
            &account,
            &acc_keys,
            llg_id,
            spec_risk_prcnt.to_string(),
            gen_mr_prcnt.to_string(),
            face_val_hcy,
            face_val_ccy,
            book_val_hcy,
            book_val_ccy,
            market_val_hcy,
            exchange_rate,
            &config_params,
        );
        write_data(op_data, &mut output_file);
        acc_succ += 1;
    }

    // TODO: use health check lib
    println!("Total account encountered: {}", acc_enc);
    println!("Total account processed: {}", acc_succ);
    let health_stat = health_report::HealthReport::new(
        acc_enc,
        acc_succ,
        acc_enc - acc_succ,
        tot_amt,
        tot_amt,
        0,
    );
    health_stat.gen_health_rpt(config_params.output_file_path())
}

fn get_bal(
    account: &AccountWithCFs,
    bal_key: &String,
    currency: String,
    currency_converter: &CurrencyConverter,
    config_params: &ConfigurationParameters,
    logger: &Logger,
) -> (f64, f64) {
    let mut bal_1 = match account.get_f64_for_key(bal_key) {
        Ok(val) => val,
        Err(_) => {
            let def = "0.0".to_string();
            let amt = account.get_string_for_key(bal_key).unwrap_or(&def);
            amt.parse().unwrap_or(0.0)
        }
    };
    let mut bal_2 = currency_converter.convert(
        &account,
        &currency,
        bal_1,
        config_params.is_consolidated(),
        logger,
    );
    if config_params.is_negative() {
        bal_1 *= -1.0;
        bal_2 *= -1.0;
    }
    let bal_ccy;
    let bal_lcy;
    if config_params.is_consolidated() {
        bal_ccy = bal_2;
        bal_lcy = bal_1;
    } else {
        bal_ccy = bal_1;
        bal_lcy = bal_2;
    }
    (bal_lcy, bal_ccy)
}
