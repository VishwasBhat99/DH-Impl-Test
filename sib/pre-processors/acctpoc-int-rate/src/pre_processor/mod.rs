use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
mod pca;
mod structs;
mod writers;
use self::pca::*;
use self::structs::*;
use self::writers::get_writer;
use macros;
use rbdate::DateParser;

pub fn process(config_param: ConfigurationParameters, _log: &Logger, _diag_log: &Logger) {
    let data_parser = DateParser::new("%d-%m-%Y".to_string(), true);
    let mut op_writer = get_writer(config_param.output_file_path());
    let mut acc_enctrd = 0;
    let mut acc_succ = 0;
    // Defaults
    let itc_default = ItcData {
        ..Default::default()
    };
    // ITC
    let mut itc_data: HashMap<String, (ItcData, ItcData)> = HashMap::new();
    let itc_file_reader = match new_buf_rdr(config_param.itc_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.itc_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    let default_itc_value = &(itc_default.to_owned(), itc_default.to_owned());
    for (line_num, lines) in itc_file_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.itc_file_path(),
                line_num + 1,
                error
            ),
        };
        let itc_fields: Vec<&str> = line.split('|').collect();
        let start_date = data_parser
            .parse_opt(itc_fields[3])
            .expect("Could not parse start date from ITC file.");
        let end_date = data_parser
            .parse_opt(itc_fields[16])
            .expect("Could not parse end date from ITC file.");
        let entity_cre_flg = itc_fields[14].to_string();
        //Store first max and second max data in ITC file.
        let itc_entity_id = itc_fields[0].trim().to_uppercase();
        if itc_data.contains_key(&itc_entity_id) {
            let existing_data = itc_data.get(&itc_entity_id).unwrap_or(default_itc_value);
            if entity_cre_flg == "Y"
                && start_date <= *config_param.as_on_date()
                && end_date >= *config_param.as_on_date()
                && existing_data.0.int_tbl_code_srl_num < itc_fields[17].to_string()
            {
                let second_data = existing_data.0.to_owned();
                let data = ItcData {
                    int_tbl_code: itc_fields[2].to_string(),
                    int_tbl_ver_num: itc_fields[29].parse::<i64>().unwrap_or(0),
                    cust_dr_pref_pcnt: itc_fields[6].parse::<f64>().unwrap_or(0.0),
                    id_cr_pref_pcnt: itc_fields[7].parse::<f64>().unwrap_or(0.0),
                    id_dr_pref_pcnt: itc_fields[8].parse::<f64>().unwrap_or(0.0),
                    int_tbl_code_srl_num: itc_fields[17].to_string(),
                    max_int_pcnt_dr: itc_fields[12].parse::<f64>().unwrap_or(0.0),
                    min_int_pcnt_dr: itc_fields[10].parse::<f64>().unwrap_or(0.0),
                };
                itc_data.insert(itc_entity_id.to_string(), (data, second_data.to_owned()));
            } else if entity_cre_flg == "Y"
                && start_date <= *config_param.as_on_date()
                && end_date >= *config_param.as_on_date()
                && existing_data.1.int_tbl_code_srl_num < itc_fields[17].to_string()
            {
                let second_data = ItcData {
                    int_tbl_code: itc_fields[2].to_string(),
                    int_tbl_ver_num: itc_fields[29].parse::<i64>().unwrap_or(0),
                    cust_dr_pref_pcnt: itc_fields[6].parse::<f64>().unwrap_or(0.0),
                    id_cr_pref_pcnt: itc_fields[7].parse::<f64>().unwrap_or(0.0),
                    id_dr_pref_pcnt: itc_fields[8].parse::<f64>().unwrap_or(0.0),
                    int_tbl_code_srl_num: itc_fields[17].to_string(),
                    max_int_pcnt_dr: itc_fields[12].parse::<f64>().unwrap_or(0.0),
                    min_int_pcnt_dr: itc_fields[10].parse::<f64>().unwrap_or(0.0),
                };
                itc_data.insert(
                    itc_entity_id.to_string(),
                    (existing_data.0.to_owned(), second_data.to_owned()),
                );
            }
        } else if entity_cre_flg == "Y"
            && start_date <= *config_param.as_on_date()
            && end_date >= *config_param.as_on_date()
        {
            let data = ItcData {
                int_tbl_code: itc_fields[2].to_string(),
                int_tbl_ver_num: itc_fields[29].parse::<i64>().unwrap_or(0),
                cust_dr_pref_pcnt: itc_fields[6].parse::<f64>().unwrap_or(0.0),
                id_cr_pref_pcnt: itc_fields[7].parse::<f64>().unwrap_or(0.0),
                id_dr_pref_pcnt: itc_fields[8].parse::<f64>().unwrap_or(0.0),
                int_tbl_code_srl_num: itc_fields[17].to_string(),
                max_int_pcnt_dr: itc_fields[12].parse::<f64>().unwrap_or(0.0),
                min_int_pcnt_dr: itc_fields[10].parse::<f64>().unwrap_or(0.0),
            };
            itc_data.insert(itc_entity_id.to_string(), (data, itc_default.to_owned()));
        }
    }

    //ICV
    let mut icv_data: HashMap<IcvKey, Vec<IcvValue>> = HashMap::new();
    let icv_file_reader = match new_buf_rdr(config_param.icv_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.icv_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    for (line_num, lines) in icv_file_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.icv_file_path(),
                line_num + 1,
                error
            ),
        };
        let icv_fields: Vec<&str> = line.split('|').collect();
        if icv_fields[4] != "Y" {
            continue;
        }
        let key = IcvKey {
            crncy_code: icv_fields[9].to_string(),
            int_tbl_code: icv_fields[5].to_string(),
        };

        let value = IcvValue {
            base_int_tbl_code: icv_fields[11].to_string(),
            int_version: icv_fields[0].to_string(),
            base_pcnt_cr: icv_fields[1].parse::<f64>().unwrap_or(0.0),
            base_pcnt_dr: icv_fields[2].parse::<f64>().unwrap_or(0.0),
            end_time: icv_fields[7].to_string(),
            lchg_time: icv_fields[10].to_string(),
            start_date: icv_fields[6].to_string(),
            int_tbl_ver_num: icv_fields[12].parse::<i64>().unwrap_or(0),
        };
        icv_data
            .entry(key)
            .and_modify(|val| val.push(value.to_owned()))
            .or_insert(vec![value]);
    }

    //PCA
    let mut pca_data: HashMap<String, Vec<PCAData>> = HashMap::new();
    let pca_file_reader = match new_buf_rdr(config_param.pca_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.pca_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    for (line_num, lines) in pca_file_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.pca_file_path(),
                line_num + 1,
                error
            ),
        };
        let pca_fields: Vec<&str> = line.split('|').collect();
        let value = PCAData {
            disb_id: pca_fields[1].to_string(),
            ost_amt: pca_fields[2].parse::<f64>().unwrap_or(0.0),
        };
        pca_data
            .entry(pca_fields[0].to_string())
            .and_modify(|val| val.push(value.to_owned()))
            .or_insert(vec![value]);
    }

    // IVS
    let mut ivs_min_data: HashMap<IvsLavsMinKey, Vec<IvsLavsMinVal>> = HashMap::new();
    let mut ivs_data: HashMap<IvsLavsKey, IvsLavsVal> = HashMap::new();
    let ivs_file_reader = match new_buf_rdr(config_param.ivs_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.ivs_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    for (line_num, lines) in ivs_file_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.ivs_file_path(),
                line_num + 1,
                error
            ),
        };

        let ivs_fields: Vec<&str> = line.split('|').collect();

        let min_key = IvsLavsMinKey {
            crncy_code: ivs_fields[11].to_string(),
            int_slab_dr_cr_flg: ivs_fields[14].to_string(),
            int_tbl_code: ivs_fields[12].to_string(),
            int_tbl_ver_num: ivs_fields[13].parse::<i64>().unwrap_or(0),
        };

        let min_val = IvsLavsMinVal {
            end_slab_amt: ivs_fields[17].parse::<f64>().unwrap_or(0.0),
            int_slab_srl_num: ivs_fields[18].parse::<i64>().unwrap_or(0),
            nrml_int_pcnt: ivs_fields[1].parse::<f64>().unwrap_or(0.0),
        };

        let key = IvsLavsKey {
            crncy_code: ivs_fields[11].to_string(),
            int_slab_dr_cr_flg: ivs_fields[14].to_string(),
            int_slab_srl_num: ivs_fields[5].to_string(),
            int_tbl_code: ivs_fields[12].to_string(),
            int_tbl_ver_num: ivs_fields[13].parse::<i64>().unwrap_or(0),
        };

        let val = IvsLavsVal {
            nrml_int_pcnt: ivs_fields[1].parse::<f64>().unwrap_or(0.0),
        };
        ivs_min_data
            .entry(min_key)
            .and_modify(|val| val.push(min_val.to_owned()))
            .or_insert(vec![min_val]);
        ivs_data.insert(key, val);
    }

    // LAVS
    let mut lavs_min_data: HashMap<IvsLavsMinKey, Vec<IvsLavsMinVal>> = HashMap::new();
    let mut lavs_data: HashMap<IvsLavsKey, IvsLavsVal> = HashMap::new();
    let lavs_file_reader = match new_buf_rdr(config_param.lavs_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.lavs_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    for (line_num, lines) in lavs_file_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.lavs_file_path(),
                line_num + 1,
                error
            ),
        };

        let lavs_fields: Vec<&str> = line.split('|').collect();

        let min_key = IvsLavsMinKey {
            crncy_code: lavs_fields[2].to_string(),
            int_slab_dr_cr_flg: lavs_fields[4].to_string(),
            int_tbl_code: lavs_fields[1].to_string(),
            int_tbl_ver_num: lavs_fields[3].parse::<i64>().unwrap_or(0),
        };

        let min_val = IvsLavsMinVal {
            end_slab_amt: lavs_fields[6].parse::<f64>().unwrap_or(0.0),
            int_slab_srl_num: lavs_fields[5].parse::<i64>().unwrap_or(0),
            nrml_int_pcnt: lavs_fields[0].parse::<f64>().unwrap_or(0.0),
        };

        let key = IvsLavsKey {
            crncy_code: lavs_fields[2].to_string(),
            int_slab_dr_cr_flg: lavs_fields[4].to_string(),
            int_slab_srl_num: lavs_fields[5].to_string(),
            int_tbl_code: lavs_fields[1].to_string(),
            int_tbl_ver_num: lavs_fields[3].parse::<i64>().unwrap_or(0),
        };

        let val = IvsLavsVal {
            nrml_int_pcnt: lavs_fields[0].parse::<f64>().unwrap_or(0.0),
        };
        lavs_min_data
            .entry(min_key)
            .and_modify(|val| val.push(min_val.to_owned()))
            .or_insert(vec![min_val]);
        lavs_data.insert(key, val);
    }

    // GAM
    let gam_file_reader = match new_buf_rdr(config_param.gam_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.gam_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    for (line_num, lines) in gam_file_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.gam_file_path(),
                line_num + 1,
                error
            ),
        };
        acc_enctrd += 1;
        let gam_fields: Vec<&str> = line.split('|').collect();
        let mut final_int_rate = 0.0;
        //Calculate interest rate for PCA scheme type:
        if gam_fields[15].trim().to_uppercase() == "PCA" {
            log_debug!(_diag_log, "PCA-acid:{}", gam_fields[0]);
            final_int_rate = get_pca_int_rate(
                gam_fields.to_owned(),
                &mut icv_data,
                &mut ivs_min_data,
                &mut itc_data,
                &mut pca_data,
                &config_param,
            );
        } else {
            let v_acctbal = gam_fields[3].parse::<f64>().unwrap_or(0.0);
            let v_balind = if v_acctbal < 0.0 {
                "D"
            } else if gam_fields[15].trim().to_uppercase() != "SBA" {
                "D"
            } else {
                "C"
            };
            let itc_val_data = itc_data.get(gam_fields[0]).unwrap_or(default_itc_value);
            let mut itc_val = itc_val_data.0.to_owned();
            if gam_fields[14].trim().to_uppercase() == *"HLOLD" {
                itc_val = itc_val_data.1.to_owned()
            }
            let icv_key = IcvKey {
                crncy_code: gam_fields[17].to_string(),
                int_tbl_code: itc_val.int_tbl_code.to_owned(),
            };
            let mut v_inttblver = 0;
            let mut v_intver = String::new();
            let mut v_basetbl = String::new();
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
                    v_inttblver_sd = if vec![
                        "TGENR".to_string(),
                        "TSTFR".to_string(),
                        "TNRER".to_string(),
                        "TNRNR".to_string(),
                    ]
                    .contains(&icv_key.int_tbl_code.to_owned())
                    {
                        data_parser
                            .parse_opt(&val.lchg_time)
                            .unwrap_or(config_param.as_on_date)
                    } else {
                        v_inttblver_sd
                    };
                    if itc_val.int_tbl_ver_num == 0 {
                        if v_inttblver_sd <= *config_param.as_on_date()
                            && v_inttblver_ed >= *config_param.as_on_date()
                            && val.int_tbl_ver_num > v_inttblver
                        {
                            v_inttblver = val.int_tbl_ver_num.to_owned();
                            v_intver = val.int_version.to_owned();
                            v_basetbl = val.base_int_tbl_code.to_owned();
                        }
                    } else if val.int_tbl_ver_num == itc_val.int_tbl_ver_num
                        && v_inttblver_sd <= *config_param.as_on_date()
                        && v_inttblver_ed >= *config_param.as_on_date()
                    {
                        v_inttblver = val.int_tbl_ver_num.to_owned();
                        v_intver = val.int_version.to_owned();
                        v_basetbl = val.base_int_tbl_code.to_owned();
                    }
                }
            }

            let mut v_baseint = 0.0;
            let mut v_basever = 0;
            if !v_basetbl.is_empty() {
                let icv_key = IcvKey {
                    crncy_code: gam_fields[17].to_string(),
                    int_tbl_code: v_basetbl.to_owned(),
                };
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
                        v_inttblver_sd = if vec![
                            "TGENR".to_string(),
                            "TSTFR".to_string(),
                            "TNRER".to_string(),
                            "TNRNR".to_string(),
                        ]
                        .contains(&icv_key.int_tbl_code.to_owned())
                        {
                            data_parser
                                .parse_opt(&val.lchg_time)
                                .unwrap_or(config_param.as_on_date)
                        } else {
                            v_inttblver_sd
                        };

                        if v_inttblver_sd <= *config_param.as_on_date()
                            && v_inttblver_ed >= *config_param.as_on_date()
                            && val.int_tbl_ver_num > v_basever
                        {
                            v_basever = val.int_tbl_ver_num;
                            if v_balind == "D" {
                                v_baseint = val.base_pcnt_dr;
                            } else {
                                v_baseint = val.base_pcnt_cr;
                            }
                        }
                    }
                }
            }

            let v_nrmlint_min_key = IvsLavsMinKey {
                crncy_code: gam_fields[17].to_string(),
                int_slab_dr_cr_flg: v_balind.to_string(),
                int_tbl_code: itc_val.int_tbl_code.to_owned(),
                int_tbl_ver_num: v_intver.parse::<i64>().unwrap_or(0),
            };
            let mut v_nrmlint = 0.0;
            if gam_fields[15].trim().to_uppercase() != "LAA" {
                let def: Vec<IvsLavsMinVal> = vec![];
                let ivs_val = ivs_min_data.get(&v_nrmlint_min_key).unwrap_or(&def);
                let mut int_slab_srl_num = 999999999;
                for val in ivs_val.iter() {
                    if val.end_slab_amt >= (v_acctbal.abs())
                        && val.int_slab_srl_num < int_slab_srl_num
                    {
                        int_slab_srl_num = val.int_slab_srl_num.to_owned();
                        v_nrmlint = val.nrml_int_pcnt;
                    }
                }
            } else {
                let def: Vec<IvsLavsMinVal> = vec![];
                let lavs_val = lavs_min_data.get(&v_nrmlint_min_key).unwrap_or(&def);
                let mut int_slab_srl_num = 999999999;
                for val in lavs_val.iter() {
                    if val.end_slab_amt >= (v_acctbal.abs())
                        && val.int_slab_srl_num < int_slab_srl_num
                    {
                        int_slab_srl_num = val.int_slab_srl_num.to_owned();
                        v_nrmlint = val.nrml_int_pcnt;
                    }
                }
            };
            let v_intrate = if v_balind == "C" {
                itc_val.id_cr_pref_pcnt + v_baseint + v_nrmlint
            } else {
                itc_val.id_dr_pref_pcnt + v_baseint + v_nrmlint + itc_val.cust_dr_pref_pcnt
            };
            final_int_rate = if (gam_fields[15].trim().to_uppercase() == "LAA"
                || gam_fields[15].trim().to_uppercase() == "ODA"
                || gam_fields[15].trim().to_uppercase() == "CCA")
                && v_balind == "D"
                && v_intrate < itc_val.min_int_pcnt_dr
            {
                itc_val.min_int_pcnt_dr
            } else if (gam_fields[15].trim().to_uppercase() == "LAA"
                || gam_fields[15].trim().to_uppercase() == "ODA"
                || gam_fields[15].trim().to_uppercase() == "CCA")
                && v_balind == "D"
                && v_intrate > itc_val.max_int_pcnt_dr
                && itc_val.max_int_pcnt_dr != 0.0
            {
                itc_val.max_int_pcnt_dr
            } else {
                v_intrate
            };
            final_int_rate = (final_int_rate * 100.0).round() / 100.0;
        }
        writeln!(op_writer, "{}|{}", gam_fields[0], final_int_rate).expect("Could not write OP");
        acc_succ += 1;
    }
    let health_report = HealthReport::new(acc_enctrd, acc_succ, acc_enctrd - acc_succ, 0.0, 0.0, 0);
    health_report.gen_health_rpt(config_param.output_file_path());
}
