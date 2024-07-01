use cashflow_generator::account_reader::input_account::InputAccount;
use rbdate;
use rbdate::NaiveDate;

/// A CFDateIterator gives you an iterator over every date on which a cashflow will occur.
pub struct CFDateIterator {
    frequency: u16,
    start_date: NaiveDate,
    mat_date: NaiveDate,
    current_cf_number: u16,
    has_account_matured: bool,
}

impl CFDateIterator {
    pub fn new_from_account(account: &InputAccount) -> CFDateIterator {
        let start_date = account
            .next_payment_date
            .expect("Cannot parse Next Payemnt Date");
        let maturity_date = account
            .next_repr_date
            .expect("Cannot parse Next Reprice Date")
            .min(account.maturity_date.expect("Cannot parse maturity date"));
        CFDateIterator::new(account.int_pay_freq, &start_date, &maturity_date)
    }

    pub fn new(frequency: i64, start_date: &NaiveDate, mat_date: &NaiveDate) -> CFDateIterator {
        CFDateIterator {
            frequency: frequency as u16,
            start_date: start_date.clone(),
            mat_date: mat_date.clone(),
            current_cf_number: 0,
            has_account_matured: false,
        }
    }
}

impl Iterator for CFDateIterator {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<NaiveDate> {
        if self.has_account_matured {
            return None;
        }

        self.current_cf_number += 1;
        let next_possible_cf_date = rbdate::incr_dt_by_mon_presrv_eom(
            self.start_date,
            (self.current_cf_number * self.frequency) as usize,
        )
        .unwrap();

        let next_cf_date = if !self.has_account_matured && next_possible_cf_date >= self.mat_date {
            self.has_account_matured = true;
            Some(self.mat_date)
        } else {
            Some(next_possible_cf_date)
        };

        next_cf_date
    }
}
