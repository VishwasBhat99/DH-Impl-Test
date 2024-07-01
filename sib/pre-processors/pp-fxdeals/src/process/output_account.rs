use crate::process::output_account;

#[derive(Debug, Clone, Default)]
pub struct OutputField {
    pub as_on_date: String,
    pub deal_number: String,
    pub deal_date: String,
    pub product_type: String,
    pub deal_ref: String,
    pub transaction_type: String,
    pub portfolio: String,
    pub counter_party: String,
    pub counterparty_category: String,
    pub internal_external_deal_type: String,
    pub maturity_date: String,
    pub crncy1: String,
    pub crncy2: String,
    pub deal_rate: String,
    pub crncy1_amt: String,
    pub crncy2_amt: String,
    pub reval_rate: String,
    pub reval_profit: String,
    pub reval_loss: String,
    pub profit_and_loss_amount: String,
    pub m_duration: String,
    pub treasury_gl_code: String,
}

pub fn format_output(output_record: OutputField) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        output_record.as_on_date,
        output_record.deal_number,
        output_record.deal_date,
        output_record.product_type,
        output_record.deal_ref,
        output_record.transaction_type,
        output_record.portfolio,
        output_record.counter_party,
        output_record.counterparty_category,
        output_record.internal_external_deal_type,
        output_record.maturity_date,
        output_record.crncy1,
        output_record.crncy2,
        output_record.deal_rate,
        output_record.crncy1_amt,
        output_record.crncy2_amt,
        output_record.reval_rate,
        output_record.reval_profit,
        output_record.reval_loss,
        output_record.profit_and_loss_amount,
        output_record.m_duration,
        output_record.treasury_gl_code,
    )
}
pub fn get_writer(file_path: &str) -> std::io::BufWriter<std::fs::File> {
    match sdb_io::buf_file_wrtr(file_path, None) {
        Ok(file) => file,
        Err(error) => panic!("Unable to create file `{}` due to: {}", file_path, error),
    }
}
