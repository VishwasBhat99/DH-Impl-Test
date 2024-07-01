use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
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
    let mut acc_enctrd = 0;
    let mut acc_succ = 0;
    let data_parser = DateParser::new("%d-%m-%Y".to_string(), true);
    let mut op_writer = get_writer(config_param.output_file_path());

    //FEI
    let mut fei_map: HashMap<String, FeiData> = HashMap::new();
    let fei_file_reader = match new_buf_rdr(config_param.fei_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.fei_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    for (line_num, lines) in fei_file_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.fei_file_path(),
                line_num + 1,
                error
            ),
        };

        let fei_fields: Vec<&str> = line.split('|').collect();
        let val = FeiData {
            dc_ref_num: fei_fields[1].to_string(),
            issu_bank_code: fei_fields[2].to_string(),
            bank_name: fei_fields[3].to_string(),
            issu_branch_code: fei_fields[4].to_string(),
            other_bank_ref_num: fei_fields[5].to_string(),
        };
        fei_map.insert(fei_fields[0].to_string(), val);
    }

    //TFAT
    let tfat_file_reader = match new_buf_rdr(config_param.tfat_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.tfat_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut tfat_map: HashMap<String, String> = HashMap::new();
    for (line_num, lines) in tfat_file_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.tfat_file_path(),
                line_num + 1,
                error
            ),
        };
        match line.split_once('|') {
            Some((key, value)) => tfat_map.insert(key.to_string(), value.to_string()),
            None => None,
        };
    }
    // FBH
    let fbh_file_reader = match new_buf_rdr(config_param.fbh_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.fbh_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };
    let mut fbh_map: HashMap<FbhKey, FbhVal> = HashMap::new();
    for (line_num, lines) in fbh_file_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.fbh_file_path(),
                line_num + 1,
                error
            ),
        };
        let fbh_fields: Vec<&str> = line.split('|').collect();

        let fd_bod_date = data_parser.parse_opt(fbh_fields[27]).unwrap_or(
            NaiveDate::from_ymd_opt(1970, 1, 1).expect("Could not get default date format."),
        );
        let fbh_key = FbhKey {
            sol_id: fbh_fields[0].to_string(),
            bill_id: fbh_fields[1].to_string(),
        };
        let bill_func = fbh_fields[2].to_string();
        let entity_cre_flag = fbh_fields[4].to_string();
        let mut fbh_value = FbhVal::default();
        fbh_value.add_fbh_value(
            bill_func.to_owned(),
            fd_bod_date,
            entity_cre_flag.to_owned(),
            *config_param.as_on_date(),
        );
        fbh_map
            .entry(fbh_key)
            .and_modify(|val| {
                val.add_fbh_value(
                    bill_func,
                    fd_bod_date,
                    entity_cre_flag,
                    *config_param.as_on_date(),
                )
            })
            .or_insert(fbh_value);
    }

    //IDT
    let mut idt_map: HashMap<String, Vec<IdtValue>> = HashMap::new();
    let idt_file_reader = match new_buf_rdr(config_param.idt_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.idt_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    for (line_num, lines) in idt_file_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.idt_file_path(),
                line_num + 1,
                error
            ),
        };
        let idt_fields: Vec<&str> = line.split('|').collect();
        let value = IdtValue {
            int_rate: idt_fields[1].parse::<f64>().unwrap_or(0.0),
            int_amt: idt_fields[2].parse::<f64>().unwrap_or(0.0),
            int_type: idt_fields[4].to_string(),
        };

        idt_map
            .entry(idt_fields[0].to_string())
            .and_modify(|val| val.push(value.to_owned()))
            .or_insert(vec![value]);
    }

    // FAE
    let mut fae_map: HashMap<FaeKey, f64> = HashMap::new();
    let fae_file_reader = match new_buf_rdr(config_param.fae_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.fae_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    for (line_num, lines) in fae_file_reader.lines().enumerate() {
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.fae_file_path(),
                line_num + 1,
                error
            ),
        };

        let fae_fields: Vec<&str> = line.split('|').collect();
        let key = FaeKey {
            bill_id: fae_fields[0].to_string(),
            sol_id: fae_fields[1].to_string(),
        };
        fae_map.insert(key, fae_fields[2].parse::<f64>().unwrap_or(0.0));
    }

    //input pp output
    let input_file_reader = match new_buf_rdr(config_param.input_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_param.input_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    for (line_num, lines) in input_file_reader.lines().enumerate() {
        acc_enctrd += 1;
        let line = match lines {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_param.input_file_path(),
                line_num + 1,
                error
            ),
        };

        let input_fields: Vec<&str> = line.split('|').collect();
        let bill_id = input_fields[3].to_string();
        let sol_id = input_fields[1].to_string();
        let bill_b2kid = input_fields[21].to_string();
        let fei_default = FeiData::default();
        let fei_data = fei_map.get(&bill_id.to_owned()).unwrap_or(&fei_default);
        let lc_ref_number = &fei_data.dc_ref_num;
        let bk = &fei_data.issu_bank_code;
        let bknam = &fei_data.bank_name;
        let brcd = &fei_data.issu_branch_code;
        let act_lcnum = &fei_data.other_bank_ref_num;
        let default = "NA|NA".to_string();
        let loc_drawee = tfat_map.get(&bill_b2kid).unwrap_or(&default);
        let idt_default = vec![IdtValue::default()];
        let idt_val = idt_map.get(&bill_b2kid).unwrap_or(&idt_default);
        let mut rate = 0.0;
        let mut int_amt1 = 0.0;
        let mut int_amt = 0.0;
        for val in idt_val {
            int_amt += val.int_amt;
            if val.int_type.to_uppercase() == *"N" {
                rate = (val.int_rate * 100.0).round() / 100.0;
                int_amt1 = val.int_amt;
            }
        }

        if int_amt == 0.0 {
            let fae_key = FaeKey {
                bill_id: bill_id.to_owned(),
                sol_id: sol_id.to_owned(),
            };
            int_amt = *fae_map.get(&fae_key).unwrap_or(&0.0);
        }
        let fbh_key = FbhKey {
            sol_id: sol_id.to_owned(),
            bill_id: bill_id.to_owned(),
        };
        let fbh_default = FbhVal::default();
        let fbh_val = fbh_map.get(&fbh_key).unwrap_or(&fbh_default);
        let delink_cnt = fbh_val.count;
        let purdt = fbh_val.vfd_bod_date;

        let due_date = data_parser.parse_opt(input_fields[17]).unwrap_or(
            NaiveDate::from_ymd_opt(1970, 1, 1).expect("Could not get default date format."),
        );
        let lodg_date = data_parser.parse_opt(input_fields[4]).unwrap_or(
            NaiveDate::from_ymd_opt(1970, 1, 1).expect("Could not get default date format."),
        );

        let days = rbdate::num_days_start_to_end(lodg_date, due_date);
        let inr_amt = input_fields[15].parse::<f64>().unwrap_or(0.0);
        let inrat = (((int_amt * 365.0 * 100.0) / (inr_amt * days as f64)) * 100.0).round() / 100.0;

        writeln!(
            op_writer,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            input_fields[9],
            input_fields[10],
            input_fields[0],
            sol_id.to_owned(),
            input_fields[2],
            input_fields[11],
            input_fields[6],
            input_fields[7],
            input_fields[5],
            bill_id,
            input_fields[16],
            input_fields[12],
            input_fields[14],
            inr_amt,
            input_fields[19],
            int_amt,
            if rate!=0.0{
                rate
            }else{
                inrat
            },
            lodg_date,
            purdt,
            due_date,
            lc_ref_number,
            bk,
            bknam,
            brcd,
            delink_cnt,
            input_fields[22],
            input_fields[16],
            act_lcnum,
            loc_drawee
        )
        .expect("Could not write to the output file.");
        acc_succ += 1;
    }
    let health_report = HealthReport::new(acc_enctrd, acc_succ, acc_enctrd - acc_succ, 0.0, 0.0, 0);
    health_report.gen_health_rpt(config_param.output_file_path());
}
