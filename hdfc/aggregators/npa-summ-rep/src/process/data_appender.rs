use super::input_account::stamper::StamperData;
use super::input_account::structs::{
    AccData, COAData, DivisionMapping, FinnoneProdToDiv, LNMAlternateAccs, NPAData, WriteOff,
};
use super::output::{is_leap_year, AggrKey, AggrVal};
use super::StoreData;
use crate::configuration_parameters::ConfigurationParameters;
use chrono::Datelike;

pub fn get_output(
    lookup_key: String,
    acc_data: &AccData,
    config_params: &ConfigurationParameters,
    master_data: &StoreData,
) -> (AggrKey, AggrVal) {
    let acc_no = lookup_key;
    let stmp_acc_no: String = if !master_data.stamper_data.contains_key(&acc_no) {
        master_data
            .lnm_data
            .get(&acc_no)
            .unwrap_or(&LNMAlternateAccs::def())
            .ref6
            .to_string()
    } else {
        acc_no.to_string()
    };
    let division_desc = if acc_data
        .source
        .trim()
        .to_uppercase()
        .starts_with(&"FIN".to_string())
    {
        master_data
            .finnone_data
            .get(
                &master_data
                    .stamper_data
                    .get(&stmp_acc_no)
                    .unwrap_or(&StamperData::def())
                    .prod_type,
            )
            .unwrap_or(&FinnoneProdToDiv::def())
            .division
            .to_string()
    } else {
        master_data
            .division_data
            .get(
                &master_data
                    .stamper_data
                    .get(&stmp_acc_no)
                    .unwrap_or(&StamperData::def())
                    .mis1,
            )
            .unwrap_or(&DivisionMapping::def())
            .mis1_desc
            .to_string()
    };
    let aggr_key = AggrKey {
        coa: master_data
            .coa_data
            .get(
                &master_data
                    .stamper_data
                    .get(&stmp_acc_no)
                    .unwrap_or(&StamperData::def())
                    .prod_type,
            )
            .unwrap_or(&COAData::def())
            .coa_mapping
            .to_string(),
        mis1_code: master_data
            .stamper_data
            .get(&stmp_acc_no)
            .unwrap_or(&StamperData::def())
            .mis1
            .to_string(),
        div_desc: division_desc,
    };
    let mut aggr_val = AggrVal {
        eop: acc_data.bal_amt,
        avg_bal: master_data
            .stamper_data
            .get(&stmp_acc_no)
            .unwrap_or(&StamperData::def())
            .average_balance,
        tpr: master_data
            .stamper_data
            .get(&stmp_acc_no)
            .unwrap_or(&StamperData::def())
            .final_ftp_rate,
        specprov_till_prev_mon: master_data
            .npa_prev_month_data
            .get(&acc_no)
            .unwrap_or(&NPAData::def())
            .specific_prov_amt,
        specprov_till_ason_mon: master_data
            .npa_ason_data
            .get(&acc_no)
            .unwrap_or(&NPAData::def())
            .specific_prov_amt,
        specprov_till_prev_year: master_data
            .npa_prev_year_data
            .get(&acc_no)
            .unwrap_or(&NPAData::def())
            .specific_prov_amt,
        writeoff_till_prev_mon: master_data
            .writeoff_prev_month_data
            .get(&acc_no)
            .unwrap_or(&WriteOff::def())
            .amount,
        writeoff_till_ason_mon: master_data
            .writeoff_ason_data
            .get(&acc_no)
            .unwrap_or(&WriteOff::def())
            .amount,
        writeoff_till_prev_year: master_data
            .writeoff_prev_year_data
            .get(&acc_no)
            .unwrap_or(&WriteOff::def())
            .amount,
        specytdprov_till_prev_mon: 0.0,
        specytdprov_till_ason_mon: 0.0,
        specytdprov_till_prev_year: 0.0,
        float_avg_bal: 0.0,
        float_amount: 0.0,
    };
    aggr_val.specytdprov_till_prev_mon = aggr_val.specprov_till_prev_mon
        - aggr_val.specprov_till_prev_year
        + aggr_val.writeoff_till_prev_mon;
    aggr_val.specytdprov_till_ason_mon = aggr_val.specprov_till_ason_mon
        - aggr_val.specprov_till_prev_year
        + aggr_val.writeoff_till_ason_mon;
    aggr_val.specytdprov_till_prev_year =
        aggr_val.specytdprov_till_ason_mon - aggr_val.specytdprov_till_prev_mon;
    aggr_val.float_avg_bal = aggr_val.specprov_till_ason_mon;
    aggr_val.float_amount = (aggr_val.float_avg_bal
        * aggr_val.tpr
        * (rbdate::get_days_from_month(*config_params.as_on_date()) as f64))
        / (100.00
            * if is_leap_year(config_params.as_on_date().year()) {
                366.00
            } else {
                365.00
            });

    (aggr_key, aggr_val)
}
