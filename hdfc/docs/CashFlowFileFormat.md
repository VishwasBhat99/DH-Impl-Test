# File Specifications

---
## Cashflow Generator

The cashflow generator generates three files.  This file details the format of these files.

## .cf

A `.cf` file is the cashflows file that’s a dump of all cashflows generated, at account level. 

![](cfformat.png)

The first 8 bytes denote the size of the `metadata` that follows. That is, if the `metadata` for a `.cf` file is `n` bytes long, the first 8 bytes are used to denote `n`. The `metadata` field is used to store details about the `.cf` file such as the version number, etc.

These bytes are followed by any number of cashflow records. A cashflow record uses 4 bytes to denote the size of the following `cashflow`. Each `cashflow` is a cashflow struct, serialized with Protocol Buffers version 3. This set of `size of cashflow`, and `cashflow` repeats any number of times.

## .idx

The `.idx` file has a list of indices that let you perform quick fetches of a specific cashflow in a corresponding `.cf` file.

![](idxformat.png)

The layout of the  `metadata` fields in an indices file is similar to that in a `.cf` file. The fields for an index needs to be decided. 

## .json

The `.json` file holds statistics about the cashflows generated. Here's a format of the `.json` file:
```
{
  "cashflowGenerationDate": "2018-07-06T15:25:58+00:00",
  "totalTimeTakenSeconds": "1800",
  "inputRecords": "200200",
  "outputRecords": "200000",
  "erroneousRecords": "200",
  "totalCashflowsGenerated": "800000",
  "totalAmountInInput": "100000000",
  "totalAmountInOutput": "400000000",
  "totalPrincipalInOutput": "500000",
  "totalInterestInOutput": "450000"
}
```

The `totalAmountInInput`, `totalPrincipalInOutput`, and `totalInterestInOutput` fields are values in a currency. The specific currency we store these data in is TBD.

---

## Cashflow Aggregator

Similar to the `.json` file produced by the cashflow generator, the cashflow file produces a health check report after the program ends. This report can be used to verify  all phases of the aggregator worked properly, and the ouputs match what was expected on the input.

Sample file example:

```json
{
    "input": {
        "accountsCount": 50,
        "cashflowsCount": 80,
        "totalPrincipalAmount": 2000.0,
        "totalInterestAmount": 1000.0,
        "totalOutstandingAmount": 3000.0
    },

    "llgs": {
        "4412-INR": {
            "accountsCount": 10,
            "cashflowsCount": 20,
            "totalPrincipalAmount": 100.0,
            "totalInterestAmount": 50.0,
            "group0": {
                "totalPrincipalAmount": 10.0,
                "totalInterestAmount": 5.0
            },
            "group1": {
                "totalPrincipalAmount": 10.0,
                "totalInterestAmount": 5.0
            },
            "group2": {
                "totalPrincipalAmount": 10.0,
                "totalInterestAmount": 5.0
            },
            "group3": {
                "totalPrincipalAmount": 10.0,
                "totalInterestAmount": 5.0
            },
            "group4": {
                "totalPrincipalAmount": 30.0,
                "totalInterestAmount": 15.0
            },
            "group5": {
                "totalPrincipalAmount": 30.0,
                "totalInterestAmount": 15.0
            }
        },
        "4400-INR": {
            "accountsCount": 10,
            "cashflowsCount": 20,
            "totalPrincipalAmount": 110.0,
            "totalInterestAmount": 50.0,
            "group0": {
                "totalPrincipalAmount": 10.0,
                "totalInterestAmount": 5.0
            },
            "group1": {
                "totalPrincipalAmount": 10.0,
                "totalInterestAmount": 5.0
            },
            "group2": {
                "totalPrincipalAmount": 10.0,
                "totalInterestAmount": 5.0
            },
            "group3": {
                "totalPrincipalAmount": 10.0,
                "totalInterestAmount": 5.0
            },
            "group4": {
                "totalPrincipalAmount": 30.0,
                "totalInterestAmount": 15.0
            },
            "group5": {
                "totalPrincipalAmount": 30.0,
                "totalInterestAmount": 15.0
            }
        }
    },

    "llgSummaries": {
        "4412-INR": {
            "totalPrincipalAmount": 100.0,
            "totalInterestAmount": 50.0
        },
        "4400-INR": {
            "totalPrincipalAmount": 100.0,
            "totalInterestAmount": 50.0
        }
    },

    "outputRecordCounts": {
        "group0": 6,
        "group1": 6,
        "group2": 6,
        "group3": 6,
        "group4": 6,
        "group5": 5
    },

    "healthChecks": {
        "inputToLLGsAccountCountDifference": 0,
        "inputToLLGsCashflowCountDifference": 0.0,
        "inputToLLGsPrincipalAmountDifference": 0.0,
        "inputToLLGsInterestAmountDifference": 0.0,
        "llgsWithMismatchedPrincipalAmountGroupDistribution": ["4400-INR"],
        "llgsWithMismatchedInterestAmountGroupDistribution": [],
        "llgsWithSummaryMismatches": {
            "4400-INR": {
                "principalAmountDifference": -10.0,
                "interstAmountDifference": 0.0
            }
        },
        "groupsWithIncorrectOutputRecords": [ "bucket5" ]
    }
}
```

A more detailed explanation is forthcoming...
