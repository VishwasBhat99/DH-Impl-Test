# Deposits

Deposits processes accounts as input and writes the account's data as-is in the output, accompanied by the cashflows the account generates. This document outlines the structure of the input, the output, and how the cashflows are calculated per account.
## Input
The input structure looks like this:

| Start point | Length | Field Name           | Field Type   | Description            | Required | Usage              |
|-------------|--------|----------------------|--------------|------------------------|----------|--------------------|
| 1           | 14     | account_number       | Text         | Account Number         | Y        | Account number     |
| 40          | 16     | bal_int_accr_lcy     | Decimal      | Accrued Interest       |          |                    |
| 60          | 3      | cod_prod             | Text         | Deposit Type           |          |                    |
| 79          | 15     | current_book_balance | Decimal      | Current Book Balance   | Y        | Oustanding balance |
| 95          | 11     | dat_maturity         | DDMONYYYY    | Maturity Date          | Y        | Maturity Date      |
| 129         | 7      | rat_acct_int         | Decimal      | $Rate_Int              |          |                    |
| 136         | 7      | rat_acct_int_var     | Decimal      | $Rate_Int_Var          |          |                    |
| 143         | 11     | dat_next_int_comp    | DDMONYYYY    | Next Compound Date     |          |                    |
| 154         | 11     | dat_next_int_pay     | DDMONYYYY    | Next Payment Date      |          |                    |
| 165         | 11     | account_start_date   | DDMONYYYY    | Account start date     | Y        | Start date         |
| 176         | 3      | currency_code        | Whole Number | Currency Code          |          |                    |
| 179         | 10     | cod_cust             | Whole Number | CUSIP Number           | Y        | Customer ID        |
| 205         | 15     | original_balance     | Decimal      | Original Balance       |          |                    |
| 243         | 11     | origination_date     | DDMONYYYY    | Origination Date       |          |                    |
| 243         | 11     | dat_value_date       | DDMONYYYY    | Previous Rollover Date |          |                    |
| 287         | 35     | nam_product          | Text         | Description            |          |                    |
| 328         | 9      | gl_liab              | Whole Number | $GLSL System           |          |                    |
| 339         | 35     | client_name          | Text         | Client Name            | Y        | Customer Name      |
| 390         | 4      | t_name               | Text         | $TName                 |          |                    |
| 396         | 10     | as_of_date           | YYYYMMDD     | As Of Date             |          |                    |
| 407         | 4      | bank_number          | Text         | Bank Number            |          |                    |
| 412         | 3      | branch               | Text         | Branch                 |          |                    |
| 416         | 3      | cost_centre_ftp      | Text         | Cost Center            |          |                    |
| 420         | 10     | new_gl_sl            | Whole Number | GL Account Number      |          |                    |
| 431         | 5      | rat_int_total        | Decimal      | Interest Rate          | Y        | Interest rate      |
| 444         | 1      | rate_flag            | Text         | Rate Flag              |          |                    |
| 446         | 1      | frq_int_pay          | Whole Number | Interest Payment Flag  | Y        | Frequency          |
| 448         | 3      | institution          | Whole Number | Institution            | Y        | Currency ID        |
| 752         | 25     | concat               | Text         | $CONCAT                | Y        | ALM_LINE           |


## Output
The output structure is:

| Field Name           | Field Type    | How to Compute                                 |
|----------------------|---------------|------------------------------------------------|
| account_number       | String        | As-is, from input                              |
| bal_int_accr_lcy     | f64           | As-is, from input                              |
| cod_prod             | String        | As-is, from input                              |
| current_book_balance | f64           | As-is, from input                              |
| dat_maturity         | i64           | Unix timestamp of input date                   |
| rat_acct_int         | f64           | As-is, from input                              |
| rat_acct_int_var     | f64           | As-is, from input                              |
| dat_next_int_comp    | i64           | Unix timestamp of input date                   |
| dat_next_int_pay     | i64           | Unix timestamp of input date                   |
| account_start_date   | i64           | Unix timestamp of input date                   |
| currency_code        | i64           | As-is, from input                              |
| cod_cust             | i64           | As-is, from input                              |
| original_balance     | f64           | A-is, from input                               |
| origination_date     | i64           | Unix timestamp of input date                   |
| dat_value_date       | i64           | Unix timestamp of input date                   |
| nam_product          | String        | As-is, from input                              |
| gl_liab              | i64           | As-is, from input                              |
| client_name          | String        | As-is, from input                              |
| t_name               | String        | As-is, from input                              |
| as_of_date           | f64           | Unix timestamp of input date                   |
| bank_number          | String        | A-is, from input                               |
| branch               | String        | As-is, from input                              |
| cost_centre_ftp      | String        | As-is, from input                              |
| new_gl_sl            | i64           | As-is, from input                              |
| rat_int_total        | f64           | As-is, from input                              |
| rate_flag            | String        | As-is, from input                              |
| frq_int_pay          | i64           | As-is, from input                              |
| institution          | i64           | As-is, from input                              |
| concat               | String        | As-is, from input                              |
| cashflows            | Vec<Cashflow> | Defined in "Calculating Cashflows"             |

