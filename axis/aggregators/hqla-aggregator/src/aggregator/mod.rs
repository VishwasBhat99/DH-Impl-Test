use self::input::*;
use self::readers::{read_hqla, read_manual};
use configuration_parameters::ConfigurationParameters;
use sdb_day_convention::accrued_days_with_convn;
use sdb_io::buf_file_wrtr;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::{
    env::current_dir,
    io::{BufRead, Write},
};

mod input;
mod output;
mod readers;
mod required_manual_fields;

pub fn aggregate(config_params: &ConfigurationParameters, logger: &Logger, _diag_logger: &Logger) {
    let mut categories_map: HashMap<String, HashMap<SecurityKey, SecurityValue>> = HashMap::new();
    let mut other_category_map: HashMap<SecurityKey, SecurityValue> = HashMap::new();

    let sec_all_file = match new_buf_rdr(config_params.input_sec_all_file_path()) {
        Ok(file) => file,
        Err(error) => panic!(
            "Could not found file `{}` on location `{}` : {}.",
            config_params.input_sec_all_file_path(),
            current_dir()
                .expect("Error while getting current directory path.")
                .display(),
            error
        ),
    };

    for (line_num, line) in sec_all_file.lines().enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(error) => panic!(
                "Unable to read file `{}` at line number: `{}` : {}",
                config_params.input_sec_all_file_path(),
                line_num + 1,
                error
            ),
        };
        let fields: Vec<&str> = line
            .split(config_params.input_delimiter())
            .map(|s| s.trim())
            .collect();

        let mut category = fields[2];
        if category.starts_with("HFT") {
            category = "HFT";
        } else if category.contains("REPO") {
            category = "FHFT";
        }        
        if config_params
            .book_categories()
            .contains(&category.to_string())
        {
            if config_params
                .book_categories()
                .contains(&category.to_string())
            {
                let mut map = categories_map
                    .entry(category.to_string())
                    .or_insert_with(HashMap::new);

                add_key_to_hashmap(&mut map, fields.clone(), config_params, logger);
            }
        } else {
            add_key_to_hashmap(
                &mut other_category_map,
                fields.clone(),
                config_params,
                logger,
            );
        }
    }
    let securities_file_vec = vec![
        config_params.input_sec_cblo_file_path(),
        config_params.input_sec_ccil_file_path(),
        config_params.input_sec_repo_file_path(),
    ];

    for file_name in securities_file_vec {
        let file = match new_buf_rdr(file_name) {
            Ok(file) => file,
            Err(error) => panic!(
                "Could not found file `{}` on location `{}` : {}.",
                file_name,
                current_dir()
                    .expect("Error while getting current directory path.")
                    .display(),
                error
            ),
        };
        for (line_num, line) in file.lines().enumerate() {
            let line = match line {
                Ok(line) => line,
                Err(error) => panic!(
                    "Unable to read file `{}` at line number: `{}` : {}",
                    file_name,
                    line_num + 1,
                    error
                ),
            };

            let fields: Vec<&str> = line
                .split(config_params.input_delimiter())
                .map(|s| s.trim())
                .collect();
            let mut category = fields[2];
            if category.starts_with("HFT") {
                category = "HFT";
            } else if category.contains("REPO") {
                category = "FHFT";
            }            
            let sec_key = SecurityKey {
                isin_no: fields[1].to_string(),
                book_category: category.to_string(),
            };
            if config_params
                .book_categories()
                .contains(&category.to_string())
            {
                categories_map
                    .entry(category.to_string())
                    .or_insert_with(HashMap::new)
                    .entry(sec_key)
                    .and_modify(|x| x.add_value(&fields));
            } else {
                other_category_map
                    .entry(sec_key)
                    .and_modify(|x| x.add_value(&fields));
            }
        }
        for (category, mut map) in categories_map.clone() {
            calculate_values(&mut map);
            categories_map.insert(category, map);
        }
        calculate_values(&mut other_category_map);
    }

    let mut categories_vec: HashMap<String, Vec<_>> = HashMap::new();

    for category in config_params.book_categories() {
        match categories_map.get(category) {
            Some(map) => {
                let mut category_vec: Vec<_> = map.into_iter().collect();
                category_vec.sort_by(|a, b| {
                    if a.1.appr_depr <= b.1.appr_depr {
                        Ordering::Greater
                    } else if a.1.appr_depr > b.1.appr_depr {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                });
                categories_vec.insert(category.to_string(), category_vec);
            }
            None => continue,
        }
    }

    let mut other_category_vec: Vec<_> = other_category_map.into_iter().collect();
    other_category_vec.sort_by(|a, b| {
        if a.1.appr_depr <= b.1.appr_depr {
            Ordering::Greater
        } else if a.1.appr_depr > b.1.appr_depr {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });

    let manual_data = read_manual(config_params, logger);
    let hqla_data = read_hqla(config_params, logger);

    let mut excess_slr_gsec = manual_data.slrgsec_maintained - manual_data.slrrequired;
    let mut msf = manual_data.slrrequired / hqla_data.req_slrperc * hqla_data.req_msfperc;
    let mut min_val = manual_data.lending_to_nbfchfc;
    if manual_data.fallcrceiling < min_val {
        min_val = manual_data.fallcrceiling;
    }
    let mut fallcr =
        (manual_data.slrrequired / hqla_data.req_slrperc * hqla_data.req_fallcrperc) + min_val;
    let mut row_no = 0;
    let as_on_dt = config_params.as_on_date();
    let mut slr_type: &str;
    let mut final_req: f64;
    let mut final_req_dirty: f64;
    let mut final_req_mv: f64;
    let mut chnmsf_flag = false;
    let mut chnfallcr_flag = false;
    let mut chnnone_flag = false;
    let mut output_writer = match buf_file_wrtr(config_params.output_file_path(), None) {
        Ok(val) => val,
        Err(error) => {
            panic!(
                "Could not create file: `{}` on location `{}` : {:?}.",
                config_params.output_file_path(),
                env::current_exe()
                    .expect("Unable to find current directory path!")
                    .display(),
                error
            );
        }
    };
    for category in config_params.book_categories() {
        match categories_vec.get(category) {
            Some(category_vec) => {
                for data in category_vec.into_iter() {
                    let mut final_rem = 0.0;
                    let mut final_rem_dirty = 0.0;
                    let mut final_rem_mv = 0.0;
                    let convention = config_params.accrued_day_convention();
                    let accrued_days_conv =
                        accrued_days_with_convn(*as_on_dt, data.1.maturity_date, 6, convention)
                            .unwrap();
                    let accrued_days = accrued_days_conv.days_btw_dts;
                    let accrued_interest =
                        (accrued_days as f64 * data.1.coupon * data.1.pledged_face_value)
                            / (accrued_days_conv.day_in_yr * 100) as f64;
                    let dirty_price = data.1.out_market_value + accrued_interest;
                    final_req = data.1.out_book_value;

                    if chnmsf_flag && chnfallcr_flag && chnnone_flag {
                        final_req = 0.0;
                        final_req_dirty = 0.0;
                        final_req_mv = 0.0;
                        slr_type = "NONE";
                    } else if chnmsf_flag && chnfallcr_flag {
                        if fallcr <= final_req {
                            let req_proportion = fallcr / final_req;
                            final_rem = final_req - fallcr;
                            final_req = fallcr;
                            let rem_proportion = final_rem / data.1.out_book_value;
                            chnnone_flag = true;
                            slr_type = "CHNNONE";
                            final_req_dirty = dirty_price * req_proportion;
                            final_rem_dirty = dirty_price * rem_proportion;
                            final_req_mv = data.1.out_market_value * req_proportion;
                            final_rem_mv = data.1.out_market_value * rem_proportion;
                        } else {
                            fallcr -= final_req;
                            slr_type = "FALLCR";
                            final_req_dirty = dirty_price;
                            final_req_mv = data.1.out_market_value;
                        }
                    } else if chnmsf_flag {
                        if msf <= final_req {
                            let req_proportion = msf / final_req;
                            final_rem = final_req - msf;
                            fallcr -= final_rem;
                            final_req = msf;
                            let rem_proportion = final_rem / data.1.out_book_value;

                            chnfallcr_flag = true;
                            slr_type = "CHNFALLCR";
                            final_req_dirty = dirty_price * req_proportion;
                            final_rem_dirty = dirty_price * rem_proportion;
                            final_req_mv = data.1.out_market_value * req_proportion;
                            final_rem_mv = data.1.out_market_value * rem_proportion;
                        } else {
                            msf -= final_req;
                            slr_type = "MSF";
                            final_req_dirty = dirty_price;
                            final_req_mv = data.1.out_market_value;
                        }
                    } else if excess_slr_gsec <= final_req {
                        let req_proportion = excess_slr_gsec / final_req;

                        final_rem = final_req - excess_slr_gsec;
                        msf -= final_rem;
                        final_req = excess_slr_gsec;
                        let rem_proportion = final_rem / data.1.out_book_value;
                        chnmsf_flag = true;
                        slr_type = "CHNMSF";
                        final_req_dirty = dirty_price * req_proportion;
                        final_rem_dirty = dirty_price * rem_proportion;
                        final_req_mv = data.1.out_market_value * req_proportion;
                        final_rem_mv = data.1.out_market_value * rem_proportion;
                    } else {
                        excess_slr_gsec -= final_req;
                        slr_type = "SLR";
                        final_req_dirty = dirty_price;
                        final_req_mv = data.1.out_market_value;
                    }

                    row_no += 1;

                    writeln!(
                                    output_writer,
                                    "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{:.4}|{}|{}|{}|{}|{}|{}|{}|{}",
                                    as_on_dt.format(config_params.output_date_format()),
                                    config_params.currency().to_owned(),
                                    row_no,
                                    data.0.isin_no,
                                    data.0.book_category,
                                    category,
                                    data.1.actual_face_value,
                                    data.1.actual_book_value,
                                    data.1.actual_market_value,
                                    data.1.pledged_face_value,
                                    data.1.pledged_book_value,
                                    data.1.pledged_market_value,
                                    data.1.out_face_value,
                                    data.1.out_book_value,
                                    data.1.out_market_value,
                                    data.1.maturity_date.format("%d-%m-%Y"),
                                    data.1.coupon,
                                    accrued_days,
                                    accrued_interest,
                                    dirty_price,
                                    data.1.appr_depr,
                                    slr_type,
                                    final_req,
                                    final_rem,
                                    final_req_dirty,
                                    final_rem_dirty,
                                    final_req_mv,
                                    final_rem_mv,
                                    data.1.repo_mat_date.format("%d-%m-%Y"),
                                ).expect("Could not write to output file.")
                }
            }
            None => {
                continue;
            }
        }
    }
    for data in other_category_vec.into_iter() {
        let mut final_rem = 0.0;
        let mut final_rem_dirty = 0.0;
        let mut final_rem_mv = 0.0;
        let convention = config_params.accrued_day_convention();
        let accrued_days_conv =
            accrued_days_with_convn(*as_on_dt, data.1.maturity_date, 6, convention).unwrap();
        let accrued_days = accrued_days_conv.days_btw_dts;
        let accrued_interest = (accrued_days as f64 * data.1.coupon * data.1.pledged_face_value)
            / (accrued_days_conv.day_in_yr * 100) as f64;
        let dirty_price = data.1.out_market_value + accrued_interest;
        final_req = data.1.out_book_value;

        if chnmsf_flag && chnfallcr_flag && chnnone_flag {
            final_req = 0.0;
            final_req_dirty = 0.0;
            final_req_mv = 0.0;
            slr_type = "NONE";
        } else if chnmsf_flag && chnfallcr_flag {
            if fallcr <= final_req {
                let req_proportion = fallcr / final_req;
                final_rem = final_req - fallcr;
                final_req = fallcr;
                let rem_proportion = final_rem / data.1.out_book_value;
                chnnone_flag = true;
                slr_type = "CHNNONE";
                final_req_dirty = dirty_price * req_proportion;
                final_rem_dirty = dirty_price * rem_proportion;
                final_req_mv = data.1.out_market_value * req_proportion;
                final_rem_mv = data.1.out_market_value * rem_proportion;
            } else {
                fallcr -= final_req;
                slr_type = "FALLCR";
                final_req_dirty = dirty_price;
                final_req_mv = data.1.out_market_value;
            }
        } else if chnmsf_flag {
            if msf <= final_req {
                let req_proportion = msf / final_req;
                final_rem = final_req - msf;
                fallcr -= final_rem;
                final_req = msf;
                let rem_proportion = final_rem / data.1.out_book_value;

                chnfallcr_flag = true;
                slr_type = "CHNFALLCR";
                final_req_dirty = dirty_price * req_proportion;
                final_rem_dirty = dirty_price * rem_proportion;
                final_req_mv = data.1.out_market_value * req_proportion;
                final_rem_mv = data.1.out_market_value * rem_proportion;
            } else {
                msf -= final_req;
                slr_type = "MSF";
                final_req_dirty = dirty_price;
                final_req_mv = data.1.out_market_value;
            }
        } else if excess_slr_gsec <= final_req {
            let req_proportion = excess_slr_gsec / final_req;

            final_rem = final_req - excess_slr_gsec;
            msf -= final_rem;
            final_req = excess_slr_gsec;
            let rem_proportion = final_rem / data.1.out_book_value;
            chnmsf_flag = true;
            slr_type = "CHNMSF";
            final_req_dirty = dirty_price * req_proportion;
            final_rem_dirty = dirty_price * rem_proportion;
            final_req_mv = data.1.out_market_value * req_proportion;
            final_rem_mv = data.1.out_market_value * rem_proportion;
        } else {
            excess_slr_gsec -= final_req;
            slr_type = "SLR";
            final_req_dirty = dirty_price;
            final_req_mv = data.1.out_market_value;
        }

        row_no += 1;

        writeln!(
                        output_writer,
                        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{:.4}|{}|{}|{}|{}|{}|{}|{}|{}",
                        as_on_dt.format(config_params.output_date_format()),
                        config_params.currency().to_owned(),
                        row_no,
                        data.0.isin_no,
                        data.0.book_category,
                        data.0.book_category,
                        data.1.actual_face_value,
                        data.1.actual_book_value,
                        data.1.actual_market_value,
                        data.1.pledged_face_value,
                        data.1.pledged_book_value,
                        data.1.pledged_market_value,
                        data.1.out_face_value,
                        data.1.out_book_value,
                        data.1.out_market_value,
                        data.1.maturity_date.format("%d-%m-%Y"),
                        data.1.coupon,
                        accrued_days,
                        accrued_interest,
                        dirty_price,
                        data.1.appr_depr,
                        slr_type,
                        final_req,
                        final_rem,
                        final_req_dirty,
                        final_rem_dirty,
                        final_req_mv,
                        final_rem_mv,
                        data.1.repo_mat_date.format("%d-%m-%Y"),
                    ).expect("Could not write to output file.")
    }
}
