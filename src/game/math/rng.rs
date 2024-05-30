use std::ops::Range;

#[cfg(not(test))]
use std::time::{SystemTime, UNIX_EPOCH};

pub fn rand_f64(range: Range<f64>) -> f64 {
    let rand_0_to_1 = (now_in_micros() % 1_000) as f64 / 1_000.;
    let diff = range.end - range.start;
    rand_0_to_1 * diff + range.start
}

pub fn rand_bool() -> bool {
    (now_in_micros() / 1_000) % 2 == 0
}

#[cfg(not(test))]
fn now_in_micros() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Went backwards in time")
        .as_micros()
}
#[cfg(test)]
fn now_in_micros() -> u128 {
    1_710_974_626_375_231
}

#[cfg(test)]
mod tests {
    use super::{rand_bool, rand_f64};

    #[test]
    fn test_rand_f64() {
        let num: f64 = rand_f64((0.)..1.);
        assert_eq!(num, 0.231);
    }

    #[test]
    fn test_rand_bool() {
        let b: bool = rand_bool();
        assert_eq!(b, false);
    }
}
