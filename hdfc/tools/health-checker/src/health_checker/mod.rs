use configuration_parameters::ConfigurationParameters;
use health_report::HealthReport;
use std::time::SystemTime;

pub fn process(config_param: ConfigurationParameters) {
    let start_write_timer = SystemTime::now();
    let health_stat = HealthReport::new(
        config_param.tot_acc(),
        config_param.tot_succ(),
        if config_param.tot_fail() == 0 && config_param.derive_fail_recs() {
            config_param.tot_acc() - config_param.tot_succ()
        } else {
            config_param.tot_fail()
        },
        config_param.tot_amt_inp(),
        config_param.tot_amt_op(),
        config_param.tot_cfs(),
    );
    health_stat.gen_health_rpt(config_param.output_file_path());
    let end_write_timer = SystemTime::now();
    let duration = end_write_timer
        .duration_since(start_write_timer)
        .expect("Could not calculate total duration for writing health check report file.");
    println!("Total duration for health check report: `{:?}`", duration);
}
