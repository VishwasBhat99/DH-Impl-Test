use rbdate::DateParser;

pub fn get_op_line(cf_data: &Vec<Vec<String>>, ip_fields: &mut Vec<&str>, op_line: &mut String) {
    let mut flow_amt1 = 0.0;
    let mut flow_amt2 = 0.0;
    let date_parser1 = DateParser::new("%d-%m-%y".to_string(), false);
    let date_parser2 = DateParser::new("%d-%m-%Y".to_string(), false);

    let delivery_date = date_parser2
        .parse_opt(&ip_fields[21].to_string().replace('/', &'-'.to_string()))
        .expect("Cannot parse Delivery Date");

    for record in cf_data {
        let cf_date = date_parser1
            .parse_opt(&record[30].replace('/', &'-'.to_string()))
            .expect("Cannot parse Cashflow Date");
        if record[25] == *"IPAY".to_string()
            && record[31] == ip_fields[33]
            && ip_fields[51] == ip_fields[33]
            && delivery_date == cf_date
        {
            flow_amt1 += record[29].to_string().parse::<f64>().unwrap_or(0.0);
        }
        if record[25] == *"IPAY".to_string()
            && record[31] == ip_fields[33]
            && ip_fields[48] == ip_fields[33]
            && delivery_date == cf_date
        {
            flow_amt2 += record[29].to_string().parse::<f64>().unwrap_or(0.0);
        }
    }
    let final_amt1 = format!(
        "{:.4}",
        ip_fields[49].parse::<f64>().unwrap_or(0.0) - flow_amt1
    );
    let final_amt2 = format!(
        "{:.4}",
        ip_fields[52].parse::<f64>().unwrap_or(0.0) - flow_amt2
    );
    if final_amt1.parse().unwrap_or(0.0) != 0.0 && ip_fields[51] != "IN1" {
        op_line.push_str(ip_fields[0]);
        op_line.push('|');
        op_line.push_str(ip_fields[1]);
        op_line.push('|');
        op_line.push_str(ip_fields[2]);
        op_line.push('|');
        append_input_fields(op_line, ip_fields.clone(), 3_usize, 16_usize);
        op_line.push_str(ip_fields[17]);
        op_line.push('|');
        op_line.push_str(
            &date_parser2
                .parse_opt(&ip_fields[18].to_string().replace('/', &'-'.to_string()))
                .expect("Cannot parse Trade Date")
                .format("%d-%m-%Y")
                .to_string(),
        );
        op_line.push('|');
        op_line.push_str(
            &date_parser2
                .parse_opt(&ip_fields[19].to_string().replace('/', &'-'.to_string()))
                .expect("Cannot parse Start Date")
                .format("%d-%m-%Y")
                .to_string(),
        );
        op_line.push('|');
        op_line.push_str(
            &date_parser2
                .parse_opt(&ip_fields[20].to_string().replace('/', &'-'.to_string()))
                .expect("Cannot parse Expiry Date")
                .format("%d-%m-%Y")
                .to_string(),
        );
        op_line.push('|');
        op_line.push_str(&delivery_date.format("%d-%m-%Y").to_string());
        op_line.push('|');
        append_input_fields(op_line, ip_fields.clone(), 22_usize, 36_usize);
        op_line.push_str(
            &date_parser2
                .parse_opt(&ip_fields[37].to_string().replace('/', &'-'.to_string()))
                .expect("Cannot parse Premium Settlement Date")
                .format("%d %m %Y")
                .to_string(),
        );
        op_line.push('|');
        append_input_fields(op_line, ip_fields.clone(), 38_usize, 69_usize);
        op_line.push_str(ip_fields[71]);
        op_line.push_str("|Delta|CCY1|CCY1|CCY1|CCY1|");
        op_line.push_str(&final_amt1);
        op_line.push('|');
        op_line.push_str(ip_fields[33]);
        op_line.push('|');
        op_line.push_str(&delivery_date.format("%d-%m-%Y").to_string());
        op_line.push('|');
        op_line.push_str(ip_fields[72]);
        op_line.push('|');
        op_line.push_str(ip_fields[75]);
        op_line.push('|');
        op_line.push_str(
            &date_parser2
                .parse_opt(&ip_fields[37].to_string().replace('/', &'-'.to_string()))
                .expect("Cannot parse Premium Settlement Date")
                .format("%d-%m-%Y")
                .to_string(),
        );
        op_line.push('|');
        append_input_fields(op_line, ip_fields.clone(), 77_usize, 85_usize);
        // op_line.push('1');
        op_line.pop();
        op_line.push('\n');
    }
    if final_amt2.parse().unwrap_or(0.0) != 0.0 && ip_fields[48] != "IN1" {
        op_line.push_str(ip_fields[0]);
        op_line.push('|');
        op_line.push_str(ip_fields[1]);
        op_line.push('|');
        op_line.push_str(ip_fields[2]);
        op_line.push('|');
        append_input_fields(op_line, ip_fields.clone(), 3_usize, 16_usize);
        op_line.push_str(ip_fields[17]);
        op_line.push('|');
        op_line.push_str(
            &date_parser2
                .parse_opt(&ip_fields[18].to_string().replace('/', &'-'.to_string()))
                .expect("Cannot parse Trade Date")
                .format("%d-%m-%Y")
                .to_string(),
        );
        op_line.push('|');
        op_line.push_str(
            &date_parser2
                .parse_opt(&ip_fields[19].to_string().replace('/', &'-'.to_string()))
                .expect("Cannot parse Start Date")
                .format("%d-%m-%Y")
                .to_string(),
        );
        op_line.push('|');
        op_line.push_str(
            &date_parser2
                .parse_opt(&ip_fields[20].to_string().replace('/', &'-'.to_string()))
                .expect("Cannot parse Expiry Date")
                .format("%d-%m-%Y")
                .to_string(),
        );
        op_line.push('|');
        op_line.push_str(&delivery_date.format("%d-%m-%Y").to_string());
        op_line.push('|');
        append_input_fields(op_line, ip_fields.clone(), 22_usize, 36_usize);
        op_line.push_str(
            &date_parser2
                .parse_opt(&ip_fields[37].to_string().replace('/', &'-'.to_string()))
                .expect("Cannot parse Premium Settlement Date")
                .format("%d %m %Y")
                .to_string(),
        );
        op_line.push('|');
        append_input_fields(op_line, ip_fields.clone(), 38_usize, 69_usize);
        op_line.push_str(ip_fields[71]);
        op_line.push_str("|Delta|CCY2|CCY2|CCY2|CCY2|");
        op_line.push_str(&final_amt2);
        op_line.push('|');
        op_line.push_str(ip_fields[33]);
        op_line.push('|');
        op_line.push_str(&delivery_date.format("%d-%m-%Y").to_string());
        op_line.push('|');
        op_line.push_str(ip_fields[72]);
        op_line.push('|');
        op_line.push_str(ip_fields[75]);
        op_line.push('|');
        op_line.push_str(
            &date_parser2
                .parse_opt(&ip_fields[37].to_string().replace('/', &'-'.to_string()))
                .expect("Cannot parse Premium Settlement Date")
                .format("%d-%m-%Y")
                .to_string(),
        );
        op_line.push('|');
        append_input_fields(op_line, ip_fields.clone(), 77_usize, 85_usize);
        // op_line.push('2');
        op_line.pop();
        op_line.push('\n');
    }
}

