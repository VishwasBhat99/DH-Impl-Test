use sdb_io;
use serde_json;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccFieldNames {
    pub cod_acc_no: String,
    pub cod_cc_brn: String,
    pub cod_prod: String,
    pub bal_book: String,
    pub bal_book_lcy: String,
    pub amt_od_lmt: String,
    pub amt_od_lmt_lcy: String,
    pub cod_cust: String,
    pub cod_acc_title: String,
    pub dt_open_acc: String,
    pub cod_int_accr_bas: String,
    pub freq_int_accr: String,
    pub dt_acc_close: String,
    pub cod_collat_id: String,
    pub collat_desc: String,
    pub as_of_dt: String,
    pub cost_cntr: String,
    pub gl_acc_no: String,
    pub rt_flg: String,
    pub inst: String,
    pub crnt_book_bal: String,
    pub acrl_basis: String,
    pub int_rt: String,
    pub div: String,
    pub alm_line: String,
    pub ia_llg: String,
    pub balm_llg: String,
    pub mis1: String,
    pub npa_flg: String,
    pub benchmark: String,
    pub rep_freq: String,
    pub nxt_rep_dt: String,
    pub lst_rep_dt: String,
    pub cust_typ: String,
    pub country: String,
    pub bm_id_lookup: String,
    pub mis2: String,
    pub avg_bal: String,
    pub alm_concat: String,
    pub two_point_concat: String,
    pub ews_flag: String,
    pub bdp_division: String,
    pub bdp_coa: String,
}

impl AccFieldNames {
    pub fn new_from_path(_path: &str) -> AccFieldNames {
        let mut file = sdb_io::open_file_read(_path).expect("Unable to open Required-Fields-File");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Cannot read input as string from Req-Fields File");
        let req_fields: AccFieldNames =
            serde_json::from_str(&buf[..]).expect("Required-Fields-File was not well-formatted");
        req_fields
    }

    pub fn get_input_fields_names(_path: &str) -> AccFieldNames {
        let req_fields = Self::new_from_path(_path);
        AccFieldNames {
            cod_acc_no: req_fields.cod_acc_no.to_string(),
            cod_cc_brn: req_fields.cod_cc_brn.to_string(),
            cod_prod: req_fields.cod_prod.to_string(),
            bal_book: req_fields.bal_book.to_string(),
            bal_book_lcy: req_fields.bal_book_lcy.to_string(),
            amt_od_lmt: req_fields.amt_od_lmt.to_string(),
            amt_od_lmt_lcy: req_fields.amt_od_lmt_lcy.to_string(),
            cod_cust: req_fields.cod_cust.to_string(),
            cod_acc_title: req_fields.cod_acc_title.to_string(),
            dt_open_acc: req_fields.dt_open_acc.to_string(),
            cod_int_accr_bas: req_fields.cod_int_accr_bas.to_string(),
            freq_int_accr: req_fields.freq_int_accr.to_string(),
            dt_acc_close: req_fields.dt_acc_close.to_string(),
            cod_collat_id: req_fields.cod_collat_id.to_string(),
            collat_desc: req_fields.collat_desc.to_string(),
            as_of_dt: req_fields.as_of_dt.to_string(),
            cost_cntr: req_fields.cost_cntr.to_string(),
            gl_acc_no: req_fields.gl_acc_no.to_string(),
            rt_flg: req_fields.rt_flg.to_string(),
            inst: req_fields.inst.to_string(),
            crnt_book_bal: req_fields.crnt_book_bal.to_string(),
            acrl_basis: req_fields.acrl_basis.to_string(),
            int_rt: req_fields.int_rt.to_string(),
            div: req_fields.div.to_string(),
            alm_line: req_fields.alm_line.to_string(),
            ia_llg: req_fields.ia_llg.to_string(),
            balm_llg: req_fields.balm_llg.to_string(),
            mis1: req_fields.mis1.to_string(),
            npa_flg: req_fields.npa_flg.to_string(),
            benchmark: req_fields.benchmark.to_string(),
            rep_freq: req_fields.rep_freq.to_string(),
            nxt_rep_dt: req_fields.nxt_rep_dt.to_string(),
            lst_rep_dt: req_fields.lst_rep_dt.to_string(),
            cust_typ: req_fields.cust_typ.to_string(),
            country: req_fields.country.to_string(),
            bm_id_lookup: req_fields.bm_id_lookup.to_string(),
            mis2: req_fields.mis2.to_string(),
            avg_bal: req_fields.avg_bal.to_string(),
            alm_concat: req_fields.alm_concat.to_string(),
            two_point_concat: req_fields.two_point_concat.to_string(),
            ews_flag: req_fields.ews_flag.to_string(),
            bdp_division: req_fields.bdp_division.to_string(),
            bdp_coa: req_fields.bdp_coa.to_string(),
        }
    }
}