Here's the struct for a Cashflow:
```
struct Cashflow {
    interest_amount: f64,
    principal_amount: f64,
    date: i64
}
```


## Calculating Cashflows

Deposits cares about Interest Amount cashflows. Each cashflow generated by Deposits *except the last*, for an Account, will have the `principal_amount` set to `0.0`. The last cashflow will have the `principal_amount` set as the `outstanding_amount` of the account.

### Calculating Interest Amounts for a Cashflow
We use the following formula for calculating the interest amount for a cashflow:

`interest_amount = (outstanding_amount * interest_rate * number_of_days) / 36500.0`

**If the `number_of_days` is 0, change it to 1**.

*If there is no `as_on_date` given to the program*, the number of days calculated by a difference of `cashflow_date - previous_date`. For the first cashflow, the `start_date` is the `previous_date`. For all following cashflows the previous `cashflow_date` is the `previous_date`.

*In cases when there is an `as_on_date` given to the program*, only the first cashflow's `number_of_days` **may** be affected. See Examples 4, and 5 for details on this case.

### Determining the dates of Cashflows
Understanding how to calculate cashflows demands that you understand how to compute the dates on which cashflows occur.

#### Fields of Importance
- Along with the input fields provided to us, the program may optionally be given an `as_on_date`.
- `account_start_date` is the account's `start_date`
- `dat_maturity` is the account's `maturity_date`
- `frq_int_pay` is the account's `payment_frequency`. This field can have the values `1`, `3`, `6`, or `12`, representing `Monthly`, `Quaterly`, `Semesterly`, or `Annually`, respectively. This field can also be `0`, in which case we need to  generate a 'one shot cashflow'. Skip to that section for its explanation.

#### Calculating Cashflow Dates

Here's a set of examples that will illustrate how the dates of cashflows are calculated, based on the input. *These are not separate cases you need to consider*. They're just examples illustrating how cashflow dates are calculated for every case.


**Example 1**: A Simple illustration. The next cashflow date is `payment_frequency` months after the previous date. We proceed doing this by starting with the start date and ending on the end date.
```
as_on_date: None
start_date: 10-01-2018
maturity_date: 10-04-2018
payment_frequency: 1

// Dates for Cashflows:
10-02-2018, 10-03-2018, 10-04-2018
```



**Example 2**: Extends the previous example by highlighting how the change in payment frequency affects the cashflow dates.
```
as_on_date: None
start_date: 10-12-2017
maturity_date: 10-12-2018
payment_frequency: 3

// Dates for Cashflows:
10-03-2018, 10-06-2018, 10-09-2018, 10-12-2018
```



**Example 3**: Capping the last date to the maturity date, when the last cashflow date would've been more than the maturity date.
```
as_on_date: None
start_date: 10-01-2018
maturity_date: 01-04-2018
payment_frequency: 1

// Dates for Cashflows:
10-02-2018, 10-03-2018, 01-04-2018
```
This example should illustrate the point that *a cashflow date will never be after the maturity date*.




**Example 4**: Describing dates if the program is given an `as-on-date`, and the date is before `start_date`. In this case, the cashflows are dated as they normally would be.
```
as_on_date: 03-10-2017
start_date: 10-01-2018
maturity_date: 01-04-2018
payment_frequency: 1

// Dates for Cashflows:
10-02-2018, 10-03-2018, 01-04-2018
```



**Example 5**: Describing dates if the program is given an `as-on-date`, and the date is after `start_date`. In this case, the cashflows' first date is after `as_on_date`, but it's a date that would've been in the chain of cashflow dates anyway; following cashflows are dated as they normally would be.
```
as_on_date: 20-02-2018  // as_on_date is greater than start_date
start_date: 10-01-2018
maturity_date: 01-04-2018
payment_frequency: 1

// Dates for Cashflows:
10-03-2018, 01-04-2018
```
For the first cashflow, the number of days will be calculated using (10-03-2018 - 20-02-2018)




**Example 6**: Preserve end-of-month-ness. When incrementing dates, if the date we are incrementing is the end of a month, we want the subsequent dates to always preserve this fact.
 ```
 as_on_date: None
 start_date: 31-12-2017
 maturity_date: 31-12-2018
 payment_frequency: 1

 // Dates for Cashflows:
 31-01-2018, 28-02-2018 (29th for a leap year), 31-03-2018, 30-04-2014, 31-04-2018 ...
 ```



**Example 7**: Ignore end-of-month-ness if it's encountered as a result of months being incremented.
 ```
 as_on_date: None
 start_date: 29-12-2017
 maturity_date: 29-12-2018
 payment_frequency: 1

 // Dates for Cashflows:
 29-01-2018, 28-02-2018 (29th for a leap year), 29-03-2018, 29-04-2014, 29-04-2018 ...
 ```
