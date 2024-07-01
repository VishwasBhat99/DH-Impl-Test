use super::*;

#[cfg(test)]
#[test]
fn test_npa_cfdate_adjusment_for_sub_standard1() {
    assert_eq!(
        npa_cfdate_adjusment(
            NaiveDate::from_ymd(2020, 12, 10),
            "Sub-standard".to_string()
        ),
        NaiveDate::from_ymd_opt(2023, 12, 10)
    );
}

#[test]
fn test_npa_cfdate_adjusment_for_sub_standard2() {
    assert_eq!(
        npa_cfdate_adjusment(NaiveDate::from_ymd(2020, 12, 10), "substandard".to_string()),
        NaiveDate::from_ymd_opt(2023, 12, 10)
    );
}

#[test]
fn test_npa_cfdate_adjusment_for_sub_standard3() {
    assert_eq!(
        npa_cfdate_adjusment(NaiveDate::from_ymd(2020, 12, 10), "SUBSTANDARD".to_string()),
        NaiveDate::from_ymd_opt(2023, 12, 10)
    );
}

#[test]
fn test_npa_cfdate_adjusment_for_sub_standard4() {
    assert_eq!(
        npa_cfdate_adjusment(
            NaiveDate::from_ymd(2020, 12, 10),
            "Sub Standard".to_string()
        ),
        NaiveDate::from_ymd_opt(2023, 12, 10)
    );
}

#[test]
fn test_npa_cfdate_adjusment_for_doubtful() {
    assert_eq!(
        npa_cfdate_adjusment(NaiveDate::from_ymd(2020, 12, 10), "Doubtful 1".to_string()),
        NaiveDate::from_ymd_opt(2025, 12, 10)
    );
}

#[test]
fn test_npa_cfdate_adjusment_for_loss_asset1() {
    assert_eq!(
        npa_cfdate_adjusment(NaiveDate::from_ymd(2021, 12, 10), "Loss asset".to_string()),
        NaiveDate::from_ymd_opt(2026, 12, 10)
    );
}

#[test]
fn test_npa_cfdate_adjusment_for_loss_asset2() {
    assert_eq!(
        npa_cfdate_adjusment(NaiveDate::from_ymd(2021, 12, 10), "Loss Asset".to_string()),
        NaiveDate::from_ymd_opt(2026, 12, 10)
    );
}

#[test]
fn test_npa_cfdate_adjusment_for_loss_asset_less_than_91_days() {
    assert_eq!(
        npa_cfdate_adjusment(
            NaiveDate::from_ymd(2021, 12, 10),
            "Loss Assets less than 91 days".to_string()
        ),
        NaiveDate::from_ymd_opt(2026, 12, 10)
    );
}
