use super::NaiveDate;

pub fn get_default_ftp_flag(
    org_dt: Option<NaiveDate>,
    mat_dt: Option<NaiveDate>,
    lst_rep_dt: Option<NaiveDate>,
    nxt_rep_dt: Option<NaiveDate>,
    rate_flag: &str,
) -> String {
    let mut def_ftp_flag = String::from("N");
    if let Some(o_dt) = org_dt {
        if let Some(l_r_dt) = lst_rep_dt {
            if l_r_dt < o_dt {
                if rate_flag == "V" {
                    def_ftp_flag = String::from("N");
                } else {
                    def_ftp_flag = String::from("Y");
                }
            }
        }
    }

    if let Some(m_dt) = mat_dt {
        if let Some(n_r_dt) = nxt_rep_dt {
            if n_r_dt > m_dt && n_r_dt != NaiveDate::from_ymd(2099, 12, 31) {
                if rate_flag == "V" {
                    def_ftp_flag = String::from("N");
                } else {
                    def_ftp_flag = String::from("Y");
                }
            }
        }
    }

    if let Some(o_dt) = org_dt {
        if let Some(l_r_dt) = lst_rep_dt {
            if let Some(n_r_dt) = nxt_rep_dt {
                if n_r_dt < l_r_dt && n_r_dt < o_dt && n_r_dt != NaiveDate::from_ymd(2099, 12, 31) {
                    if rate_flag == "V" {
                        def_ftp_flag = String::from("N");
                    } else {
                        def_ftp_flag = String::from("Y");
                    }
                }
            }
        }
    }

    if let Some(o_dt) = org_dt {
        if let Some(m_dt) = mat_dt {
            if m_dt < o_dt {
                def_ftp_flag = String::from("Y");
            }
        }
    }

    def_ftp_flag
}
