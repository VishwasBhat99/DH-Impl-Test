# IA Aggregator

IA processes the ".cf" output file of the cashflows generator. It aggregates the cashflows into 731 buckets(a data structure which represents data for a day) that represents two years, starting from the `as-on-date`. Each bucket holds the total outstanding amount, and the weighted interest rate as of that day. Cashflows represent a decrement in outstanding amount over time. The IA aggregator tracks the progress of how each account’s outstanding balance reduces to zero over time. All the cashflows after 2 years are aggregated on the 731<sup>st</sup> day.

*Note: We use the [weighted average](https://en.wikipedia.org/wiki/Weighted_arithmetic_mean) technique to aggregate the interest rate and the date for all the accounts for each bucket.*

## Input

The ".cf" output file of the cashflows generator consists of all account data along with all the cashflows generated for every account. 
Here's the sample structure for a Cashflow:
```
struct Cashflow {
    interest_amount: f64,
    principal_amount: f64,
    date: i64
}
```

## Output

We aggregate all the accounts based on the llg for each account and write all the aggregated data across 731 buckets into a single file. An llg consists of `llg id`, `currency`(pass-through field), and `benchmark id`(pass-through field). To determine the llg id for an account we use rules based on the pass-through data. For example in loans we match `concat` and `npa status` fields(these fields come as pass-through data) with the conditions in the llg configuration file. Here is a sample of the loans [llg configuration file](https://docs.google.com/spreadsheets/d/1rADdGdrKDK8O9ZPHEIaGm1nAQDXubtRr4vlYN_QkL64/edit?usp=sharing).

## Flowchart

[IA-flowchart](https://drive.google.com/file/d/1wbvjctr2TKzTLE2jF8iHH2m-P3uPKq9z/view?usp=sharing)

## Example

An example for [IA Simulation](https://docs.google.com/spreadsheets/d/1qvdsiqxs-OSQhSXtQf3sSpFDY83jvOhkoehkIFtYH-U/edit?usp=sharing).