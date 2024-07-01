use chrono::{Datelike, Duration};
use configuration_parameters::ConfigurationParameters;
use rbdate::NaiveDate;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::prelude::*;
mod structs;
mod writers;
use self::structs::*;
use self::writers::get_writer;
use rbdate::*;

pub fn process(config_param: ConfigurationParameters, _log: &Logger, _diag_log: &Logger) {
    let data_parser = DateParser::new("%d-%m-%Y".to_string(), true);
    let mut op_writer = get_writer(config_param.output_file_path());

    let mut final_out_td: HashMap<String, IntData> = HashMap::new();

    //GAM pp output
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

        let gam_fields: Vec<&str> = line.split('|').collect();

        final_out_td.insert(
            gam_fields[0].to_string(),
            IntData {
                v_entity: gam_fields[0].to_string(),
                v_schmtype: gam_fields[15].to_string(),
                v_schmcode: gam_fields[14].to_string(),
                v_crncy: gam_fields[17].to_string(),
                v_opndate: NaiveDate::parse_from_str(gam_fields[20], "%d-%m-%Y")
                    .unwrap_or(*config_param.as_on_date()),
                v_clsdate: NaiveDate::parse_from_str(gam_fields[22], "%d-%m-%Y")
                    .unwrap_or(*config_param.as_on_date()),
                v_depamt: 0.0,
                v_perdmths: 0,
                v_perddays: 0,
                v_opneffdate: NaiveDate::from_ymd_opt(1970, 1, 1)
                    .expect("Could not get default date format."),
                v_matdate: NaiveDate::from_ymd_opt(1970, 1, 1)
                    .expect("Could not get default date format."),
                v_credt: NaiveDate::from_ymd_opt(1970, 1, 1)
                    .expect("Could not get default date format."),
                int_tbl_code_srl_num: 0,
                v_inttbl: "".to_string(),
                v_crpref: 0.0,
                v_drpref: 0,
                v_passdt: NaiveDate::from_ymd_opt(1970, 1, 1)
                    .expect("Could not get default date format."),
                tam_deposit_type: "".to_string(),
                tam_spl_catg_ind: "".to_string(),
                tam_deposit_status: "".to_string(),
                tam_auto_renewed_counter: 0,
                itc_min_int_pcnt_cr: 0.0,
                itc_max_int_pcnt_cr: 0.0,
                itc_nrml_int_pcnt: 0.0,
                itc_base_differential: "".to_string(),
            },
        );
    }

    //RHT
    let mut rht_data: HashMap<String, RhtData> = HashMap::new();
    let rht_file_reader = match new_buf_rdr(config_param.rht_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.rht_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    for (line_num, lines) in rht_file_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.rht_file_path(),
                line_num + 1,
                error
            ),
        };

        let rht_fields: Vec<&str> = line.split('|').collect();
        let opn_eff_date = NaiveDate::parse_from_str(rht_fields[4], "%d-%m-%Y")
            .unwrap_or(*config_param.as_on_date());
        let ren_srl_num = rht_fields[1].to_string().parse::<i64>().unwrap_or(0);
        let val = RhtData {
            deposit_amount: rht_fields[7].to_string().parse::<f64>().unwrap_or(0.0),
            deposit_period_mths: rht_fields[2].to_string().parse::<i64>().unwrap_or(0),
            deposit_period_days: rht_fields[3].to_string().parse::<i64>().unwrap_or(0),
            open_effective_date: opn_eff_date,
            ren_srl_num,
            maturiy_date: NaiveDate::parse_from_str(rht_fields[6], "%d-%m-%Y")
                .unwrap_or(*config_param.as_on_date()),
            rcre_time: NaiveDate::parse_from_str(rht_fields[22], "%d-%m-%Y")
                .unwrap_or(*config_param.as_on_date()),
        };
        if let std::collections::hash_map::Entry::Vacant(e) =
            rht_data.entry(rht_fields[0].to_string())
        {
            e.insert(val);
        } else {
            let rht_acc = rht_data
                .get_mut(&rht_fields[0].trim().to_string())
                .expect("Could not fetch struct for an account from RHT file.");
            if opn_eff_date <= *config_param.as_on_date() && ren_srl_num > rht_acc.ren_srl_num {
                *rht_acc = val;
            }
        }
    }

    //TAM
    let tam_file_reader = match new_buf_rdr(config_param.tam_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.tam_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    for (line_num, lines) in tam_file_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.tam_file_path(),
                line_num + 1,
                error
            ),
        };

        let tam_fields: Vec<&str> = line.split('|').collect();
        if final_out_td.contains_key(&tam_fields[0].to_string()) {
            let mut td_acc = final_out_td
                .get_mut(&tam_fields[0].trim().to_string())
                .expect("Could not fetch struct for an account from TAM file.");
            td_acc.v_opneffdate = NaiveDate::parse_from_str(tam_fields[5], "%d-%m-%Y")
                .unwrap_or(*config_param.as_on_date());
            if (*config_param.as_on_date() < td_acc.v_opndate
                && *config_param.as_on_date() < td_acc.v_opneffdate)
                || *config_param.as_on_date() > td_acc.v_clsdate
            {
                continue;
            } else {
                td_acc.v_depamt = tam_fields[7].to_string().parse::<f64>().unwrap_or(0.0);
                td_acc.v_perdmths = tam_fields[3].to_string().parse::<i64>().unwrap_or(0);
                td_acc.v_perddays = tam_fields[4].to_string().parse::<i64>().unwrap_or(0);
                td_acc.v_matdate = NaiveDate::parse_from_str(tam_fields[6], "%d-%m-%Y")
                    .unwrap_or(*config_param.as_on_date());
                td_acc.v_credt = NaiveDate::parse_from_str(tam_fields[8], "%d-%m-%Y")
                    .unwrap_or(*config_param.as_on_date());
                td_acc.tam_deposit_type = tam_fields[10].to_string();
                td_acc.tam_spl_catg_ind = tam_fields[9].to_string();
                td_acc.tam_deposit_status = tam_fields[1].to_string();
                td_acc.tam_auto_renewed_counter = tam_fields[11].parse::<i64>().unwrap_or(0);
            }
            if *config_param.as_on_date() < td_acc.v_opneffdate {
                match rht_data.get(&tam_fields[0].to_string()) {
                    Some(rht_value) => {
                        td_acc.v_depamt = rht_value.deposit_amount;
                        td_acc.v_perdmths = rht_value.deposit_period_mths;
                        td_acc.v_perddays = rht_value.deposit_period_days;
                        td_acc.v_matdate = rht_value.maturiy_date;
                        td_acc.v_credt = rht_value.rcre_time;
                        //Fields not found in RHT table.
                        td_acc.tam_spl_catg_ind = "".to_string();
                        td_acc.tam_spl_catg_ind = "".to_string();
                        td_acc.tam_deposit_status = "".to_string();
                        td_acc.tam_auto_renewed_counter = 0;
                    }
                    None => {
                        //Append default values.
                        td_acc.v_depamt = 0.0;
                        td_acc.v_perdmths = 0;
                        td_acc.v_perddays = 0;
                        td_acc.v_matdate = NaiveDate::from_ymd_opt(1970, 1, 1)
                            .expect("Could not get default date format.");
                        td_acc.v_credt = NaiveDate::from_ymd_opt(1970, 1, 1)
                            .expect("Could not get default date format.");
                        td_acc.tam_deposit_type = "".to_string();
                        td_acc.tam_spl_catg_ind = "".to_string();
                        td_acc.tam_spl_catg_ind = "".to_string();
                        td_acc.tam_deposit_status = "".to_string();
                        td_acc.tam_auto_renewed_counter = 0;
                    }
                };
            }
        }
    }
    // ITC
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

        let start_date = data_parser.parse_opt(itc_fields[3]).unwrap_or(
            NaiveDate::from_ymd_opt(1970, 1, 1).expect("Could not get default date format."),
        );
        let end_date = data_parser.parse_opt(itc_fields[16]).unwrap_or(
            NaiveDate::from_ymd_opt(1970, 1, 1).expect("Could not get default date format."),
        );
        if final_out_td.contains_key(itc_fields[0]) {
            let td_acc = final_out_td
                .get_mut(itc_fields[0])
                .expect("Could not fetch struct for an account from ITC file.");
            if start_date <= td_acc.v_opneffdate
                && end_date >= td_acc.v_opneffdate
                && itc_fields[17].to_string().parse::<i64>().unwrap_or(0)
                    > td_acc.int_tbl_code_srl_num
            {
                td_acc.v_inttbl = itc_fields[2].to_string();
                td_acc.v_crpref = itc_fields[7].to_string().parse::<f64>().unwrap_or(0.0);
                td_acc.v_drpref = itc_fields[8].to_string().parse::<i64>().unwrap_or(0);
                td_acc.int_tbl_code_srl_num =
                    itc_fields[17].to_string().parse::<i64>().unwrap_or(0);
                td_acc.itc_min_int_pcnt_cr =
                    itc_fields[9].to_string().parse::<f64>().unwrap_or(0.0);
                td_acc.itc_max_int_pcnt_cr = itc_fields[11].parse::<f64>().unwrap_or(0.0);
                td_acc.itc_nrml_int_pcnt = itc_fields[22].parse::<f64>().unwrap_or(0.0);
                td_acc.itc_base_differential = match itc_fields[20].trim().to_lowercase().as_str() {
                    "null" | "" => "N".to_string(),
                    _ => itc_fields[20].trim().to_string(),
                }
            }
        } else {
            //Write int rate is 0.
            continue;
        }
    }

    //ICV
    let mut icv_map: HashMap<IcvKey, Vec<IcvValue>> = HashMap::new();
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
        let key = IcvKey {
            crncy_code: icv_fields[9].to_string(),
            int_tbl_code: icv_fields[5].to_string(),
        };
        let value = IcvValue {
            lchg_time: NaiveDate::parse_from_str(icv_fields[10], "%d-%m-%Y")
                .unwrap_or(*config_param.as_on_date()),
            start_date: NaiveDate::parse_from_str(icv_fields[6], "%d-%m-%Y")
                .unwrap_or(*config_param.as_on_date()),
            end_date: NaiveDate::parse_from_str(icv_fields[7], "%d-%m-%Y")
                .unwrap_or(*config_param.as_on_date()),
            int_tbl_ver_num: if icv_fields[12].is_empty()
                || icv_fields[12].to_string().to_uppercase() == *"NULL"
            {
                0
            } else {
                icv_fields[12].parse::<i64>().unwrap_or(0)
            },
            int_version: icv_fields[0].parse::<i64>().unwrap_or(0),
        };
        icv_map
            .entry(key)
            .and_modify(|val| val.push(value.to_owned()))
            .or_insert(vec![value]);
    }

    // TVS
    let mut tvs_map: HashMap<TvsKey, Vec<TvsValue>> = HashMap::new();
    let tvs_file_reader = match new_buf_rdr(config_param.tvs_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.tvs_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    for (line_num, lines) in tvs_file_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.tvs_file_path(),
                line_num + 1,
                error
            ),
        };

        let tvs_fields: Vec<&str> = line.split('|').collect();
        let key = TvsKey {
            int_tbl_code: tvs_fields[6].to_string(),
            crncy_code: tvs_fields[4].to_string(),
            int_tbl_ver_num: tvs_fields[7].parse::<i64>().unwrap_or(0),
        };
        let value = TvsValue {
            max_period_run_days: tvs_fields[13].parse::<i64>().unwrap_or(0),
            max_period_run_mths: tvs_fields[12].parse::<i64>().unwrap_or(0),
            max_slab_amt: tvs_fields[9].parse::<f64>().unwrap_or(0.0),
            int_slab_srl_num: tvs_fields[16].parse::<i64>().unwrap_or(0),
            nrml_int_pcnt: tvs_fields[0].parse::<f64>().unwrap_or(0.0),
        };
        tvs_map
            .entry(key)
            .and_modify(|val| val.push(value.to_owned()))
            .or_insert(vec![value]);
    }

    for (key, value) in final_out_td {
        let v_passdt = if vec![
            "TGENR".to_string(),
            "TSTFR".to_string(),
            "TNRER".to_string(),
            "TNRNR".to_string(),
        ]
        .contains(&value.v_inttbl.to_owned())
        {
            value.v_credt.to_owned()
        } else {
            value.v_opneffdate
        };

        let mut v_inttblver = 0;
        let mut v_intver = 0;
        let icv_key = IcvKey {
            int_tbl_code: value.v_inttbl.to_owned(),
            crncy_code: value.v_crncy.to_owned(),
        };
        let mut int_version = 0;
        if icv_map.contains_key(&icv_key) {
            let icv_val = icv_map.get_mut(&icv_key).expect("Could not get ICV data.");
            for val in icv_val.iter() {
                let pass_dt = if vec![
                    "TGENR".to_string(),
                    "TSTFR".to_string(),
                    "TNRER".to_string(),
                    "TNRNR".to_string(),
                ]
                .contains(&value.v_inttbl.to_owned())
                {
                    val.lchg_time
                } else {
                    val.start_date
                };
                if pass_dt <= v_passdt
                    && val.end_date >= v_passdt
                    && val.int_tbl_ver_num > v_inttblver
                {
                    v_inttblver = val.int_tbl_ver_num.to_owned();
                    v_intver = val.int_version.to_owned();
                    int_version = val.int_version;
                }
            }
        }
        let v_datediff = num_days_start_to_end(value.v_opneffdate, value.v_matdate);
        //TVS
        let tvs_key = TvsKey {
            int_tbl_code: value.v_inttbl.to_owned(),
            crncy_code: value.v_crncy.to_owned(),
            int_tbl_ver_num: v_intver,
        };
        let mut v_nrmlint = 0.0;
        if tvs_map.contains_key(&tvs_key) {
            let tvs_value = tvs_map.get(&tvs_key).expect("Cannot get from TVS map.");
            let mut int_slab_srl_num = 9999;
            for val in tvs_value.iter() {
                let max_run_date = match val.max_period_run_days {
                    999 => {
                        let mut dt = incr_dt_by_mon_presrv_eom_checked(
                            value.v_opneffdate,
                            (val.max_period_run_mths + 1) as usize,
                        )
                        .unwrap_or(value.v_matdate);
                        let day1 = dt.day();
                        let day2 = value.v_opneffdate.day();
                        // Compare and get the minimum day
                        let min_day = if day1 < day2 { day1 } else { day2 };

                        // Set the minimum day in dt
                        dt = dt.with_day(min_day).unwrap_or(dt);
                        dt - Duration::days(1)
                    }
                    _ => {
                        let mut dt = incr_dt_by_mon_presrv_eom_checked(
                            value.v_opneffdate,
                            (val.max_period_run_mths) as usize,
                        )
                        .unwrap_or(value.v_matdate);
                        let day1 = dt.day();
                        let day2 = value.v_opneffdate.day();
                        // Compare and get the minimum day
                        let min_day = if day1 < day2 { day1 } else { day2 };

                        // Set the minimum day in dt
                        dt = dt.with_day(min_day).unwrap_or(dt);
                        dt + Duration::days(val.max_period_run_days)
                    }
                };
                if val.max_slab_amt > value.v_depamt
                    && (num_days_start_to_end(value.v_opneffdate, max_run_date) >= v_datediff)
                    && val.int_slab_srl_num < int_slab_srl_num
                {
                    int_slab_srl_num = val.int_slab_srl_num;
                    v_nrmlint = val.nrml_int_pcnt;
                }
            }
        }
        let final_int_rate = value.v_crpref + v_nrmlint;
        writeln!(
            op_writer,
            "{}||{}|{}|{}|{}|{}|{}|{}|{}||||||||{}|{}|{}|{}|{}|{}||{}|{}||{}|{}|{}",
            key,
            value.v_opneffdate.format("%d-%m-%Y"),
            value.v_schmtype,
            value.v_inttbl,
            int_version,
            value.int_tbl_code_srl_num,
            value.v_crpref,
            v_nrmlint,
            final_int_rate,
            value.v_perdmths,
            value.v_perddays,
            value.v_depamt,
            value.v_crncy,
            value.tam_deposit_type,
            value.tam_spl_catg_ind,
            value.itc_base_differential,
            value.tam_deposit_status,
            value.v_matdate.format("%d-%m-%Y"),
            value.v_credt,
            value.tam_auto_renewed_counter
        )
        .expect("Could not write to the output file.");
    }
}
