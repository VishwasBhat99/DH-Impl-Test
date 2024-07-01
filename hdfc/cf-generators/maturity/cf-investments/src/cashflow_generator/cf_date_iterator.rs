use cashflow_generator::account_reader::input_account::InputAccount;
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
    pub fn new_from_account(
        account: &InputAccount,
        as_on_date: &NaiveDate,
        maturity_date: &NaiveDate,
    ) -> CFDateIterator {
        let st_dt = if let Some(dt) = account.deal_dt {
            dt
        } else {
            *as_on_date
        };
        CFDateIterator::new(account.int_pay_freq, &st_dt, &maturity_date, as_on_date)
    }

    pub fn new(
        frequency: i64,
        start_date: &NaiveDate,
        mat_date: &NaiveDate,
        as_on_date: &NaiveDate,
    ) -> CFDateIterator {
        CFDateIterator {
            frequency: frequency as u16,
            start_date: start_date.clone(),
            mat_date: mat_date.clone(),
            as_on_date: as_on_date.to_owned(),
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
                        let new_date = rbdate::increment_date_by_months_unchecked(
                            self.start_date,
                            (self.current_cf_number * self.frequency) as u16,
                        );
                        if new_date >= as_on_date {
                            cf_date = new_date;
                            break;
                        }
                    }

                    cf_date
                };

                cf_date
            } else {
                self.current_cf_number += 1;
                let cf_date = rbdate::increment_date_by_months_unchecked(
                    self.start_date,
                    (self.current_cf_number * self.frequency) as u16,
                );

                cf_date
            }
        };

        let next_cf_date = {
            if next_possible_cf_date >= self.mat_date {
                self.has_account_matured = true;

                self.mat_date
            } else {
                next_possible_cf_date
            }
        };

        return Some(next_cf_date);
    }
}