The difference between Examples 6 and 7 should highlight the importance of the `start_date`. If the `start_date` was the end of month, we preserve it throughout the consequent dates. If the end of month was encountered in one of the cashflow days, we don't care about it in the following cashflows.


## Cashflow Cases
Depending on the input's parameters, there are a few kinds of cashflow generation we do. Here's a high-level overview:
```
if current_book_balance < 0.0 {
        generate_negative_outstanding_balance_cashflow()
    }

    if interest_payment_frequency <= 0 {
        generate_one_shot_cashflow_at_maturity()
    }

    if compounding_frequency > 0 &&
        account.compounding_frequency < account.interest_payment_frequency {

        generate_compounding_interest_cashflows()

    }

    generate_simple_interest_cashflows()
```


### Negative Outstanding Amount
We generate one cashflow:
```
Cashflow {
    interest_amount: 0.0,
    principal_amount: input.outstanding_ammount,
    date: maturity_date
}
```

### One Shot Cashflow At Maturity
`number_of_days = maturity_date - start_date`
`interest_amount = /* Determine using formula */`

Generated Cashflow:
```
Cashflow {
    interest_amount: interest_amount,
    principal_amount: input.outstanding_ammount,
    date: maturity_date
}
```

### Simple Interest Cashflows
Go on calculating `interest_amount` using the formula mentioned above for each cashflow date. `number_of_days` can be calculated using the difference between cashflow dates explained above.

### Compounding Interest Cashflows (Optional)

*NOTE: Compounding Interest records were not present in the HDFC sample data. The program should handle it anyway.*

The difference between simple interest cashflows and compounding interest cashflows is this:
- Simple interest cashflows are paid (added to the Cashflows `Vec`) each time they are calculated. Compounding interest cashflows are calculated at a different rate compared to when they're paid out.
- Simple interest are always calculated against the same outstanding amount. Compounding interest cashflows are calculated against an increasing outstanding amount. The outstanding amount increases as it compounds. When the cashflow is generated, the outstanding amount is reset to its original value.

To determine the rate at which compounding interest cashflows should be calculated, an account has a `compounding_frequency` field. The `interest_payment_frequency` determines when the cashflow should be added to the `Vec`.


Here's an example of how compounding interest cashflows are generated.

If the Account has the values like so:
```
as_on_date: 31-07-18
maturity_date: 26-07-19
outstanding_amount: 361167.8
interest_rate: 6.25
compounding_frequency: Monthly
payment_frequency: Quarterly
```

The cashflows will be:

| Date     | Interest Amount | New Outstanding Amount | Notes                                                                                                           | Principal Amount |
|----------|-----------------|------------------------|-----------------------------------------------------------------------------------------------------------------|------------------|
| 30-04-18  | 1855.31         | 363023.11              | New Outstanding Amount = Previous Outstanding Amount (in this case the original 361167.8) + the Interest Amount | 0.0              |
| 31-05-18  | 1927.01         | 364950.12              | New Outstanding Amount = Previous Outstanding Amount (in this case 363023.11) + the Interest Amount             | 0.0              |
| 30-06-18  | 1874.74         | 361167.8               | Matches the payment frequency: Include this Cashflow in the Vec. Reset the outstanding Amount to the original   | 0.0              |
| 31-07-18  | 1917.16         | 363084.96              | -                                                                                                               | 0.0              |
| 31-08-18  | 1927.33         | 365012.29              | -                                                                                                               | 0.0              |
| 30-09-18  | 1875.06         | 361167.8               | Matches the payment frequency: Include this Cashflow in the Vec.                                                | 0.0              |
| 31-10-18 | 1917.16         | 363084.96              | -                                                                                                               | 0.0              |
| 30-11-18 | 1865.16         | 364950.12              | -                                                                                                               | 0.0              |
| 31-12-18 | 1937.24         | 361167.8               | Matches the payment frequency: Include this Cashflow in the Vec.                                                | 0.0              |
| 31-01-19  | 1917.16         | 363084.96              | -                                                                                                               | 0.0              |
| 28-02-19  | 1740.82         | 364825.78              | -                                                                                                               | 0.0              |
| 31-03-19  | 1936.58         | 361167.8               | Matches the payment frequency: Include this Cashflow in the Vec.                                                | 0.0              |
| 30-04-19  | 1855.31         | 363023.11              | -                                                                                                               | 0.0              |
| 31-05-19  | 1927.01         | 364950.12              | -                                                                                                               | 0.0              |
| 30-06-19  | 1874.74         | 361167.8               | Matches the payment frequency: Include this Cashflow in the Vec.                                                | 0.0              |
| 26-07-19  | 1607.94         | 362775.74              | Maturity date is met. Include this Cashflow in the Vec.                                                         | 361167.8         |

## Conditions of Note
- If `compounding_frequency <= payment_frequency`, calculate cashflows as per the simple interest cashflow generation criteria.
- If an account's `start_date > maturity_date`, log this account as an error.
