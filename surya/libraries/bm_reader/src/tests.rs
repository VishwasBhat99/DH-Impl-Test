#[allow(unused)]
use super::*;

#[test]
fn test_calc_yield_rate() {
    let mut bm: IntermediateBMPoints = Vec::new();
    bm.push(IntermediateBMPoint {
        vertex: 1,
        uom: String::from("M"),
        rate: 5.64,
        days_diff: 30,
        month: 30,
    });
    bm.push(IntermediateBMPoint {
        vertex: 1094,
        uom: String::from("D"),
        rate: 5.30,
        days_diff: 1094,
        month: 36,
    });
    bm.push(IntermediateBMPoint {
        vertex: 1095,
        uom: String::from("D"),
        rate: 5.25,
        days_diff: 1095,
        month: 36,
    });
    assert_eq!(calc_yield_rate(&mut bm, 10, false).unwrap(), 5.64);
    assert_eq!(calc_yield_rate(&mut bm, 1095, false).unwrap(), 5.25);
}
