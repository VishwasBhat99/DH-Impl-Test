use dbpool::OracleConnectionManager;
use r2d2::Pool;

pub fn get_cust_type(
    cust_id: &String,
    pool: Pool<OracleConnectionManager>,
) -> Result<String, String> {
    let conn = &pool.get().unwrap().conn;
    match conn {
        Some(db) => {
            let sql = "select CUST_TYPE from CUST_MASTER where CUST_ID = :1";
            let rows = db
                .conn
                .query(sql, &[cust_id])
                .expect("Query Failed to Fetch CUST_TYPE Data from DB.");
            for row_result in &rows {
                let row = row_result.expect("Failed to read query output.");
                let cust_type: String = row.get("CUST_TYPE").unwrap_or("NONE".to_string());
                return Ok(cust_type);
            }
            return Ok("NONE".to_string());
        }
        None => {
            return Err(format!(
                "Cannot not connect to oracle db. Please check credentials."
            ))
        }
    }
}
