use super::structs::{GAMFields, NPAFields};
use rbdate::DateParser;
use std::collections::HashMap;

pub fn get_op_line(
    gam_fields: &GAMFields,
    fields: Vec<&str>,
    npa_hm: &HashMap<String, NPAFields>,
) -> String {
    let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
    let default_npa = NPAFields::new();
    let npa_data = match npa_hm.get(&fields[1].to_string()) {
        Some(data) => data,
        None => &default_npa,
    };
    format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|0.0|0.0|0.0|0.0|0.0|0.0|{}|{}|{}|{}||\n",
            fields[0],
            fields[1],
            gam_fields.branch,
            fields[3],
            gam_fields.scheme_code,
             date_parser.parse(fields[5]).format("%d-%m-%Y"),
            fields[6].parse::<f64>().unwrap_or(0.0),
            date_parser.parse(fields[7]).format("%d-%m-%Y"),
            date_parser.parse(fields[8]).format("%d-%m-%Y"),
            gam_fields.acid,
            gam_fields.customer_id,
            fields[11].parse::<f64>().unwrap_or(0.0),
            fields[12].parse::<f64>().unwrap_or(0.0),
            fields[13].parse::<f64>().unwrap_or(0.0),
            fields[14].parse::<f64>().unwrap_or(0.0),
            fields[15],
            fields[16],
            gam_fields.gl_code,
            fields[18],
            fields[19].parse::<f64>().unwrap_or(0.0),
            gam_fields.entity_cre_flag,
            date_parser.parse(fields[21]).format("%d-%m-%Y"),
            fields[22].parse::<i64>().unwrap_or(0),
            gam_fields.cif_id,
            gam_fields.schm_type,
            gam_fields.acct_ownership,
            fields[31],
            npa_data.npa_classification,
            npa_data.provision_amt,
            npa_data.claim_amount,
            npa_data.writeoff,
            npa_data.gnpa,
        )
}
