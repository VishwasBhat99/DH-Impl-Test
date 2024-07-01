use super::structs::*;
use configuration_parameters::ConfigurationParameters;
use rbdate::DateParser;
use std::collections::*;

pub fn get_pca_int_rate(
    gam_fields: Vec<&str>,
    icv_data: &HashMap<IcvKey, Vec<IcvValue>>,
    ivs_min_data: &HashMap<IvsLavsMinKey, Vec<IvsLavsMinVal>>,
    itc_data: &HashMap<String, (ItcData, ItcData)>,
    pca_data: &HashMap<String, Vec<PCAData>>,
    config_param: &ConfigurationParameters,
) -> f64 {
    let data_parser = DateParser::new("%d-%m-%Y".to_string(), true);
    let default_pca = &vec![PCAData::default()];
    let pca_value = pca_data.get(gam_fields[0]).unwrap_or(&default_pca);
    let mut v_totoutstdamt = 0.0;
    let mut v_intamt = 0.0;
    let mut v_tot_int = 0.0;
    let itc_default = ItcData {
        ..Default::default()
    };

    let default_itc_value = &(itc_default.to_owned(), itc_default.to_owned());
    let mut v_intrate = 0.0;
    for disb_val in pca_value.iter() {
        let acid = disb_val.disb_id.trim().to_owned();
        let itc_val_data = itc_data.get(&acid).unwrap_or(default_itc_value);
        let mut itc_val = itc_val_data.0.to_owned();
        let v_inttbl = itc_val.int_tbl_code;
        let v_drpref = itc_val.cust_dr_pref_pcnt + itc_val.id_dr_pref_pcnt;
        let icv_key = IcvKey {
            crncy_code: gam_fields[17].to_string(),
            int_tbl_code: v_inttbl.to_owned(),
        };
        let mut v_intver = "".to_string();
        let mut v_baseint = 0.0;
        let mut v_basever = 0;
        if icv_data.contains_key(&icv_key) {
            let icv_val = icv_data
                .get(&icv_key)
                .expect("Cannot get ICV value from Map.");
            for val in icv_val.iter() {
                let mut v_inttblver_sd = data_parser
                    .parse_opt(&val.start_date)
                    .unwrap_or(config_param.as_on_date);
                let v_inttblver_ed = data_parser
                    .parse_opt(&val.end_time)
                    .unwrap_or(config_param.as_on_date);

                if v_inttblver_sd <= *config_param.as_on_date()
                    && v_inttblver_ed >= *config_param.as_on_date()
                    && val.int_tbl_ver_num > v_basever
                {
                    v_basever = val.int_tbl_ver_num;
                    v_intver = val.int_version.to_owned();
                    v_baseint = val.base_pcnt_dr;
                }
            }
        }
        let v_nrmlint_min_key = IvsLavsMinKey {
            crncy_code: gam_fields[17].to_string(),
            int_slab_dr_cr_flg: "D".to_string(),
            int_tbl_code: v_inttbl.to_owned(),
            int_tbl_ver_num: v_intver.parse::<i64>().unwrap_or(0),
        };
        let mut v_nrmlint = 0.0;
        let def: Vec<IvsLavsMinVal> = vec![];
        let ivs_val = ivs_min_data.get(&v_nrmlint_min_key).unwrap_or(&def);
        let mut int_slab_srl_num = 999999999;
        for val in ivs_val.iter() {
            if val.int_slab_srl_num < int_slab_srl_num {
                int_slab_srl_num = val.int_slab_srl_num.to_owned();
                v_nrmlint = val.nrml_int_pcnt;
            }
        }
        let v_interest_rate = v_drpref + v_baseint + v_nrmlint;
        v_totoutstdamt += disb_val.ost_amt;
        v_intamt = (disb_val.ost_amt * v_interest_rate) / 100.0;
        v_tot_int += v_intamt;
    }
    v_intrate = (v_tot_int / v_totoutstdamt) * 100.0;
    v_intrate
}
