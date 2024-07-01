use cashflow_generator::account_reader::input_account::InputAccount;
use rbdate;
use rbdate::NaiveDate;

/// A CFDateIterator gives you an iterator over ever date on which a cashflow will occur.
pub struct CFDateIterator {
    frequency: u16,
    start_date: NaiveDate,
    mat_date: NaiveDate,
    as_on_date: NaiveDate,
    current_cf_number: u16,
    is_as_on_date_handled: bool,
    has_account_matured: bool,
}

impl CFDateIterator {
    pub fn new_from_account(account: &InputAccount, as_on_date: NaiveDate) -> CFDateIterator {
        let start_date = account.acct_open_dt;
        let mat_dt = account.mat_dt;
        CFDateIterator::new(
            account.int_repay_freq,
            &start_date.unwrap(),
            &mat_dt.unwrap(),
            as_on_date,
        )
    }

    pub fn new(
        frequency: i64,
        start_date: &NaiveDate,
        mat_date: &NaiveDate,
        as_on_date: NaiveDate,
    ) -> CFDateIterator {
        CFDateIterator {
            frequency: frequency as u16,
            start_date: start_date.clone(),
            mat_date: mat_date.clone(),
            as_on_date: as_on_date,
            current_cf_number: 0,
            is_as_on_date_handled: false,
            has_account_matured: false,
        }
    }
}

impl Iterator for CFDateIterator {
    type Item = NaiveDate;

    // TODO: Maybe we can avoid so many if-else branches?
    fn next(&mut self) -> Option<NaiveDate> {
        if self.has_account_matured {
            return None;
        }

        let next_possible_cf_date = {
            if self.is_as_on_date_handled == false {
                self.is_as_on_date_handled = true;

                let cf_date = {
                    let as_on_date = self.as_on_date;
                    let cf_date;
                    loop {
                        self.current_cf_number += 1;
                        let new_date = rbdate::incr_dt_by_mon_presrv_eom(
                            self.start_date,
                            (self.current_cf_number * self.frequency) as usize,
                        )
                        .unwrap();
                        if new_date > as_on_date {
                            cf_date = new_date;
                            break;
                        }
                    }

                    cf_date
                };

                cf_date
            } else {
                self.current_cf_number += 1;
                let cf_date = rbdate::incr_dt_by_mon_presrv_eom(
                    self.start_date,
                    (self.current_cf_number * self.frequency) as usize,
                )
                .unwrap();

                cf_date
            }
        };

        let next_cf_date = {
            if next_possible_cf_date >= self.mat_date {
                self.has_account_matured = true;
                Some(self.mat_date)
            } else {
                Some(next_possible_cf_date)
            }
        };

        next_cf_date
    }
}
