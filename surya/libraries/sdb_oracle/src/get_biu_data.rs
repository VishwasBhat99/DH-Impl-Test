use dbpool::OracleConnectionManager;
use r2d2::Pool;

pub fn get_biu_data(
    cust_id: &String,
    pool: Pool<OracleConnectionManager>,
) -> Result<(String, String, String, String), String> {
    let conn = &pool.get().unwrap().conn;
    match conn {
        Some(db) => {
            let sql = "select T1,T2,T3,NOB from BIU_MASTER where CUST_ID = :1";
            let rows = db
                .conn
                .query(sql, &[cust_id])
                .expect("Query Failed to Fetch CUST_TYPE Data from DB.");
            for row_result in &rows {
                let row = row_result.expect("Failed to read query output.");
                let t1: String = row.get("T1").unwrap_or("NONE".to_string());
                let t2: String = row.get("T2").unwrap_or("NONE".to_string());
                let t3: String = row.get("T3").unwrap_or("NONE".to_string());
                let nob: String = row.get("NOB").unwrap_or("NONE".to_string());
                return Ok((t1,t2,t3,nob));
            }
            return Ok(("NONE".to_string(),"NONE".to_string(),"NONE".to_string(),"NONE".to_string()));
        }
        None => {
            return Err(format!(
                "Cannot not connect to oracle db. Please check credentials."
            ))
        }
    }
    return Err(format!("Exiting DB."));
}
