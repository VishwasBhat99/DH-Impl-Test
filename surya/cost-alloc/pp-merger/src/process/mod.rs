use self::io::*;
use crate::configuration_parameters::ConfigurationParameters;
use slog::Logger;
mod io;

use health_report::HealthReport;
use rbdate::DateParser;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

pub fn process(config_params: &ConfigurationParameters, _logger: &Logger, _diag_logger: &Logger) {
    let mut acc_enc = 0;
    let mut acc_proc = 0;
    let mut ip_amt = 0.0;
    let mut op_amt = 0.0;

    let soldim_reader =
        fs::read_to_string(config_params.soldim_file()).expect("Could Not Read SolDim File");
    let mut soldim_data: HashMap<String, Vec<&str>> = HashMap::new();

    for line in soldim_reader.lines().skip(1) {
        let vecsol = line.split(config_params.delimeter()).collect::<Vec<&str>>();
        soldim_data.insert(vecsol[0].to_string(), vecsol);
    }

    let divdim_reader =
        fs::read_to_string(config_params.divdim_file()).expect("Could Not Read DivDim File");
    let mut divdim_data: HashMap<String, Vec<&str>> = HashMap::new();
    for line in divdim_reader.lines().skip(1) {
        let vecdiv = line.split(config_params.delimeter()).collect::<Vec<&str>>();
        divdim_data.insert(vecdiv[0].to_string(), vecdiv);
    }

    let prddim_reader =
        fs::read_to_string(config_params.prddim_file()).expect("Could Not Read PrdDim File");
    let mut prddim_data: HashMap<String, Vec<&str>> = HashMap::new();

    for line in prddim_reader.lines().skip(1) {
        let vecprd = line.split(config_params.delimeter()).collect::<Vec<&str>>();
        prddim_data.insert(vecprd[0].to_string(), vecprd);
    }

    let mut op_writer = get_writer(config_params.cost_alloc_pp2_file_path());

    writeln!(
        op_writer,
        "BookDt|Unique ID|GLCode|GLDesc|SolLineID|SolName|SolType|SolCat1|SolCat2|SolCat3|SolCat4|SolCat5|HL-HO|HL-RO|HL-AD1|HL-AD2|HL-AD3|DivLineID|DivName|DivType|DivCat1|DivCat2|DivCat3|DivCat4|DivCat5|PrdLineID|PrdName|PrdType|PrdCat1|PrdCat2|PrdCat3|PrdCat4|PrdCat5|HL-Division1|HL-Division2|HL-AD1|HL-AD2|HL-AD3|AdLineid1|AdLineID2|Cycle|CCY|Cost Amount HCY|BRID|BRPROP1|BRPROP2"
    ).expect("Error in Writing Headers to Output File");

    let cost_data_pp1_reader = fs::read_to_string(config_params.cost_data_pp1_file())
        .expect("Could Not Read Cost Data PP1 File");
    let date_parser = DateParser::new("%d-%m-%Y".to_string(), false);
    for line in cost_data_pp1_reader.lines().skip(1) {
        acc_enc += 1;
        let veccostpp1 = line.split(config_params.delimeter()).collect::<Vec<&str>>();
        ip_amt += veccostpp1[veccostpp1.len() - 1].parse().unwrap_or(0.0);

        let book_dt = date_parser
            .parse_opt(veccostpp1[0])
            .unwrap_or(*config_params.as_on_date())
            .format("%d-%m-%Y");
        let mut unique_id = String::new();
        let unique_id_columns = config_params.unique_id_columns();
        for unique_id_col in unique_id_columns.split(',') {
            let unique_id_col: usize = unique_id_col
                .parse()
                .expect("Error in parsing unique_id_col");
            unique_id = format!("{}{}", unique_id, veccostpp1[unique_id_col - 1]);
        }
        let gl_code = if veccostpp1[1].is_empty() {
            ""
        } else {
            veccostpp1[1]
        };
        let gl_desc = if veccostpp1[2].is_empty() {
            ""
        } else {
            veccostpp1[2]
        };
        let solline_id = if veccostpp1[3].is_empty() {
            ""
        } else {
            veccostpp1[3]
        };

        let mut sol_name = "N/A";
        let mut sol_type = "N/A";
        let mut sol_cat1 = "N/A";
        let mut sol_cat2 = "N/A";
        let mut sol_cat3 = "N/A";
        let mut sol_cat4 = "N/A";
        let mut sol_cat5 = "N/A";
        let mut hl_ho = "N/A";
        let mut hl_ro = "N/A";
        let mut hl_ad1_s = "N/A";
        let mut hl_ad2_s = "N/A";
        let mut hl_ad3_s = "N/A";

        if soldim_data.contains_key(solline_id) {
            let vec_sol: Vec<&str> = soldim_data
                .get(solline_id as &str)
                .unwrap_or_else(|| {
                    panic!(
                        "Could not Sol-Dim-Data get data for SolLineID; {}",
                        solline_id
                    )
                })
                .to_vec();

            sol_name = if vec_sol[1].is_empty() {
                ""
            } else {
                vec_sol[1]
            };
            sol_type = if vec_sol[2].is_empty() {
                ""
            } else {
                vec_sol[2]
            };
            sol_cat1 = if vec_sol[3].is_empty() {
                ""
            } else {
                vec_sol[3]
            };
            sol_cat2 = if vec_sol[4].is_empty() {
                ""
            } else {
                vec_sol[4]
            };
            sol_cat3 = if vec_sol[5].is_empty() {
                ""
            } else {
                vec_sol[5]
            };
            sol_cat4 = if vec_sol[6].is_empty() {
                ""
            } else {
                vec_sol[6]
            };
            sol_cat5 = if vec_sol[7].is_empty() {
                ""
            } else {
                vec_sol[7]
            };
            hl_ho = if vec_sol[8].is_empty() {
                ""
            } else {
                vec_sol[8]
            };
            hl_ro = if vec_sol[9].is_empty() {
                ""
            } else {
                vec_sol[9]
            };
            hl_ad1_s = if vec_sol[10].is_empty() {
                ""
            } else {
                vec_sol[10]
            };
            hl_ad2_s = if vec_sol[11].is_empty() {
                ""
            } else {
                vec_sol[11]
            };
            hl_ad3_s = if vec_sol[12].is_empty() {
                ""
            } else {
                vec_sol[12]
            };
        } else if solline_id.is_empty() {
            sol_name = "";
            sol_type = "";
            sol_cat1 = "";
            sol_cat2 = "";
            sol_cat3 = "";
            sol_cat4 = "";
            sol_cat5 = "";
            hl_ho = "";
            hl_ro = "";
            hl_ad1_s = "";
            hl_ad2_s = "";
            hl_ad3_s = "";
        }

        let divline_id = if veccostpp1[4].is_empty() {
            ""
        } else {
            veccostpp1[4]
        };
        let mut div_name = "N/A";
        let mut div_type = "N/A";
        let mut div_cat1 = "N/A";
        let mut div_cat2 = "N/A";
        let mut div_cat3 = "N/A";
        let mut div_cat4 = "N/A";
        let mut div_cat5 = "N/A";

        if divdim_data.contains_key(divline_id) {
            let vec_div: Vec<&str> = divdim_data
                .get(divline_id as &str)
                .unwrap_or_else(|| {
                    panic!(
                        "Could not Div-Dim-Data get data for DivLineID; {}",
                        divline_id
                    )
                })
                .to_vec();

            div_name = if vec_div[1].is_empty() {
                ""
            } else {
                vec_div[1]
            };
            div_type = if vec_div[2].is_empty() {
                ""
            } else {
                vec_div[2]
            };
            div_cat1 = if vec_div[3].is_empty() {
                ""
            } else {
                vec_div[3]
            };
            div_cat2 = if vec_div[4].is_empty() {
                ""
            } else {
                vec_div[4]
            };
            div_cat3 = if vec_div[5].is_empty() {
                ""
            } else {
                vec_div[5]
            };
            div_cat4 = if vec_div[6].is_empty() {
                ""
            } else {
                vec_div[6]
            };
            div_cat5 = if vec_div[7].is_empty() {
                ""
            } else {
                vec_div[7]
            };
        } else if divline_id.is_empty() {
            div_name = "";
            div_type = "";
            div_cat1 = "";
            div_cat2 = "";
            div_cat3 = "";
            div_cat4 = "";
            div_cat5 = "";
        }

        let prdline_id = if veccostpp1[5].is_empty() {
            ""
        } else {
            veccostpp1[4]
        };
        let mut prd_name = "N/A";
        let mut prd_type = "N/A";
        let mut prd_cat1 = "N/A";
        let mut prd_cat2 = "N/A";
        let mut prd_cat3 = "N/A";
        let mut prd_cat4 = "N/A";
        let mut prd_cat5 = "N/A";
        let mut hl_division1 = "N/A";
        let mut hl_division2 = "N/A";
        let mut hl_ad1_p = "N/A";
        let mut hl_ad2_p = "N/A";
        let mut hl_ad3_p = "N/A";

        if prddim_data.contains_key(prdline_id) {
            let vec_prd: Vec<&str> = prddim_data
                .get(prdline_id as &str)
                .unwrap_or_else(|| {
                    panic!(
                        "Could not Prd-Dim-Data get data for PrdLineID; {}",
                        prdline_id
                    )
                })
                .to_vec();

            prd_name = if vec_prd[1].is_empty() {
                ""
            } else {
                vec_prd[1]
            };
            prd_type = if vec_prd[2].is_empty() {
                ""
            } else {
                vec_prd[2]
            };
            prd_cat1 = if vec_prd[3].is_empty() {
                ""
            } else {
                vec_prd[3]
            };
            prd_cat2 = if vec_prd[4].is_empty() {
                ""
            } else {
                vec_prd[4]
            };
            prd_cat3 = if vec_prd[5].is_empty() {
                ""
            } else {
                vec_prd[5]
            };
            prd_cat4 = if vec_prd[6].is_empty() {
                ""
            } else {
                vec_prd[6]
            };
            prd_cat5 = if vec_prd[7].is_empty() {
                ""
            } else {
                vec_prd[7]
            };
            hl_division1 = if vec_prd[8].is_empty() {
                ""
            } else {
                vec_prd[8]
            };
            hl_division2 = if vec_prd[9].is_empty() {
                ""
            } else {
                vec_prd[9]
            };
            hl_ad1_p = if vec_prd[10].is_empty() {
                ""
            } else {
                vec_prd[10]
            };
            hl_ad2_p = if vec_prd[11].is_empty() {
                ""
            } else {
                vec_prd[11]
            };
            hl_ad3_p = if vec_prd[12].is_empty() {
                ""
            } else {
                vec_prd[12]
            };
        } else if prdline_id.is_empty() {
            prd_name = "";
            prd_type = "";
            prd_cat1 = "";
            prd_cat2 = "";
            prd_cat3 = "";
            prd_cat4 = "";
            prd_cat5 = "";
            hl_division1 = "";
            hl_division2 = "";
            hl_ad1_p = "";
            hl_ad2_p = "";
            hl_ad3_p = "";
        }

        let adline_id1 = if veccostpp1[6].is_empty() {
            ""
        } else {
            veccostpp1[6]
        };
        let adline_id2 = if veccostpp1[7].is_empty() {
            ""
        } else {
            veccostpp1[7]
        };
        let cycle = if veccostpp1[8].is_empty() {
            ""
        } else {
            veccostpp1[8]
        };
        let ccy = if veccostpp1[9].is_empty() {
            ""
        } else {
            veccostpp1[9]
        };
        let costamount_hcy = if veccostpp1[10].is_empty() {
            "0"
        } else {
            veccostpp1[10]
        };
        let mut brid = "";
        let mut brprop1 = "";
        let brprop2 = "";
        if sol_type == "BR" {
            brid = solline_id;
            brprop1 = hl_ro;
        } else if sol_type == "RO" && hl_ro == "ALL" {
            brprop1 = solline_id;
            brid = hl_ro;
        } else if sol_type == "RO" {
            brprop1 = solline_id;
        }
        writeln!(
            op_writer,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            book_dt,
            unique_id,
            gl_code,
            gl_desc,
            solline_id,
            sol_name,
            sol_type,
            sol_cat1,
            sol_cat2,
            sol_cat3,
            sol_cat4,
            sol_cat5,
            hl_ho,
            hl_ro,
            hl_ad1_s,
            hl_ad2_s,
            hl_ad3_s,
            divline_id,
            div_name,
            div_type,
            div_cat1,
            div_cat2,
            div_cat3,
            div_cat4,
            div_cat5,
            prdline_id,
            prd_name,
            prd_type,
            prd_cat1,
            prd_cat2,
            prd_cat3,
            prd_cat4,
            prd_cat5,
            hl_division1,
            hl_division2,
            hl_ad1_p,
            hl_ad2_p,
            hl_ad3_p,
            adline_id1,
            adline_id2,
            cycle,
            ccy,
            costamount_hcy,
            brid,
            brprop1,
            brprop2
        ).expect("Error in Writing Data to Output File");
        acc_proc += 1;
        op_amt += costamount_hcy.parse().unwrap_or(0.0);
    }

    let health_report = HealthReport::new(acc_enc, acc_proc, acc_enc - acc_proc, ip_amt, op_amt, 0);
    health_report.gen_health_rpt(config_params.cost_alloc_pp2_file_path());
}
