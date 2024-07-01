#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct PeriodKey {
    pub start_days: i64,
    pub end_days: i64,
}

impl PeriodKey {
    pub fn get_period_key(fields: Vec<&str>) -> PeriodKey {
        let start_days = 365 * fields[0].parse::<i64>().unwrap_or(0)
            + 30 * fields[1].parse::<i64>().unwrap_or(0)
            + fields[2].parse::<i64>().unwrap_or(0);
        let end_days = 365 * fields[3].parse::<i64>().unwrap_or(0)
            + 30 * fields[4].parse::<i64>().unwrap_or(0)
            + fields[5].parse::<i64>().unwrap_or(0);
        PeriodKey {
            start_days,
            end_days,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TimeBandData {
    pub timeband: String,
    pub period: String,
    pub zone: String,
}

impl TimeBandData {
    pub fn new() -> TimeBandData {
        TimeBandData {
            timeband: "NA".to_string(),
            period: "NA".to_string(),
            zone: "NA".to_string(),
        }
    }

    pub fn get_timeband_fields(fields: Vec<&str>) -> TimeBandData {
        TimeBandData {
            timeband: fields[6].to_string(),
            period: fields[7].to_string(),
            zone: fields[8].to_string(),
        }
    }
}
