use super::{ConfigurationParameters, CurrencyConverter};
use chrono::{Datelike, Local, NaiveDate};

pub fn get_op_line(
    ccy: &CurrencyConverter,
    as_on_dt: NaiveDate,
    config_params: &ConfigurationParameters,
) -> String {
    let dt = as_on_dt.format("%d-%m-%Y");

    let now = Local::now();
    let crnt_dt = NaiveDate::from_ymd(now.year(), now.month(), now.day())
        .format("%d-%m-%Y")
        .to_string();

    let mut output_line = String::new();

    if config_params.ccy() == ccy.source {
        output_line.push_str(&ccy.print());
        output_line.push('|');
        output_line.push_str(&dt.to_string());
        output_line.push_str("|Y|dataharmonizer|");
        output_line.push_str(&crnt_dt);
        output_line.push_str("|dataharmonizer|");
        output_line.push_str(&crnt_dt);
        output_line.push('\n');

        output_line.push_str(&ccy.print_rev_order());
        output_line.push('|');
        output_line.push_str(&dt.to_string());
        output_line.push_str("|Y|dataharmonizer|");
        output_line.push_str(&crnt_dt);
        output_line.push_str("|dataharmonizer|");
        output_line.push_str(&crnt_dt);
        output_line.push('\n');
    } else if config_params.ccy() == ccy.target {
        let swap_ccy = ccy.swap();
        output_line.push_str(&swap_ccy.print());
        output_line.push('|');
        output_line.push_str(&dt.to_string());
        output_line.push_str("|Y|dataharmonizer|");
        output_line.push_str(&crnt_dt);
        output_line.push_str("|dataharmonizer|");
        output_line.push_str(&crnt_dt);
        output_line.push('\n');

        output_line.push_str(&swap_ccy.print_rev_order());
        output_line.push('|');
        output_line.push_str(&dt.to_string());
        output_line.push_str("|Y|dataharmonizer|");
        output_line.push_str(&crnt_dt);
        output_line.push_str("|dataharmonizer|");
        output_line.push_str(&crnt_dt);
        output_line.push('\n');
    } else {
        panic!(
            "`{}` is not present on line: `{:?}`",
            config_params.ccy(),
            ccy
        );
    }
    output_line
}

pub fn append_op_line(lines: &mut String, config_param: &ConfigurationParameters) {
    let dt = config_param.as_on_date().format("%d-%m-%Y");

    let now = Local::now();
    let crnt_dt = NaiveDate::from_ymd(now.year(), now.month(), now.day())
        .format("%d-%m-%Y")
        .to_string();

    lines.push_str(&format!(
        "{}|{}|1.0|{}|Y|dataharmonizer|{}|dataharmonizer|{}\n",
        config_param.ccy(),
        config_param.lcy(),
        dt.to_string(),
        crnt_dt,
        crnt_dt,
    ));
    lines.push_str(&format!(
        "{}|{}|1.0|{}|Y|dataharmonizer|{}|dataharmonizer|{}\n",
        config_param.lcy(),
        config_param.ccy(),
        dt.to_string(),
        crnt_dt,
        crnt_dt,
    ));
    lines.push_str(&format!(
        "{}|{}|1.0|{}|Y|dataharmonizer|{}|dataharmonizer|{}\n",
        config_param.ccy(),
        config_param.ccy(),
        dt.to_string(),
        crnt_dt,
        crnt_dt,
    ));
    lines.push_str(&format!(
        "{}|{}|1.0|{}|Y|dataharmonizer|{}|dataharmonizer|{}\n",
        config_param.fcy(),
        config_param.ccy(),
        dt.to_string(),
        crnt_dt,
        crnt_dt,
    ));
    lines.push_str(&format!(
        "{}|{}|1.0|{}|Y|dataharmonizer|{}|dataharmonizer|{}\n",
        config_param.ccy(),
        config_param.fcy(),
        dt.to_string(),
        crnt_dt,
        crnt_dt,
    ));
}
