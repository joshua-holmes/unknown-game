use std::ops::Range;

#[cfg(not(test))]
use std::time::{SystemTime, UNIX_EPOCH};

pub fn rand_f64(range: Range<f64>) -> f64 {
    let rand_0_to_1 = (now_in_nanos() % 1_000_000) as f64 / 1_000_000.;
    let diff = range.end - range.start;
    rand_0_to_1 * diff + range.start
}

#[cfg(not(test))]
fn now_in_nanos() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Went backwards in time")
        .as_nanos()
}
#[cfg(test)]
fn now_in_nanos() -> u128 {
    490_000
}

#[cfg(test)]
mod tests {
    use super::rand_f64;

    #[test]
    fn test_rand_f64() {
        let num: f64 = rand_f64((0.)..1.);
        assert_eq!(num, 0.49);
    }
}
