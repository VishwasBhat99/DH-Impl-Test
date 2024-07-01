use config_params::ConfigurationParameters;
use processing::structs::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

pub fn derive_values_prod(
    cust_dep_map: &mut HashMap<String, GroupVal>,
    cust_dep_prod_map: &mut HashMap<CustId, CustVal>,
    ucic_map: &HashMap<String, UcicDet>,
    field: Vec<&str>,
    ex_rt: &f64,
    ucic_id: String,
) {
    let sa_amt_hcy = ex_rt
        * field[4]
            .parse::<f64>()
            .expect(&format!("Could not parse {} as float", field[4]));
    let ca_amt_hcy = ex_rt
        * field[3]
            .parse::<f64>()
            .expect(&format!("Could not parse {} as float", field[3]));
    let td_wd_amt_hcy = ex_rt
        * (field[5]
            .parse::<f64>()
            .expect(&format!("Could not parse {} as float", field[5]))
            + field[7]
                .parse::<f64>()
                .expect(&format!("Could not parse {} as float", field[7])));
    let td_nwd_amt_hcy = ex_rt
        * field[6]
            .parse::<f64>()
            .expect(&format!("Could not parse {} as float", field[6]));
    let tot_hcy = sa_amt_hcy + ca_amt_hcy + td_wd_amt_hcy + td_nwd_amt_hcy;

    let cust_id = CustId {
        cust_id: ucic_id.to_string(),
        ccy: field[2].to_string(),
    };
    let mut cust_val: CustVal;
    if cust_dep_prod_map.contains_key(&cust_id) {
        let present_val = cust_dep_prod_map.get(&cust_id).unwrap();
        cust_val = CustVal {
            ca_amt_ccy: (present_val.ca_amt_ccy.parse::<f64>().unwrap()
                + field[3].parse::<f64>().unwrap())
            .to_string(),
            sa_amt_ccy: (present_val.sa_amt_ccy.parse::<f64>().unwrap()
                + field[4].parse::<f64>().unwrap())
            .to_string(),
            td_wd_amt_ccy: (present_val.td_wd_amt_ccy.parse::<f64>().unwrap()
                + field[5].parse::<f64>().unwrap())
            .to_string(),
            td_nwd_amt_ccy: (present_val.td_nwd_amt_ccy.parse::<f64>().unwrap()
                + field[6].parse::<f64>().unwrap())
            .to_string(),
            rd_ccy: (present_val.rd_ccy.parse::<f64>().unwrap() + field[7].parse::<f64>().unwrap())
                .to_string(),
            ca_amt_hcy: (present_val.ca_amt_hcy.parse::<f64>().unwrap() + ca_amt_hcy).to_string(),
            sa_amt_hcy: (present_val.sa_amt_hcy.parse::<f64>().unwrap() + sa_amt_hcy).to_string(),
            td_wd_amt_hcy: (present_val.td_wd_amt_hcy.parse::<f64>().unwrap() + td_wd_amt_hcy)
                .to_string(),
            td_nwd_amt_hcy: (present_val.td_nwd_amt_hcy.parse::<f64>().unwrap() + td_nwd_amt_hcy)
                .to_string(),
            ca_int_rt: field[8].to_string(),
            sa_int_rt: field[9].to_string(),
            td_wd_int_rt: field[10].to_string(),
            td_nwd_int_rt: field[11].to_string(),
        };
    } else {
        cust_val = CustVal {
            ca_amt_ccy: field[3].to_string(),
            sa_amt_ccy: field[4].to_string(),
            td_wd_amt_ccy: field[5].to_string(),
            td_nwd_amt_ccy: field[6].to_string(),
            rd_ccy: field[7].to_string(),
            ca_amt_hcy: ca_amt_hcy.to_string(),
            sa_amt_hcy: sa_amt_hcy.to_string(),
            td_wd_amt_hcy: td_wd_amt_hcy.to_string(),
            td_nwd_amt_hcy: td_nwd_amt_hcy.to_string(),
            ca_int_rt: field[8].to_string(),
            sa_int_rt: field[9].to_string(),
            td_wd_int_rt: field[10].to_string(),
            td_nwd_int_rt: field[11].to_string(),
        };
    }

    cust_dep_prod_map.insert(cust_id, cust_val);
    if cust_dep_map.contains_key(&ucic_id) {
        let mut grp_val = cust_dep_map.get(&ucic_id).unwrap().clone();
        grp_val.tot_hcy_amt = (grp_val.tot_hcy_amt.parse::<f64>().unwrap() + tot_hcy).to_string();
        cust_dep_map.insert(ucic_id.to_string(), grp_val);
    } else {
        let mut ucic_val = UcicDet {
            ucic_id: field[1].to_string(),
            ucic_name: "NONE".to_string(),
            cust_type: "NONE".to_string(),
        };
        if ucic_map.contains_key(field[1]) {
            ucic_val = ucic_map.get(field[1]).unwrap().clone();
        }

        let grp_val = GroupVal {
            cust_type: ucic_val.cust_type,
            cust_name: ucic_val.ucic_name,
            tot_hcy_amt: tot_hcy.to_string(),
        };
        cust_dep_map.insert(ucic_id.to_string(), grp_val);
    }
}

