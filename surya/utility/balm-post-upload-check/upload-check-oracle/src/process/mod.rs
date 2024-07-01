use configuration_parameters::ConfigurationParameters;
use macros;
use odbc::*;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
use dbpool::OracleConnectionManager;
use r2d2::Pool;

pub fn process(pool: Pool<OracleConnectionManager>,config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let mut records: HashMap<String, i32> = HashMap::new();
    let conn = &pool
        .get()
        .expect("Could not fetch db connection from connection pool!")
        .conn;
    match conn {
        Some(db) => {
            let sql = format!("select * from \"tblProductTotals\" where \"As_On\"= \'{}\'",config_params.as_on_date().format("%d-%m-%Y").to_string());  
            let rows = db
                .conn
                .query(&sql, &[])
                .expect("Query Failed to Fetch max(RunID) Data from DB.");
            for row_result in &rows {
                let mut record_key = String::new();
                let row = row_result.expect("Failed to read query output.");
                let run_id: String = row.get("Currency_ID").unwrap_or("0".to_string());
                record_key.push_str(&row.get("SubType_ID").unwrap_or("0".to_string()).to_string());
                record_key.push_str("|");
                record_key.push_str(&row.get("As_On").unwrap_or("0".to_string()).to_string());
                record_key.push_str("|");
                record_key.push_str(&row.get("Currency_ID").unwrap_or("0".to_string()).to_string());
                
                let all_slr = row.get("SLRorIRS").unwrap_or("0".to_string());
                let count = records.get(&record_key).unwrap_or(&0);
                if all_slr == "ALL" {
                    records.insert(record_key, *count + 1);
                } else {
                    records.insert(record_key, *count - 1);
                }
            } 
        }
        None => {}
    }
    log_debug!(logger, "Records Present are: {:?}", records);
    for record_key in records {
        if record_key.1 != -3 && record_key.1 != 1 {
            log_error!(logger, "Records are not proper for: {}", record_key.0);
            panic!("Data is not proper")
        }
    }
}
