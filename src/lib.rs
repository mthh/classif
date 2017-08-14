extern crate num_traits;

pub mod stats;
mod jenks;
mod classif;
pub use classif::{Classification, BoundsInfo};
pub use jenks::get_jenks_breaks;
pub use classif::{get_quantiles, get_equal_interval, get_head_tail_breaks, get_arithmetic_breaks};

#[cfg(test)]
mod tests {
    use ::*;
    fn get_test_values() -> [f64; 76] {
        [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0,
         3.0, 3.0, 3.0, 2.0, 2.0, 2.0, 2.0, 1.0, 1.0, 12.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0,
         10.0, 11.0, 5.0, 6.0, 7.0, 6.0, 5.0, 6.0, 7.0, 8.0, 8.0, 9.0, 8.0, 7.0, 6.0, 7.0, 8.0,
         9.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 3.0,
         2.0, 2.0, 2.0, 1.0, 1.0, 1.0]
    }
    #[test]
    fn test_head_tail() {
        let mut values = get_test_values();
        let b = BoundsInfo::new(4, &values, Classification::HeadTail).unwrap();
        assert_eq!(b.bounds.as_slice(), [1., 7., 9.090909090909092, 11., 12.]);
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let breaks = get_head_tail_breaks(&mut values);
        assert_eq!(breaks.as_slice(), [1., 7., 9.090909090909092, 11., 12.]);
    }

    #[test]
    fn test_jenks_breaks() {
        let mut values = get_test_values();
        let b = BoundsInfo::new(5, &values, Classification::JenksNaturalBreaks).unwrap();
        assert_eq!(b.bounds.as_slice(), [1.0, 2.0, 4.0, 7.0, 9.0, 12.0]);
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let breaks = get_jenks_breaks(&values, 5);
        assert_eq!(breaks.as_slice(), [1.0, 2.0, 4.0, 7.0, 9.0, 12.0]);
    }

    #[test]
    fn test_quantiles_breaks() {
        let mut values = get_test_values();
        let b = BoundsInfo::new(4, &values, Classification::Quantiles).unwrap();
        assert_eq!(b.bounds.as_slice(), [1., 2., 3., 6., 12.]);
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let breaks = get_quantiles(&values, 4);
        assert_eq!(breaks.as_slice(), [1., 2., 3., 6., 12.]);
    }

    #[test]
    fn test_equal_interval_breaks() {
        let mut values = get_test_values();
        let b = BoundsInfo::new(4, &values, Classification::EqualInterval).unwrap();
        assert_eq!(b.bounds.as_slice(), [1., 3.75, 6.5, 9.25, 12.]);
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let breaks = get_equal_interval(&values, 4);
        assert_eq!(breaks.as_slice(), [1., 3.75, 6.5, 9.25, 12.]);
    }

    #[test]
    fn test_arithmetic_breaks() {
        let mut values = get_test_values();
        let b = BoundsInfo::new(6, &values, Classification::Arithmetic).unwrap();
        assert_eq!(b.bounds.as_slice(),
                   [1.,
                    1.5238095238095237,
                    2.571428571428571,
                    4.142857142857142,
                    6.238095238095237,
                    8.857142857142856,
                    12.]);
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let breaks = get_arithmetic_breaks(&values, 6);
        assert_eq!(breaks.as_slice(),
                   [1.,
                    1.5238095238095237,
                    2.571428571428571,
                    4.142857142857142,
                    6.238095238095237,
                    8.857142857142856,
                    12.]);

    }

    #[test]
    fn test_kurtosis() {
        let values = get_test_values();
        let kv = stats::kurtosis(&values);
        assert_eq!(kv, 0.042107329018970074);
    }

    #[test]
    fn test_deviations() {
        let values = get_test_values();
        let r = stats::sum_pow_deviations(&values, 2);
        assert_eq!(r, 608.631578947369);
        let v = stats::variance(&values);
        assert_eq!(v, 8.008310249307486);
    }

    #[test]
    fn test_get_class_index() {
        let values = get_test_values();
        let b = BoundsInfo::new(4, &values, Classification::EqualInterval).unwrap();
        assert_eq!(b.bounds.as_slice(), [1., 3.75, 6.5, 9.25, 12.]);
        // 0.1 is below the minimum so it belongs in no class, None is expected:
        assert_eq!(b.get_class_index(0.1), None);
        // 2.0 is between 1.0 and 3.75 so 2.0 should belong in the first class, with index 0:
        assert_eq!(b.get_class_index(2.), Some(0));
        // And so on...
        assert_eq!(b.get_class_index(4.), Some(1));
        assert_eq!(b.get_class_index(7.), Some(2));
        assert_eq!(b.get_class_index(10.), Some(3));
        // 15.0 is larger than the maximum so it belongs in no class, None is expected:
        assert_eq!(b.get_class_index(15.0), None);
    }
}
