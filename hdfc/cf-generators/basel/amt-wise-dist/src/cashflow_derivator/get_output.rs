use crate::cashflow_derivator::convert_datatype::*;
use crate::cashflow_derivator::AccsData;
use crate::cashflow_derivator::KeyID;
use crate::macros;
use slog::Logger;
use std::collections::HashMap;

pub fn get_output_map(
    mut output_map: HashMap<KeyID, AccsData>,
    input_fields: Vec<&str>,
    log: &Logger,
) -> HashMap<KeyID, AccsData> {
    if str_to_int(input_fields[2]) != 6 && {
        str_to_int(input_fields[20]) != 0
            || str_to_int(input_fields[63]) != 0
            || str_to_int(input_fields[76]) != 0
    } {
        let key_for_cid_6 = KeyID {
            custid: str_to_int(input_fields[1]),
            classid: str_to_int("6"),
        };
        let data_for_cid_6 = AccsData {
            curr: input_fields[5].to_string(),
            tot_amt: str_to_flt(input_fields[20]),
            tot_nwd_amt: 0.0,
            tot_accs: 0.0,
            tot_nwd_accs: 0.0,
            ca_accs: 0.0,
            sa_accs: 0.0,
            td_accs: 0.0,
            rd_accs: 0.0,
            ca_nwd_accs_op: str_to_flt(input_fields[20]),
            ca_nwd_accs_nonop: 0.0,
            sa_nwd_accs: 0.0,
            td_nwd_accs: 0.0,
            rd_nwd_accs: 0.0,
            td_amt: str_to_flt(input_fields[23]),
            rd_amt: str_to_flt(input_fields[24]),
            sa_amt: str_to_flt(input_fields[22]),
            ca_amt: str_to_flt(input_fields[20]) + str_to_flt(input_fields[21]),
        };
        output_map.insert(key_for_cid_6, data_for_cid_6);

        let key_for_oth_cid = KeyID {
            custid: str_to_int(input_fields[1]),
            classid: str_to_int(input_fields[2]),
        };
        let amt = str_to_flt(input_fields[19]) - str_to_flt(input_fields[20]);
        let data_for_oth_cid = AccsData {
            curr: input_fields[5].to_string(),
            tot_amt: amt,
            tot_nwd_amt: str_to_flt(input_fields[86]),
            tot_accs: str_to_flt(input_fields[87]),
            tot_nwd_accs: str_to_flt(input_fields[75]),
            ca_accs: str_to_flt(input_fields[70]),
            sa_accs: str_to_flt(input_fields[71]),
            td_accs: str_to_flt(input_fields[72]),
            rd_accs: str_to_flt(input_fields[73]),
            ca_nwd_accs_op: 0.0,
            ca_nwd_accs_nonop: str_to_flt(input_fields[82]),
            sa_nwd_accs: str_to_flt(input_fields[83]),
            td_nwd_accs: str_to_flt(input_fields[84]),
            rd_nwd_accs: str_to_flt(input_fields[85]),
            td_amt: str_to_flt(input_fields[23]),
            rd_amt: str_to_flt(input_fields[24]),
            sa_amt: str_to_flt(input_fields[22]),
            ca_amt: str_to_flt(input_fields[20]) + str_to_flt(input_fields[21]),
        };
        log_info!(
            log,
            "CustID: `{}` has been split as, ClassID: `{}`, CA_Operational_Total: `{}`, CA_Op_Int_Rate: `{}`, Number_of_CA_WD_accounts(Operational): `{}`.",
            input_fields[1],
            input_fields[2],
            input_fields[20],
            input_fields[63],
            input_fields[76],
        );
        output_map.insert(key_for_oth_cid, data_for_oth_cid);
    } else {
        let keyid = KeyID {
            custid: str_to_int(input_fields[1]),
            classid: str_to_int(input_fields[2]),
        };
        let data = AccsData {
            curr: input_fields[5].to_string(),
            tot_amt: str_to_flt(input_fields[19]),
            tot_nwd_amt: str_to_flt(input_fields[86]),
            tot_accs: str_to_flt(input_fields[87]),
            tot_nwd_accs: str_to_flt(input_fields[75]),
            ca_accs: str_to_flt(input_fields[70]),
            sa_accs: str_to_flt(input_fields[71]),
            td_accs: str_to_flt(input_fields[72]),
            rd_accs: str_to_flt(input_fields[73]),
            ca_nwd_accs_op: str_to_flt(input_fields[81]),
            ca_nwd_accs_nonop: str_to_flt(input_fields[82]),
            sa_nwd_accs: str_to_flt(input_fields[83]),
            td_nwd_accs: str_to_flt(input_fields[84]),
            rd_nwd_accs: str_to_flt(input_fields[85]),
            td_amt: str_to_flt(input_fields[23]),
            rd_amt: str_to_flt(input_fields[24]),
            sa_amt: str_to_flt(input_fields[22]),
            ca_amt: str_to_flt(input_fields[20]) + str_to_flt(input_fields[21]),
        };
        output_map.insert(keyid, data);
    }
    output_map
}
