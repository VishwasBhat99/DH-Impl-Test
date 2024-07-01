pub mod derive_fields;
pub mod structs;
use sdb_io::new_buf_rdr;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use config_params::ConfigurationParameters;
use health_report::HealthReport;
use processing::derive_fields::*;
use processing::structs::*;
use slog::Logger;

pub fn process_data(config_params: &ConfigurationParameters, log: &Logger, diag_log: &Logger) {
    let mut ttl_accnts: usize = 0;
    let mut ttl_suc_accnts: usize = 0;
    let mut ttl_bal_ip = 0.0;
    let mut ttl_bal_op = 0.0;
    let mut output_line_prod = String::new();
    let mut output_line = String::new();
    let op_path = config_params.output_file().to_string() + &".txt".to_string();
    let op_path_prod = config_params.output_file().to_string() + &"_prod.txt".to_string();
    let output_file = match File::create(op_path) {
        Ok(val) => val,
        Err(error) => panic!("{}", error),
    };
    let output_file_prod = match File::create(op_path_prod) {
        Ok(val) => val,
        Err(error) => panic!("{}", error),
    };
    let mut cust_dep_map: HashMap<String, GroupVal> = HashMap::new();
    let mut cust_dep_prod_map: HashMap<CustId, CustVal> = HashMap::new();
    let ucic_file = match new_buf_rdr(config_params.ucic_map_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` : {}.",
            config_params.ucic_map_file(),
            error
        ),
    };
    let mut ucic_cust_map: HashMap<String, UcicDet> = HashMap::new();
    for (idx, line) in ucic_file.lines().enumerate() {
        let ucic_det_line = line.expect(&format!("Cannot read line at {}", idx));
        let fields: Vec<&str> = ucic_det_line.split("~#~").collect();
        let ucic_data = UcicDet {
            ucic_id: fields[3].to_string(),
            ucic_name: fields[2].to_string(),
            cust_type: fields[1].to_string(),
        };
        if !ucic_cust_map.contains_key(fields[0]) {
            ucic_cust_map.insert(fields[0].to_string(), ucic_data);
        }
    }
    info!(
        diag_log,
        "End of creating hashtable for custid and ucic details mapping."
    );
    let exrt_file = match new_buf_rdr(config_params.exrt_file()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` : {}.",
            config_params.exrt_file(),
            error
        ),
    };
    let mut exrt_map: HashMap<Exrt, f64> = HashMap::new();
    for (idx, line) in exrt_file.lines().enumerate() {
        let exrt_line = line.expect(&format!("Cannot read line at {}", idx));
        let fields: Vec<&str> = exrt_line.split("|").collect();
        let exrt = Exrt {
            from_ccy: fields[0].to_string(),
            to_ccy: fields[1].to_string(),
        };
        if !exrt_map.contains_key(&exrt) {
            exrt_map.insert(exrt, fields[2].to_string().parse::<f64>().unwrap_or(0.0));
        }
    }
    info!(
        diag_log,
        "End of creating hashtable for exchange rate mapping."
    );

    let input_file = match File::open(config_params.input_file()) {
        Ok(input_file) => input_file,
        Err(error) => panic!("{}", error),
    };
    let reader = BufReader::new(input_file);
    for (line_no, line) in reader.lines().enumerate() {
        let acc_info: String = match line {
            Ok(acc_info) => acc_info,
            Err(error) => {
                panic!("Cannot read line from input file: {:?}", error);
            }
        };
        let fields: Vec<&str> = acc_info.split('|').collect();
        ttl_accnts += 1;
        let exchange = Exrt {
            from_ccy: fields[2].to_string(),
            to_ccy: config_params.currency_id.to_string(),
        };
        let mut ucic_value = &UcicDet::default();
        if ucic_cust_map.contains_key(fields[1]) {
            ucic_value = ucic_cust_map
                .get(fields[1])
                .expect("could not get data for cust id");
        }
        let ucic_id = ucic_value.ucic_id.to_string();
        let ex_rt = exrt_map.get(&exchange).expect(&format!(
            "Exchnage Rate not found for {} to {}",
            exchange.from_ccy, exchange.to_ccy
        ));
        derive_values_prod(
            &mut cust_dep_map,
            &mut cust_dep_prod_map,
            &ucic_cust_map,
            fields,
            ex_rt,
            ucic_id,
        );
    }
    let mut amt_vec: Vec<f64> = Vec::new();
    derive_values(&mut cust_dep_map, &mut amt_vec);

    get_op_line(
        &mut cust_dep_map,
        &mut amt_vec,
        config_params.top_cust_count(),
        &mut output_line,
        config_params,
    );
    get_op_line_prod(
        &cust_dep_prod_map,
        &cust_dep_map,
        config_params,
        &mut output_line_prod,
    );
    write(output_file, output_file_prod, output_line_prod, output_line);
    // Generating health report.
    let health_report = HealthReport::new(
        ttl_accnts as i64,
        ttl_suc_accnts as i64,
        (ttl_accnts - ttl_suc_accnts) as i64,
        ttl_bal_ip,
        ttl_bal_op,
        0,
    );
    health_report.display();
    health_report.gen_health_rpt(&config_params.output_file());
}
