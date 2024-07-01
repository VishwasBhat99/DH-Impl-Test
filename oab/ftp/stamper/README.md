# LLG Stamper
These programs stamp llg_id to the cashflow data using the llg stamping rules. This program runs immediately after cf programs.

Note: Any changes made in output structure of cashflow program, must be updated in it's corresponding LLG Stamper program.

## Input
1. Cashflow data file in .cf format

2. Metadata containing the fields from the cashflow program's output struct.

3. Rules to determine which llg_id is stamped to which account.

## Output
1. Stamped cashflow file in .cf data with an additional llg_id field

## Further Reading

[Google Doc](https://docs.google.com/document/d/1ziUkA2CPJRMw8Kt4blisWKtWLKvVgNyoErMQ_OEkLc4/edit?usp=sharing)