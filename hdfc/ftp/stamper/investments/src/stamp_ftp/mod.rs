use sdb_dyn_proto_rdr::reader::account_with_cfs::AccountWithCFs;
use stamp_ftp::CFout::AccountWithCashflows;

pub mod CFout;
pub mod amb_file_reader;
pub mod append_output;
pub mod bm_reader;
pub mod calc_ftp;
pub mod cfinput;
pub mod ftp_rates_reader;
pub mod io;
pub mod one_acc_view;
pub mod read_adjustments;
pub mod rule_stamper;

use math::round::half_away_from_zero;
use statics::*;

pub fn read_cashflow(
    cfin: &AccountWithCFs,
    input_field_names: &cfinput::AccFieldNames,
    rate_precision: i8,
    bal_precision: i8,
) -> AccountWithCashflows {
    let mut cfoutput = AccountWithCashflows::new();

    cfoutput.deal_no = match cfin.get_string_for_key(&input_field_names.deal_no) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.inst = match cfin.get_string_for_key(&input_field_names.inst) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.book_yield = half_away_from_zero(
        match cfin.get_f64_for_key(&input_field_names.book_yield) {
            Ok(result) => result,
            Err(_) => DEFAULT_FLOAT,
        },
        rate_precision,
    );

    cfoutput.val_dt = match cfin.get_i64_for_key(&input_field_names.val_dt) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };

    cfoutput.mat_dt = match cfin.get_i64_for_key(&input_field_names.mat_dt) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };

    cfoutput.put_dt = match cfin.get_i64_for_key(&input_field_names.put_dt) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };

    cfoutput.lst_rep_dt = match cfin.get_i64_for_key(&input_field_names.lst_rep_dt) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };

    cfoutput.nxt_rep_dt = match cfin.get_i64_for_key(&input_field_names.nxt_rep_dt) {
        Ok(result) => result,
        Err(_) => cfoutput.mat_dt,
    };

    cfoutput.ftp_mat_dt = match cfin.get_i64_for_key(&input_field_names.ftp_mat_dt) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };

    cfoutput.call_dt = match cfin.get_i64_for_key(&input_field_names.call_dt) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };

    cfoutput.rep_tenor = match cfin.get_i64_for_key(&input_field_names.rep_tenor) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };

    cfoutput.gl = match cfin.get_i64_for_key(&input_field_names.gl) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };

    cfoutput.rt_flg = match cfin.get_string_for_key(&input_field_names.rt_flg) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.prod_cd = match cfin.get_string_for_key(&input_field_names.prod_cd) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.alm_line = match cfin.get_string_for_key(&input_field_names.alm_line) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.os_cv_after_amort = half_away_from_zero(
        match cfin.get_f64_for_key(&input_field_names.os_cv_after_amort) {
            Ok(result) => result,
            Err(_) => DEFAULT_FLOAT,
        },
        bal_precision,
    );

    cfoutput.os_cv_before_amort = half_away_from_zero(
        match cfin.get_f64_for_key(&input_field_names.os_cv_before_amort) {
            Ok(result) => result,
            Err(_) => DEFAULT_FLOAT,
        },
        bal_precision,
    );

    cfoutput.int_rt = half_away_from_zero(
        match cfin.get_f64_for_key(&input_field_names.int_rt) {
            Ok(result) => result,
            Err(_) => DEFAULT_FLOAT,
        },
        rate_precision,
    );

    cfoutput.deal_dt = match cfin.get_i64_for_key(&input_field_names.deal_dt) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };

    cfoutput.val_dt = match cfin.get_i64_for_key(&input_field_names.val_dt) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };

    cfoutput.os_face_val = match cfin.get_i64_for_key(&input_field_names.os_face_val) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };

    cfoutput.entity = match cfin.get_string_for_key(&input_field_names.entity) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.short_name = match cfin.get_string_for_key(&input_field_names.short_name) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.isin = match cfin.get_string_for_key(&input_field_names.isin) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.prod_desc = match cfin.get_string_for_key(&input_field_names.prod_desc) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.portfolio = match cfin.get_string_for_key(&input_field_names.portfolio) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.desk = match cfin.get_string_for_key(&input_field_names.desk) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.port_typ = match cfin.get_string_for_key(&input_field_names.port_typ) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.acc_sec_igaap = match cfin.get_string_for_key(&input_field_names.acc_sec_igaap) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.deal_rt = match cfin.get_f64_for_key(&input_field_names.deal_rt) {
        Ok(result) => result,
        Err(_) => DEFAULT_FLOAT,
    };

    cfoutput.contract_no = match cfin.get_string_for_key(&input_field_names.contract_no) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.instr_id = match cfin.get_string_for_key(&input_field_names.instr_id) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();
    cfoutput.parent_code = match cfin.get_string_for_key(&input_field_names.parent_code) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.issuer_name = match cfin.get_string_for_key(&input_field_names.issuer_name) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.intr_typ = match cfin.get_string_for_key(&input_field_names.intr_typ) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.sec_issuance_date = match cfin.get_i64_for_key(&input_field_names.sec_issuance_date) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };

    cfoutput.coupon = match cfin.get_string_for_key(&input_field_names.coupon) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.last_intr_dt = match cfin.get_i64_for_key(&input_field_names.last_intr_date) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };
    cfoutput.next_intr_dt = match cfin.get_i64_for_key(&input_field_names.next_intr_date) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };
    cfoutput.nxt_rep_dt = match cfin.get_i64_for_key(&input_field_names.nxt_rep_dt) {
        Ok(result) => result,
        Err(_) => DEFAULT_INT,
    };

    cfoutput.rating = match cfin.get_string_for_key(&input_field_names.rating) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.tax_status = match cfin.get_string_for_key(&input_field_names.tax_status) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.slr_nslr = match cfin.get_string_for_key(&input_field_names.slr_nslr) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.deal_ytm = match cfin.get_f64_for_key(&input_field_names.deal_ytm) {
        Ok(result) => result,
        Err(_) => DEFAULT_FLOAT,
    };

    cfoutput.amort_till_dt = match cfin.get_f64_for_key(&input_field_names.amort_till_date) {
        Ok(result) => result,
        Err(_) => DEFAULT_FLOAT,
    };

    cfoutput.intr_app_freq = match cfin.get_string_for_key(&input_field_names.intr_app_freq) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.comp_freq = match cfin.get_string_for_key(&input_field_names.comp_freq) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.intr_prac = match cfin.get_string_for_key(&input_field_names.intr_prac) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.rate_spread = match cfin.get_string_for_key(&input_field_names.rate_spread) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput.asset_class = match cfin.get_string_for_key(&input_field_names.asset_class) {
        Ok(result) => result,
        Err(_) => "",
    }
    .to_string();

    cfoutput
}