pub fn derive_values(cust_dep_map: &mut HashMap<String, GroupVal>, amt_vec: &mut Vec<f64>) {
    for (_, value) in cust_dep_map {
        amt_vec.push(value.tot_hcy_amt.parse::<f64>().unwrap());
    }
}

pub fn get_op_line(
    cust_dep_map: &mut HashMap<String, GroupVal>,
    amt_vec: &mut Vec<f64>,
    top_cust_count: &u32,
    op_line: &mut String,
    config_params: &ConfigurationParameters,
) {
    let mut cust_id_vec: Vec<String> = Vec::new();
    amt_vec.sort_by(|a, b| b.partial_cmp(a).unwrap());
    if (top_cust_count + 1) as usize <= amt_vec.len() {
        amt_vec.split_off((top_cust_count + 0) as usize);
    }
    let mut keys: Vec<String> = Vec::new();
    for (key, value) in cust_dep_map.clone() {
        if amt_vec.contains(&value.tot_hcy_amt.parse::<f64>().unwrap()) {
            cust_id_vec.push(key.to_string());
            op_line.push_str(config_params.country_code());
            op_line.push_str("|");
            op_line.push_str(&config_params.ason_date().to_string());
            op_line.push_str("|");
            op_line.push_str(&key);
            op_line.push_str("|");
            op_line.push_str(&value.cust_name);
            op_line.push_str("|");
            op_line.push_str(&value.cust_type);
            op_line.push_str("|");
            op_line.push_str(&value.tot_hcy_amt);
            op_line.push_str("\n")
        } else {
            keys.push(key.to_string());
        }
    }
    for key in keys {
        cust_dep_map.remove(&key);
    }
}

pub fn get_op_line_prod(
    cust_dep_prod_map: &HashMap<CustId, CustVal>,
    cust_dep_map: &HashMap<String, GroupVal>,
    config_params: &ConfigurationParameters,
    op_line_prod: &mut String,
) {
    for (key, value) in cust_dep_prod_map {
        if cust_dep_map.contains_key(&key.cust_id) {
            op_line_prod.push_str(config_params.country_code());
            op_line_prod.push_str("|");
            op_line_prod.push_str(&config_params.ason_date().to_string());
            op_line_prod.push_str("|");
            op_line_prod.push_str(&key.cust_id);
            op_line_prod.push_str("|");
            op_line_prod.push_str(&key.ccy);
            op_line_prod.push_str("|");
            op_line_prod.push_str(&value.sa_amt_hcy);
            op_line_prod.push_str("|");
            op_line_prod.push_str(&value.sa_amt_ccy);
            op_line_prod.push_str("|");
            op_line_prod.push_str(&value.ca_amt_hcy);
            op_line_prod.push_str("|");
            op_line_prod.push_str(&value.ca_amt_ccy);
            op_line_prod.push_str("|");
            op_line_prod.push_str(&value.td_wd_amt_hcy);
            op_line_prod.push_str("|");
            op_line_prod.push_str(&value.td_wd_amt_ccy);
            op_line_prod.push_str("|");
            op_line_prod.push_str(&value.td_nwd_amt_hcy);
            op_line_prod.push_str("|");
            op_line_prod.push_str(&value.td_nwd_amt_ccy);
            op_line_prod.push_str("|");
            op_line_prod.push_str(&value.sa_int_rt);
            op_line_prod.push_str("|");
            op_line_prod.push_str(&value.ca_int_rt);
            op_line_prod.push_str("|");
            op_line_prod.push_str(&value.td_wd_int_rt);
            op_line_prod.push_str("|");
            op_line_prod.push_str(&value.td_nwd_int_rt);
            op_line_prod.push_str("\n");
        }
    }
}

pub fn write(
    output_file: File,
    output_file_prod: File,
    output_line_prod: String,
    output_line: String,
) {
    let mut writer = BufWriter::new(output_file);
    let mut writer_prod = BufWriter::new(output_file_prod);

    match writer.write_all(output_line.as_bytes()) {
        Ok(_val) => println!("Successfully processed all accounts"),
        Err(error) => {
            panic!("Cannot pre process the input file: {:?}", error);
        }
    }

    match writer_prod.write_all(output_line_prod.as_bytes()) {
        Ok(_val) => println!("Successfully processed all accounts"),
        Err(error) => {
            panic!("Cannot pre process the input file: {:?}", error);
        }
    }
}
