use aggregator::account_field_names::AccField;
use aggregator::required_fields::ReqFields;
use configuration_parameters::ConfigurationParameters;
use currency::currency_converter::CurrencyConverter;
use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use sdb_dyn_proto_rdr::reader::types::Type;
use sdb_dyn_proto_rdr::reader::Reader;
use slog::Logger;

pub fn get_passthrough(
    account: &AccountWithCFs,
    keys: &AccField,
    req_fields: &ReqFields,
    config_params: &ConfigurationParameters,
    reader_for_calling_method: &Reader,
    currency_converter: &CurrencyConverter,
    log: &Logger,
) -> String {
    let mut value: String = String::new();
    let mut line: String = "".to_string();

    let acc_ccy: String = account
        .get_string_for_key(&req_fields.acc_ccy)
        .unwrap_or(&config_params.base_ccy().to_string())
        .to_string();

    for field in &keys.fields {
        let field_name = field.name.replace("_#HCY#", "").replace("_#CCY#", "");

        match reader_for_calling_method
            .get_field_type(&field_name)
            .unwrap_or(Type::String)
        {
            Type::I32 => {
                value = match account.get_i32_for_key(&field.name) {
                    Ok(value) => get_op_data(&field.r#type, &value.to_string()),
                    Err(_error) => get_default_data(&field.r#type),
                };
            }

            Type::I64 => {
                value = match account.get_i64_for_key(&field.name) {
                    Ok(value) => get_op_data(&field.r#type, &value.to_string()),
                    Err(_error) => get_default_data(&field.r#type),
                };
            }

            Type::F32 => {
                value = match account.get_f32_for_key(&field.name) {
                    Ok(value) => get_op_data(&field.r#type, &value.to_string()),
                    Err(_error) => get_default_data(&field.r#type),
                };
            }
            Type::F64 => {
                if field.name.contains("_#HCY#") {
                    let act_field_name = field.name.replace("_#HCY#", "").trim().to_string();
                    value = match account.get_f64_for_key(&act_field_name) {
                        Ok(value) => get_op_data(&field.r#type, &value.to_string()),
                        Err(_error) => get_default_data(&field.r#type),
                    };
                    let amt: f64 = value.parse().unwrap_or(0.0);
                    value = currency_converter
                        .convert_to_base(amt, &acc_ccy, &log)
                        .to_string();
                } else if field.name.contains("_#CCY#") {
                    let act_field_name = field.name.replace("_#CCY#", "").trim().to_string();
                    value = match account.get_f64_for_key(&act_field_name) {
                        Ok(value) => get_op_data(&field.r#type, &value.to_string()),
                        Err(_error) => get_default_data(&field.r#type),
                    };
                    let amt: f64 = value.parse().unwrap_or(0.0);
                    value = currency_converter
                        .convert_from_base(amt, &acc_ccy, &log)
                        .to_string();
                } else {
                    value = match account.get_f64_for_key(&field.name) {
                        Ok(value) => get_op_data(&field.r#type, &value.to_string()),
                        Err(_error) => get_default_data(&field.r#type),
                    };
                }
            }
            Type::String => {
                value = match account.get_string_for_key(&field.name) {
                    Ok(value) => get_op_data(&field.r#type, &value.to_string()),
                    Err(_error) => get_default_data(&field.r#type),
                };
            }
            _ => {
                panic!("Cashflow data decryption not supported!!");
            }
        }
        if value == config_params.base_ccy() {
            value = config_params.consol_ccy().to_string();
        }
        line.push_str(&value);
        line.push_str("|");
    }
    line
}

pub fn naivedate_from_timestamp(t: i64) -> rbdate::NaiveDate {
    let naive_date_time = rbdate::NaiveDateTime::from_timestamp(t, 0);
    naive_date_time.date()
}

pub fn get_default_data(data_type: &str) -> String {
    match &data_type[..] {
        "Integer" => 0.to_string(),
        "Float" => 0.0.to_string(),
        "Date" => "01-01-1900".to_string(),
        "String" => "".to_string(),
        _ => panic!("Invalid data type encountered: {}", data_type),
    }
}
pub fn get_op_data(data_type: &str, value: &str) -> String {
    match &data_type[..] {
        "Integer" => value.parse::<i64>().unwrap_or(0).to_string(),
        "Float" => value.parse::<f64>().unwrap_or(0.0).to_string(),
        "Date" => {
            let timestamp_val = value.parse::<i64>().unwrap_or(0);
            naivedate_from_timestamp(timestamp_val)
                .format("%d-%m-%Y")
                .to_string()
        }
        "String" => value.to_string(),
        _ => panic!("Invalid data type encountered: {}", data_type),
    }
}
