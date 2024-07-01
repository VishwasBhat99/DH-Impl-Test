use cashflow_generator::account_reader::input_account::InputAccount;
use cashflow_generator::account_with_cashflows::AccountWithCashflows;
use cashflow_generator::account_with_cashflows::Cashflow;
use protobuf;
use rbdate;

pub fn create_account_with_cashflows(
    account: InputAccount,
    cashflows: Vec<Cashflow>,
) -> AccountWithCashflows {
    let mut account_with_cashflows = AccountWithCashflows::new();
    let mut total_interest_amount = 0.0;
    let mut total_principal_amount = 0.0;

    account_with_cashflows.account_number = account.account_number;
    account_with_cashflows.accrued_interest = account.accrued_interest;
    account_with_cashflows.deposit_type = account.deposit_type;
    account_with_cashflows.maturity_date = rbdate::timestamp(account.maturity_date);
    account_with_cashflows.rat_acct_int = account.rat_acct_int;
    account_with_cashflows.rat_acct_int_var = account.rat_acct_int_var;
    account_with_cashflows.next_compound_date = {
        if account.next_compound_date.is_some() {
            rbdate::timestamp(account.next_compound_date.unwrap())
        } else {
            0
        }
    };
    account_with_cashflows.next_payment_date = {
        if account.next_payment_date.is_some() {
            rbdate::timestamp(account.next_payment_date.unwrap())
        } else {
            0
        }
    };
    account_with_cashflows.account_start_date = rbdate::timestamp(account.account_start_date);
    account_with_cashflows.currency_code = account.currency_code;
    account_with_cashflows.customer_id = account.customer_id;
    account_with_cashflows.original_balance = account.original_balance;
    account_with_cashflows.origination_date = {
        if account.origination_date.is_some() {
            rbdate::timestamp(account.origination_date.unwrap())
        } else {
            0
        }
    };
    account_with_cashflows.previous_roll_over_date = {
        if account.previous_roll_over_date.is_some() {
            rbdate::timestamp(account.previous_roll_over_date.unwrap())
        } else {
            0
        }
    };
    account_with_cashflows.description = account.description;
    account_with_cashflows.client_name = account.client_name;
    account_with_cashflows.tname = account.tname;
    account_with_cashflows.as_on_date = account.as_on_date;
    account_with_cashflows.bank_num = account.bank_num;
    account_with_cashflows.branch = account.branch;
    account_with_cashflows.rate_flag = account.rate_flag;
    account_with_cashflows.cost_centre_ftp = account.cost_centre_ftp;
    account_with_cashflows.int_pay_freq = account.int_pay_freq;
    account_with_cashflows.institution = account.institution;
    account_with_cashflows.new_gl = account.new_gl;
    account_with_cashflows.int_rate = account.int_rate;
    account_with_cashflows.concat = account.concat;
    account_with_cashflows.ia_llg = account.ia_llg;
    account_with_cashflows.balm_llg = account.balm_llg;
    account_with_cashflows.current_book_balance = account.current_book_balance;
    account_with_cashflows.cost_center = account.cost_center;
    account_with_cashflows.comp_freq = account.comp_freq;
    account_with_cashflows.fin_cost_ftp = account.fin_cost_ftp;
    account_with_cashflows.cust_category = account.cust_category;

    account_with_cashflows.cod_prod = account.cod_prod;
    account_with_cashflows.com_mis_comp_1 = account.com_mis_comp_1;
    account_with_cashflows.rat_prod_var = account.rat_prod_var;
    account_with_cashflows.dat_value_date = {
        if account.dat_value_date.is_some() {
            rbdate::timestamp(account.dat_value_date.unwrap())
        } else {
            0
        }
    };
    account_with_cashflows.alm_concat = account.alm_concat;
    account_with_cashflows.amt_initl_dep = account.amt_initl_dep;
    account_with_cashflows.bal_principle_lcy = account.bal_principle_lcy;
    account_with_cashflows.bal_int_accr_lcy = account.bal_int_accr_lcy;
    account_with_cashflows.lien_marked = account.lien_marked;
    account_with_cashflows.prod_code = account.prod_code;
    account_with_cashflows.acc_open_date = {
        if account.acc_open_date.is_some() {
            rbdate::timestamp(account.acc_open_date.unwrap())
        } else {
            0
        }
    };
    account_with_cashflows.gl_int_comp = account.gl_int_comp;
    account_with_cashflows.division = account.division;
    account_with_cashflows.lcr_class_code = account.lcr_class_code;
    account_with_cashflows.contr_mat = account.contr_mat;
    account_with_cashflows.res_mat = account.res_mat;
    account_with_cashflows.bal_int_comp_lcy = account.bal_int_comp_lcy;

    for cf in &cashflows {
        total_interest_amount += cf.interest_amount;
        total_principal_amount += cf.principal_amount;
    }
    account_with_cashflows.total_principal_amount = total_principal_amount;
    account_with_cashflows.total_interest_amount = total_interest_amount;
    account_with_cashflows.cashflows = protobuf::RepeatedField::from_vec(cashflows);

    account_with_cashflows
}
