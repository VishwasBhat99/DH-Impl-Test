use super::manual_handler::remove_comma;
use macros;
use rbdate::NaiveDate;
use slog::Logger;

pub fn get_output_line(row: &Vec<String>, as_on_dt: &NaiveDate, log: &Logger) -> String {
    let mut output_line = String::new();
    let st_dt: String = match NaiveDate::parse_from_str(&row[20], "%d-%m-%y") {
        Ok(dt) => dt.format("%d-%m-%Y").to_string(),
        Err(error) => {
            log_error!(
                log,
                "start_date: `{}` for account: `{}` not in `DD-MM-YY` format: `{}`.",
                row[20],
                row[1],
                error,
            );
            String::new()
        }
    };
    let mat_dt: String = match NaiveDate::parse_from_str(&row[21], "%d-%m-%y") {
        Ok(dt) => dt.format("%d-%m-%Y").to_string(),
        Err(error) => {
            log_error!(
                log,
                "maturity_date: `{}` for account: `{}` not in `DD-MM-YY` format: `{}`.",
                row[21],
                row[1],
                error,
            );
            String::new()
        }
    };

    output_line.push_str(&row[1]);
    output_line.push('|');
    output_line.push_str(&row[2]);
    output_line.push_str("|");
    output_line.push_str(&row[7]);
    output_line.push('|');
    output_line.push_str(&row[11]);
    output_line.push('|');
    output_line.push_str(&st_dt);
    output_line.push('|');
    output_line.push_str(&mat_dt);
    output_line.push('|');
    output_line.push_str(&remove_comma(&row[26]));
    output_line.push('|');
    output_line.push_str(&row[29]);
    output_line.push('|');
    output_line.push_str(&row[31]);
    output_line.push('|');
    output_line.push_str(&row[38]);
    output_line.push('|');
    output_line.push_str(&row[41]);
    output_line.push('|');
    output_line.push_str(&row[44]);
    output_line.push('|');
    output_line.push_str(&row[45]);
    output_line.push('|');
    output_line.push_str(&row[49]);
    output_line.push('|');
    output_line.push_str(&row[52]);
    output_line.push('|');
    output_line.push_str(&as_on_dt.format("%d-%m-%Y").to_string());
    output_line.push('\n');

    output_line
}
