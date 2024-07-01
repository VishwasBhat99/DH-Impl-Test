use super::remove_comma;
use super::InputAccount;
use macros;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_op_line(account: &InputAccount, as_on_dt: NaiveDate, log: &Logger) -> String {
    let dt = as_on_dt.format("%d-%m-%Y");
    get_line(account, &dt.to_string(), log)
}

fn get_line(account: &InputAccount, dt: &str, log: &Logger) -> String {
    let mut op_line = String::new();
    op_line.push_str(&remove_comma(&account.deal_no));
    op_line.push('|');

    let acc_open_dt = match NaiveDate::parse_from_str(&account.value_dt_1st_leg, "%d %b %Y") {
        Ok(dt) => dt.format("%d-%m-%Y").to_string(),
        Err(error) => {
            log_error!(
                log,
                "value_date_1st_leg: `{}` not well-formatted for account: `{}` as 'DD MMM YYYY' : {}.",
                account.value_dt_1st_leg,
                account.deal_no,
                error,
            );
            String::new()
        }
    };
    op_line.push_str(&acc_open_dt);
    op_line.push('|');

    op_line.push_str(&account.nature);
    op_line.push('|');
    op_line.push_str(&account.deal_stat);
    op_line.push('|');
    op_line.push_str(&account.deal_type);
    op_line.push('|');
    op_line.push_str(&account.slr_typ);
    op_line.push('|');
    op_line.push_str(&account.security);
    op_line.push('|');
    op_line.push_str(&account.category);
    op_line.push('|');
    op_line.push_str(&account.sub_category);
    op_line.push('|');
    op_line.push_str(&account.desk);
    op_line.push('|');
    op_line.push_str(&account.portfolio);
    op_line.push('|');
    op_line.push_str(&account.accounting_section);
    op_line.push('|');
    op_line.push_str(&account.counterparty);
    op_line.push('|');
    op_line.push_str(&account.counterparty_full_name);
    op_line.push('|');
    op_line.push_str(&account.currency);
    op_line.push('|');
    op_line.push_str(&remove_comma(&account.repo_rate));
    op_line.push('|');
    op_line.push_str(&remove_comma(&account.ytm));
    op_line.push('|');

    let val_dt = match NaiveDate::parse_from_str(&account.value_dt, "%d %b %Y") {
        Ok(dt) => dt.format("%d-%m-%Y").to_string(),
        Err(error) => {
            log_error!(
                log,
                "value_date: `{}` not well-formatted for account: `{}` as 'DD MMM YYYY' : {}.",
                account.value_dt,
                account.deal_no,
                error,
            );
            String::new()
        }
    };
    op_line.push_str(&val_dt);
    op_line.push('|');

    op_line.push_str(&remove_comma(&account.price));
    op_line.push('|');
    op_line.push_str(&remove_comma(&account.settle_amt_1st_leg));
    op_line.push('|');
    op_line.push_str(&remove_comma(&account.accrued_interest));
    op_line.push('|');
    op_line.push_str(&remove_comma(&account.repo_interest));
    op_line.push('|');
    op_line.push_str(&remove_comma(&account.settle_amt_2nd_leg));
    op_line.push('|');
    op_line.push_str(&account.entity);
    op_line.push('|');
    op_line.push_str(&account.bank_non_bank);
    op_line.push('|');
    op_line.push_str(&remove_comma(&account.air_aip));
    op_line.push('|');
    op_line.push_str(&dt);

    op_line.push('\n');

    op_line
}
