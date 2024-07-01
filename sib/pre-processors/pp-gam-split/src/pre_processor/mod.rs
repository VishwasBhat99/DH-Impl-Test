use self::exrt::*;
use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use macros;
use sdb_io::buf_file_wrtr;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use std::env::current_dir;
use std::io::BufRead;
use std::io::Write;
use std::time::SystemTime;
mod exrt;

pub fn process(config_param: ConfigurationParameters, log: &Logger, _diag_log: &Logger) {
    let mut acc_enc = 0;
    let mut acc_proc = 0;
    let mut ip_amt = 0.0;
    let mut op_amt = 0.0;

    struct CmgVal {
        cust_name: String,
        pan_gir_num: String,
        cust_const: String,
    }
    let mut writer = match buf_file_wrtr(config_param.output_file_path(), None) {
        Ok(file) => file,
        Err(error) => panic!(
            "Unable to create file `{}` on location `{}` : {}",
            config_param.output_file_path(),
            current_dir()
                .expect("Unable to get current directory path.")
                .display(),
            error
        ),
    };
    let start_gac_read_timer = SystemTime::now();
    let mut gac_map: HashMap<String, String> = HashMap::new();
    let gac_file = match new_buf_rdr(config_param.gac_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.gac_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in gac_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.gac_file_path(),
                line_num + 1,
                error
            ),
        };
        match line.split_once('|') {
            Some((key, value)) => gac_map.insert(key.to_string(), value.to_string()),
            None => None,
        };
    }
    let end_gac_read_timer = SystemTime::now();
    let duration = end_gac_read_timer
        .duration_since(start_gac_read_timer)
        .expect("Could not calculate total duration for gac read timer.");
    debug!(log, "Readings GAC File, Total Duration: {:?}.", duration);

    let start_cmg_read_timer = SystemTime::now();
    let mut cmg_map: HashMap<String, CmgVal> = HashMap::new();
    let cmg_file = match new_buf_rdr(config_param.cmg_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.cmg_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    for (line_num, lines) in cmg_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.cmg_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        cmg_map.insert(
            fields[0].to_string(),
            CmgVal {
                cust_name: get_str(config_param.cmg_file_path(), &fields, 1, line_num),
                pan_gir_num: get_str(config_param.cmg_file_path(), &fields, 2, line_num),
                cust_const: get_str(config_param.cmg_file_path(), &fields, 3, line_num),
            },
        );
    }
    let end_cmg_read_timer = SystemTime::now();
    let duration = end_cmg_read_timer
        .duration_since(start_cmg_read_timer)
        .expect("Could not calculate total duration for read timer.");
    debug!(log, "Reading CMG File, Total Duration: {:?}.", duration);

    let start_gsp_read_timer = SystemTime::now();
    let mut gsp_map: HashMap<String, (String, String)> = HashMap::new();
    let gsp_file = match new_buf_rdr(config_param.gsp_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.gsp_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in gsp_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.gsp_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        gsp_map.insert(
            get_str(config_param.gsp_file_path(), &fields, 0, line_num),
            (
                get_str(config_param.gsp_file_path(), &fields, 1, line_num),
                get_str(config_param.gsp_file_path(), &fields, 2, line_num),
            ),
        );
    }
    let end_gsp_read_timer = SystemTime::now();
    let duration = end_gsp_read_timer
        .duration_since(start_gsp_read_timer)
        .expect("Could not calculate total duration for read timer.");
    debug!(log, "Reading GSP File, Total Duration: {:?}.", duration);

    let start_itc_read_timer = SystemTime::now();
    let mut itc_map: HashMap<String, String> = HashMap::new();
    let itc_file = match new_buf_rdr(config_param.itc_file_path()) {
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
    for (line_num, lines) in itc_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.itc_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        itc_map.insert(
            get_str(config_param.itc_file_path(), &fields, 0, line_num),
            get_str(config_param.itc_file_path(), &fields, 2, line_num),
        );
    }
    let end_itc_read_timer = SystemTime::now();
    let duration = end_itc_read_timer
        .duration_since(start_itc_read_timer)
        .expect("Could not calculate total duration for read timer.");
    debug!(log, "Reading ITC File, Total Duration: {:?}.", duration);

    let start_eab_read_timer = SystemTime::now();
    let eab_file = match new_buf_rdr(config_param.eab_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.eab_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    struct EabData {
        tran_date_bal: f64,
        acct_crncy_code: String,
    }
    let mut eab_map: HashMap<String, EabData> = HashMap::new();
    for (line_num, lines) in eab_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.eab_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        eab_map.insert(
            fields[4].trim().to_string(),
            EabData {
                tran_date_bal: get_str(config_param.eab_file_path(), &fields, 0, line_num)
                    .parse::<f64>()
                    .unwrap_or(0.0),
                acct_crncy_code: get_str(config_param.eab_file_path(), &fields, 1, line_num),
            },
        );
    }
    let end_eab_read_timer = SystemTime::now();
    let duration = end_eab_read_timer
        .duration_since(start_eab_read_timer)
        .expect("Could not calculate total duration for read timer.");
    debug!(log, "Reading eab File, Total Duration: {:?}.", duration);

    let start_ex_rt_read_timer = SystemTime::now();
    let mut ex_rt_map: HashMap<ExrtKey, String> = HashMap::new();
    let ex_rt_file = match new_buf_rdr(config_param.ex_rt_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.ex_rt_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    for (line_num, lines) in ex_rt_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.ex_rt_file_path(),
                line_num + 1,
                error
            ),
        };
        let derived_fields: Vec<&str> = line.split('|').collect();
        let ex_rt_key = ExrtKey::new(
            get_str(config_param.ex_rt_file_path(), &derived_fields, 0, line_num),
            get_str(config_param.ex_rt_file_path(), &derived_fields, 1, line_num),
        );
        ex_rt_map.insert(
            ex_rt_key,
            get_str(config_param.ex_rt_file_path(), &derived_fields, 2, line_num),
        );
    }
    let end_ex_rt_read_timer = SystemTime::now();
    let duration = end_ex_rt_read_timer
        .duration_since(start_ex_rt_read_timer)
        .expect("Could not calculate total duration for read timer.");
    log_debug!(
        log,
        "Reading EXCHANGE RATE File, Total Duration: {:?}.",
        duration
    );

    let start_gam_read_timer = SystemTime::now();
    let gam_file = match new_buf_rdr(config_param.gam_file_path()) {
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

    let mut ex_rt;
    let mut nfs;

    let mapping_master_file = match new_buf_rdr(config_param.mapping_master_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.mapping_master_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    #[derive(PartialEq, Clone, Default)]
    struct MapMaster {
        c_d: String,
        group: String,
        llg: String,
        other_llg: String,
    }
    let mut mapping_master_map: HashMap<String, Vec<MapMaster>> = HashMap::new();
    for (line_num, lines) in mapping_master_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.mapping_master_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        let mapping_acc = MapMaster {
            c_d: get_str(
                config_param.mapping_master_file_path(),
                &fields,
                3,
                line_num,
            ),
            group: get_str(
                config_param.mapping_master_file_path(),
                &fields,
                4,
                line_num,
            ),
            llg: get_str(
                config_param.mapping_master_file_path(),
                &fields,
                5,
                line_num,
            ),
            other_llg: get_str(
                config_param.mapping_master_file_path(),
                &fields,
                5,
                line_num,
            ),
        };
        mapping_master_map
            .entry(fields[0].to_string())
            .and_modify(|mapping_val: &mut Vec<MapMaster>| mapping_val.push(mapping_acc.to_owned()))
            .or_insert_with(|| vec![mapping_acc.to_owned()]);
    }

    let start_crm_user_read_timer = SystemTime::now();
    let crm_user_file = match new_buf_rdr(config_param.crm_user_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` due to: {}.",
            config_param.crm_user_file_path(),
            error
        ),
    };

    let mut crm_user_map: HashMap<String, String> = HashMap::new();
    for (line_num, lines) in crm_user_file.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.crm_user_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line.split('|').collect();
        crm_user_map.insert(
            get_str(config_param.crm_user_file_path(), &fields, 1, line_num),
            get_str(config_param.crm_user_file_path(), &fields, 2, line_num),
        );
    }
    let end_crm_user_read_timer = SystemTime::now();
    let duration = end_crm_user_read_timer
        .duration_since(start_crm_user_read_timer)
        .expect("Could not calculate total duration for read timer.");
    debug!(
        log,
        "Reading CRM User File, Total Duration: {:?}.", duration
    );

    for (line_num, lines) in gam_file.lines().enumerate() {
        let gam_line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.gam_file_path(),
                line_num + 1,
                error
            ),
        };

        let fields: Vec<&str> = gam_line.split('|').collect();
        acc_enc += 1;

        let default_cmg_fields = CmgVal {
            cust_name: "".to_string(),
            pan_gir_num: "".to_string(),
            cust_const: "".to_string(),
        };
        let cmg_fields = match cmg_map.get(&get_str(
            config_param.gam_file_path(),
            &fields,
            6,
            line_num,
        )) {
            Some(val) => val,
            None => {
                log_debug!(log,"The lookup key:{} could not be found in CMG file.Empty fields passed for account with id: {} is skipped.",get_str(config_param.gam_file_path(), &fields, 6, line_num),get_str(config_param.gam_file_path(), &fields, 0, line_num));
                &default_cmg_fields
            }
        };
        let default_eab_val = &EabData {
            tran_date_bal: 0.0,
            acct_crncy_code: "INR".to_string(),
        };
        let (eab_clr_bal_amt, acct_crncy_code) =
            match eab_map.get(&get_str(config_param.gam_file_path(), &fields, 0, line_num)) {
                Some(eab_val) => (eab_val.tran_date_bal, eab_val.acct_crncy_code.to_owned()),
                None => (
                    default_eab_val.tran_date_bal,
                    default_eab_val.acct_crncy_code.to_owned(),
                ),
            };
        ip_amt += eab_clr_bal_amt;
        if acct_crncy_code == "INR" {
            ex_rt = 1.0
        } else {
            let default_exrt_val = "1".to_string();
            let get_exrt_key = ExrtKey::new(acct_crncy_code.to_string(), "INR".to_string());
            ex_rt = ex_rt_map
                .get(&get_exrt_key)
                .unwrap_or(&default_exrt_val)
                .parse::<f64>()
                .unwrap_or(0.0);
        }
        let nfs_key = &get_str(config_param.gam_file_path(), &fields, 5, line_num)[0..3];
        if nfs_key == "NFS" {
            nfs = "NFS".to_string();
        } else {
            nfs = "".to_string();
        }

        let default_gsp_val = (&"".to_string(), &"".to_string());
        let (staff_schm_flg, nre_schm_flg) = match gsp_map.get(&get_str(
            config_param.gam_file_path(),
            &fields,
            14,
            line_num,
        )) {
            Some((ssf, nsf)) => (ssf, nsf),
            None => default_gsp_val,
        };
        let mut bmid = "".to_string();
        if let Some(id) = itc_map.get(&get_str(config_param.gam_file_path(), &fields, 0, line_num))
        {
            bmid = id.to_string()
        }

        let mut group = "".to_string();
        let mut llg = "".to_string();
        let mut other_llg = "".to_string();
        match mapping_master_map.get(&get_str(
            config_param.gam_file_path(),
            &fields,
            13,
            line_num,
        )) {
            Some(master_val) => {
                for val in master_val.iter() {
                    if (eab_clr_bal_amt < 0.0 && val.c_d.to_uppercase() == 'C'.to_string())
                        || (eab_clr_bal_amt >= 0.0 && val.c_d.to_uppercase() == 'D'.to_string())
                    {
                        group = val.group.to_owned();
                        llg = val.llg.to_owned();
                        other_llg = val.other_llg.to_owned();
                    }
                }
            }
            None => {
                log_error!(
                    log,
                    "Cannot fetch mapping master data for acc:{}",
                    fields[13]
                )
            }
        };
        //ACID is lookup key for gac_map.
        let default_gac_value = "||||||".to_string();
        let gac_line = match gac_map.get(&get_str(config_param.gam_file_path(), &fields, 0, line_num)) {
            Some(val) => val.to_string(),
            None => {
                log_debug!(
                    log,
                    "The lookup key:{} could not be found in GAC file. Empty fields passed.",
                    fields[0]
                );
                default_gac_value.to_string()
            }
        };
        let gac_fields: Vec<&str> = gac_line.split('|').collect();
        gac_map.remove(&get_str(config_param.gam_file_path(), &fields, 0, line_num));

        let gam_clr_bal_amt = get_str(config_param.gam_file_path(), &fields, 3, line_num)
            .parse::<f64>()
            .unwrap_or(0.0);
        acc_proc += 1;
        op_amt += eab_clr_bal_amt;
        writeln!(writer,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}||||{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|||{}|{}|{}|{}",
            get_str(config_param.gam_file_path(), &fields, 0, line_num),
            get_str(config_param.gam_file_path(), &fields, 1, line_num),
            get_str(config_param.gam_file_path(), &fields, 2, line_num),
            eab_clr_bal_amt,
            get_str(config_param.gam_file_path(), &fields, 4, line_num),
            get_str(config_param.gam_file_path(), &fields, 5, line_num),
            get_str(config_param.gam_file_path(), &fields, 6, line_num),
            get_str(config_param.gam_file_path(), &fields, 7, line_num),
            get_str(config_param.gam_file_path(), &fields, 8, line_num),
            get_str(config_param.gam_file_path(), &fields, 9, line_num),
            get_str(config_param.gam_file_path(), &fields, 10, line_num),
            get_str(config_param.gam_file_path(), &fields, 11, line_num),
            get_str(config_param.gam_file_path(), &fields, 12, line_num),
            get_str(config_param.gam_file_path(), &fields, 13, line_num),
            get_str(config_param.gam_file_path(), &fields, 14, line_num),
            get_str(config_param.gam_file_path(), &fields, 15, line_num),
            nre_schm_flg,
            acct_crncy_code,
            get_str(config_param.gam_file_path(), &fields, 18, line_num),
            get_str(config_param.gam_file_path(), &fields, 19, line_num),
            get_str(config_param.gam_file_path(), &fields, 20, line_num),
            get_str(config_param.gam_file_path(), &fields, 21, line_num),
            get_str(config_param.gam_file_path(), &fields, 22, line_num),
            get_str(config_param.gam_file_path(), &fields, 23, line_num),
            get_str(config_param.gam_file_path(), &fields, 24, line_num),
            get_str(config_param.gam_file_path(), &fields, 25, line_num),
            get_str(config_param.gam_file_path(), &fields, 26, line_num),
            get_str(config_param.gam_file_path(), &fields, 27, line_num),
            get_str(config_param.gam_file_path(), &fields, 28, line_num),
            get_str(config_param.gam_file_path(), &fields, 29, line_num),
            get_str(config_param.gam_file_path(), &fields, 30, line_num),
            get_str(config_param.gam_file_path(), &fields, 31, line_num),
            bmid,
            crm_user_map.get(&get_str(config_param.gam_file_path(), &fields, 6, line_num)).unwrap_or(&"BBB".to_string()),
            gac_fields.first().expect("Error deriving Sector-Code from GAC Data"),
            gac_fields.get(1).expect("Error deriving Sub-Sector-Code from GAC Data"),
            gac_fields.get(2).expect("Error deriving Type-of-Advn from GAC Data"),
            gac_fields.get(3).expect("Error deriving Mode-of-Advn from GAC Data"),
            gac_fields.get(4).expect("Error deriving Purpose-of-Advn from GAC Data"),
            gac_fields.get(5).expect("Error deriving Industry-Type from GAC Data"),
            gac_fields.get(6).expect("Error deriving Match-Opp-Rate from GAC Data"),
            cmg_fields.cust_name,
            cmg_fields.pan_gir_num,
            cmg_fields.cust_const,
            group,
            llg,
            other_llg,
            staff_schm_flg,
            ex_rt,
            gam_clr_bal_amt,
            gam_clr_bal_amt * ex_rt,
            nfs
        ).expect("Cannot write to output file.");
    }
    let end_gam_read_timer = SystemTime::now();
    let duration = end_gam_read_timer
        .duration_since(start_gam_read_timer)
        .expect("Could not calculate total duration for read timer.");
    debug!(
        log,
        "Reading and processing GAM File, Total Duration: {:?}.", duration
    );

    let health_report = HealthReport::new(acc_enc, acc_proc, acc_enc - acc_proc, ip_amt, op_amt, 0);
    println!("{}", health_report.display());
    health_report.gen_health_rpt(config_param.output_file_path());
}

pub fn get_str(input_file: &str, data: &[&str], index: usize, row: usize) -> String {
    data.get(index)
        .unwrap_or_else(|| {
            panic!(
                "Could not get data at column-no: `{}` in row-no: `{:?}` from File: {}",
                index + 1,
                row + 1,
                input_file,
            )
        })
        .trim()
        .trim_matches(|pat| pat == ' ' || pat == '"')
        .to_string()
}
