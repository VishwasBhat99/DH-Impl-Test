use crate::process::structs::*;
use crate::process::ConfigurationParameters;
use chrono::Duration;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

pub fn write_ei_data(
    output_map: &mut HashMap<OPKeyEI, OPDataEI>,
    config_params: &ConfigurationParameters,
    op_writer: &mut BufWriter<File>,
    op_amt: &mut i64,
) {
    for (key, val) in output_map {
        match val.lr_freq_type.as_str() {
            "F" => {
                while val.flow_start_date <= *config_params.as_on_date() {
                    val.flow_start_date += Duration::days(14);
                }
            }
            "B" => {
                if val.flow_start_date <= *config_params.as_on_date() {
                    val.flow_start_date = *config_params.as_on_date();
                }
            }
            "M" => {
                while val.flow_start_date <= *config_params.as_on_date() {
                    val.flow_start_date =
                        rbdate::incr_dt_by_mon_presrv_eom_checked(val.flow_start_date, 1)
                            .unwrap_or(*config_params.as_on_date());
                }
            }
            "Q" => {
                while val.flow_start_date <= *config_params.as_on_date() {
                    val.flow_start_date =
                        rbdate::incr_dt_by_mon_presrv_eom_checked(val.flow_start_date, 3)
                            .unwrap_or(*config_params.as_on_date());
                }
            }
            "H" => {
                while val.flow_start_date <= *config_params.as_on_date() {
                    val.flow_start_date =
                        rbdate::incr_dt_by_mon_presrv_eom_checked(val.flow_start_date, 6)
                            .unwrap_or(*config_params.as_on_date());
                }
            }
            "Y" => {
                while val.flow_start_date <= *config_params.as_on_date() {
                    val.flow_start_date =
                        rbdate::incr_dt_by_mon_presrv_eom_checked(val.flow_start_date, 12)
                            .unwrap_or(*config_params.as_on_date());
                }
            }
            _ => val.flow_start_date = *config_params.as_on_date(),
        }
        writeln!(
            op_writer,
            "{}|{}|{}|{}|{}|{}|||{}||{}",
            key.acid,
            val.shdl_num,
            val.num_of_dmds,
            val.flow_start_date.format("%d-%m-%Y"),
            val.flow_amt,
            val.lr_freq_type,
            val.cf_code,
            val.num_of_flows,
        )
        .unwrap_or_else(|err| println!("Error in writing file: {:?}", err));
        *op_amt += val.flow_amt;
    }
}
