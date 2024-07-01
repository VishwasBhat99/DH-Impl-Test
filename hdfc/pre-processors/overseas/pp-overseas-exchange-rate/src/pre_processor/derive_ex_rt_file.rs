use super::{ConfigurationParameters, CurrencyConverter};

pub fn get_ex_rt_lines(
    ccy: &CurrencyConverter,
    lcy_equi_rt: f64,
    config_params: &ConfigurationParameters,
) -> String {
    let mut output_line = String::new();

    if config_params.ccy() == ccy.source {
        output_line.push_str(&ccy.print());
        output_line.push('\n');
        output_line.push_str(&ccy.print_rev_order());
        output_line.push('\n');
        output_line.push_str(&ccy.print_lcy_equi_rt(lcy_equi_rt));
        output_line.push('\n');
    } else if config_params.ccy() == ccy.target {
        let swap_ccy = ccy.swap();
        output_line.push_str(&swap_ccy.print());
        output_line.push('\n');
        output_line.push_str(&swap_ccy.print_rev_order());
        output_line.push('\n');
        output_line.push_str(&swap_ccy.print_lcy_equi_rt(lcy_equi_rt));
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

pub fn append_ccy(
    lines: &mut String,
    usl_inr_conv_val: f64,
    config_params: &ConfigurationParameters,
) {
    let ccy = config_params.ccy();
    let lcy = config_params.lcy();
    let fcy = config_params.fcy();
    lines.push_str(&format!("{}|{}|{}\n", "USL", "INR", usl_inr_conv_val));
    lines.push_str(&format!("{}|{}|{}\n", ccy, lcy, "1.0"));
    lines.push_str(&format!("{}|{}|{}\n", lcy, ccy, "1.0"));
    lines.push_str(&format!("{}|{}|{}\n", ccy, ccy, "1.0"));
    lines.push_str(&format!("{}|{}|{}\n", fcy, ccy, "1.0"));
    lines.push_str(&format!("{}|{}|{}\n", ccy, fcy, "1.0"));
}
