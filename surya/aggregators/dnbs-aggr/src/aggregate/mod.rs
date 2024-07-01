mod account_field_names;
pub mod aggr_data;
pub mod aggr_key;
pub mod derive_llg;
pub mod reader;
mod writer;
use self::account_field_names::AccFieldNames;
use self::aggr_data::Data;
use self::aggr_key::AggrKey;
use self::derive_llg::get_llg;
use self::writer::write_aggr_smry;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use sdb_agg_rules::agg_rules::AggRules;
use sdb_dyn_proto_rdr::reader::account_with_cfs::get_field_value;
use sdb_dyn_proto_rdr::reader::Reader;
use slog::Logger;
use std::collections::HashMap;

pub fn aggregate(config_params: ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    // To store aggregated data
    let mut aggr_data: HashMap<AggrKey, Data> = HashMap::new();
    let mut acc_enc = 0;
    let mut acc_succ = 0;
    let mut ip_amt = 0.0;
    let mut op_amt = 0.0;
    // Read cashflow file
    let mut file_rdr: Reader = reader::read_file(
        config_params.input_file_path(),
        config_params.metadata_file_path(),
    );
    let method_reader: Reader = reader::read_file(
        config_params.input_file_path(),
        config_params.metadata_file_path(),
    );
    let rules = AggRules::new_from_path(config_params.rules_file_path(), &file_rdr);
    let req_field = AccFieldNames::new_from_path(config_params.req_fields_file_path());
    for account in file_rdr.iter() {
        acc_enc += 1;
        let account_number = &get_field_value(
            &account,
            &method_reader,
            req_field.account_number.to_string(),
        )
        .expect("Can not read account number field.");

        let ccy = &get_field_value(&account, &method_reader, req_field.currency.to_string())
            .expect("Can not read currency field.");

        let amt = get_field_value(&account, &method_reader, req_field.amount.to_string())
            .expect("Can not read amount field.")
            .parse()
            .unwrap_or(0.0);

        let int_rate = get_field_value(&account, &method_reader, req_field.int_rate.to_string())
            .expect("Can not read int rate field.")
            .parse()
            .unwrap_or(0.0);

        ip_amt += amt;
        let llg_id = get_llg(
            &config_params,
            &account,
            &account_number,
            &amt,
            &rules,
            logger,
        );
        // Construct AggrKey for account
        let aggr_key = AggrKey {
            llg_id: llg_id.to_string(),
            as_on_date: config_params.as_on_date().format("%d-%m-%Y").to_string(),
            ccy: ccy.to_string(),
        };

        let acc_data = Data {
            tot_amt: amt,
            weighted_int_rate_sum: int_rate * amt,
            count: 1,
            max_amt: amt,
            min_amt: amt,
            max_int_rate: int_rate,
            min_int_rate: int_rate,
            int_rate_sum: int_rate,
        };

        // Aggregate data
        aggr_data
            .entry(aggr_key)
            .and_modify(|data| data.append_data(acc_data))
            .or_insert(acc_data);
        acc_succ += 1;
    }
    // Write output
    write_aggr_smry(aggr_data, &mut op_amt, &config_params);

    let health_report = HealthReport::new(acc_enc, acc_succ, acc_enc - acc_succ, ip_amt, op_amt, 0);
    health_report.gen_health_rpt(&config_params.output_file_path());
}