fn append_input_fields(
    op_line: &mut String,
    ip_fields: Vec<&str>,
    start_index: usize,
    end_index: usize,
) {
    for index in start_index..end_index + 1 {
        op_line.push_str(ip_fields[index]);
        op_line.push('|');
    }
}

pub fn get_last_line(cf_data: &Vec<Vec<String>>, ip_fields: &mut Vec<&str>, op_line: &mut String) {
    let date_parser1 = DateParser::new("%d-%m-%y".to_string(), false);
    let date_parser2 = DateParser::new("%d-%m-%Y".to_string(), false);

    let delivery_date = date_parser2
        .parse_opt(&ip_fields[21].to_string().replace('/', &'-'.to_string()))
        .expect("Cannot parse delivery date");

    for record in cf_data {
        let cf_date = date_parser1
            .parse_opt(&record[30].replace('/', &'-'.to_string()))
            .expect("Cannot parse cashflow date");
        let flow_amt = format!("{:.4}", record[29].parse::<f64>().unwrap_or(0.0));
        if record[10] == "OPT"
            && (delivery_date.to_string() == "" || cf_date != delivery_date)
            && ((record[24] == "CAP"
                && (record[25] != "IPAY" || record[28] != "INT")
                && (record[25] != "XIT" || record[28] != "INT")
                && (record[25] != "IPAY" || record[28] != "MID" || record[26] != "ADD")
                && (record[25] != "IPAY" || record[28] != "ADD"))
                || (record[24] == "BRK"
                    && (record[25] != "BFEE" || record[28] != "INT")
                    && (record[25] != "CIFE" || record[28] != "INT")))
            && flow_amt.parse::<f64>().unwrap_or(0.0) != 0.0
        {
            op_line.push_str(&record[1]);
            op_line.push('|');
            op_line.push_str(record[2].to_string().split('.').collect::<Vec<&str>>()[0]);
            op_line.push('|');
            op_line.push_str(&record[3]);
            op_line.push('|');
            append_input_fields(op_line, ip_fields.clone(), 3_usize, 16_usize);

            if record[11] == *"INT".to_string() {
                op_line.push_str("I|");
            } else {
                op_line.push_str("E|");
            }
            op_line.push_str(
                &date_parser2
                    .parse_opt(&ip_fields[18].to_string().replace('/', &'-'.to_string()))
                    .expect("Cannot parse Trade Date.")
                    .format("%d-%m-%Y")
                    .to_string(),
            );
            op_line.push('|');
            op_line.push_str(
                &date_parser2
                    .parse_opt(&ip_fields[19].to_string().replace('/', &'-'.to_string()))
                    .expect("Cannot parse Delivery Date")
                    .format("%d-%m-%Y")
                    .to_string(),
            );
            op_line.push('|');
            op_line.push_str(
                &date_parser2
                    .parse_opt(&ip_fields[20].to_string().replace('/', &'-'.to_string()))
                    .expect("Cannot parse Expiry Date")
                    .format("%d-%m-%Y")
                    .to_string(),
            );
            op_line.push('|');
            op_line.push_str(&delivery_date.format("%d-%m-%Y").to_string());
            op_line.push('|');
            append_input_fields(op_line, ip_fields.clone(), 22_usize, 36_usize);
            op_line.push_str(
                &date_parser2
                    .parse_opt(&ip_fields[37].to_string().replace('/', &'-'.to_string()))
                    .expect("Cannot parse premium settlement date")
                    .format("%d %m %Y")
                    .to_string(),
            );
            op_line.push('|');
            append_input_fields(op_line, ip_fields.clone(), 38_usize, 69_usize);

            op_line.push_str(ip_fields[71]);
            op_line.push('|');

            if !record[24].is_empty() {
                op_line.push_str(&record[24]);
            } else {
                op_line.push_str("NA");
            }
            op_line.push('|');
            if !record[25].is_empty() {
                op_line.push_str(&record[25]);
            } else {
                op_line.push_str("NA");
            }
            op_line.push('|');
            if !record[26].is_empty() {
                op_line.push_str(&record[26]);
            } else {
                op_line.push_str("NA");
            }
            op_line.push('|');
            if !record[27].is_empty() {
                op_line.push_str(&record[27]);
            } else {
                op_line.push_str("NA");
            }
            op_line.push('|');
            if !record[28].is_empty() {
                op_line.push_str(&record[28]);
            } else {
                op_line.push_str("NA");
            }
            op_line.push('|');
            if !record[29].is_empty() {
                op_line.push_str(&record[29]);
            } else {
                op_line.push('0')
            }
            op_line.push('|');
            if !record[31].is_empty() {
                op_line.push_str(&record[31]);
            } else {
                op_line.push('0')
            }
            op_line.push('|');
            op_line.push_str(&cf_date.format("%d-%m-%Y").to_string());
            op_line.push('|');
            op_line.push_str(ip_fields[72]);
            op_line.push('|');
            op_line.push_str(ip_fields[75]);
            op_line.push('|');
            op_line.push_str(
                &date_parser2
                    .parse_opt(&ip_fields[37].to_string().replace('/', &'-'.to_string()))
                    .expect("Cannot parse premium settlement date")
                    .format("%d-%m-%Y")
                    .to_string(),
            );
            op_line.push('|');
            append_input_fields(op_line, ip_fields.clone(), 77_usize, 85_usize);
            // op_line.push('3');
            op_line.pop();
            op_line.push('\n');
        }
    }
}
