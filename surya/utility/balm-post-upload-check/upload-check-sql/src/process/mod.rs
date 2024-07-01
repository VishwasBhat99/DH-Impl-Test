use configuration_parameters::ConfigurationParameters;
use macros;
use odbc::*;
use sdb_io::new_buf_rdr;
use slog::Logger;
use std::collections::HashMap;
pub fn process(config_params: &ConfigurationParameters, logger: &Logger, diag_logger: &Logger) {
    let con_str = config_params.connection_string();
    let env = create_environment_v3()
        .map_err(|e| e.unwrap())
        .expect("Cannot create DB environment.");
    let conn = env
        .connect_with_connection_string(&con_str)
        .expect("Cannot establish a DB connection.");

    let mut records: HashMap<String, i32> = HashMap::new();
    let stmt =
        Statement::with_parent(&conn).expect("Cannot create a statement instance to run queries.");

    let sql_cmd = format!(
        "select * from tblProductTotals where As_on=\'{}\'",
        config_params.as_on_date().format("%d-%m-%Y").to_string()
    );
    match stmt
        .exec_direct(&sql_cmd)
        .expect("Failed to execute a sql cmd!!")
    {
        Data(mut stmt) => {
            while let Some(mut cursor) = stmt.fetch().expect("Cannot read output of query.") {
                let mut record_key = String::new();
                match cursor
                    .get_data::<&str>(1)
                    .expect("Cannot read row data from query.")
                {
                    Some(val) => record_key.push_str(val),
                    None => {}
                }
                match cursor
                    .get_data::<&str>(2)
                    .expect("Cannot read row data from query.")
                {
                    Some(val) => {
                        record_key.push_str("|");
                        record_key.push_str(val)
                    }
                    None => {}
                }
                match cursor
                    .get_data::<&str>(3)
                    .expect("Cannot read row data from query.")
                {
                    Some(val) => {
                        record_key.push_str("|");
                        record_key.push_str(val)
                    }
                    None => {}
                }
                let all_slr = cursor
                    .get_data::<&str>(4)
                    .expect("Cannot read row data from query.");
                let count = records.get(&record_key).unwrap_or(&0);
                if all_slr == Some("ALL") {
                    records.insert(record_key, *count + 1);
                } else {
                    records.insert(record_key, *count - 1);
                }
            }
        }
        NoData(_) => {
            println!("Query \"{}\" executed, no data returned", sql_cmd);
        }
    }
    log_debug!(logger, "Records present are: {:?}", records);
    for record_key in records {
        if record_key.1 != -3 && record_key.1 != 1 {
            log_error!(logger, "Records are not proper for: {}", record_key.0);
            panic!("Data is not proper")
        }
    }
}
