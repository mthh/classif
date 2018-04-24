//! Library for **one-dimensional data classification** and **simple statistics**,
//! especially methods used for cartographic purposes such as "quantiles breaks"
//! or "Jenks natural breaks".
//!
//! This library allows to computed break values according to a few methods (listed in
//! the [`Classification`] Enum).
//! Theses break values can be computed by creating a new [`BoundsInfo`] struct.
//!
//! If the input values are already sorted and you don't care about error checking
//! you can directly use the `get_{jenks|quantiles|etc...}_breaks` functions.
//!
//! This library also provide a few basic statistical functionnalities, allowing to
//! compute mean value, kurtosis value, standard deviation, variance, root of mean square, etc.
//!
//! [`Classification`]: enum.Classification.html
//! [`BoundsInfo`]: struct.BoundsInfo.html
#[macro_use]
extern crate assert_approx_eq;
extern crate num_traits;
extern crate failure;
#[macro_use] extern crate failure_derive;

/// Basic statistical functionnalities: mean, standard deviation, kurtosis, variance, etc.
pub mod stats;

mod jenks;
mod classif;

pub use classif::{Classification, BoundsInfo};
pub use jenks::get_jenks_breaks;
pub use classif::{get_quantiles, get_equal_interval, get_head_tail_breaks, get_tail_head_breaks,
                  get_arithmetic_breaks};


mod error {
    use std::{self, fmt};
    #[derive(Fail, Debug)]
    pub enum ClassifError {
      #[fail(display = "{} requires only positive numbers as input", _0)]
      OnlyPositive(MayFail),
      #[fail(display = "An unknown error has occurred.")]
      UnknownError,
    }
    #[derive(Debug)]
    pub enum MayFail {
        HarmonicMean,
        GeometricMean,
    }
    impl std::fmt::Display for MayFail {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", format!("{:?}", self))
        }
    }
    pub type ClassifResult<T> = std::result::Result<T, ClassifError>;
}

#[cfg(test)]
mod tests {
    use ::*;
    use num_traits::Float;
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

    #[test]
    fn test_kurtosis() {
        let values = get_test_values();
        let kv = stats::kurtosis(&values);
        assert_eq!(kv, 0.042107329018970074);
    }

    #[test]
    fn test_variance() {
        let values = get_test_values();
        let r = stats::sum_pow_deviations(&values, 2);
        assert_eq!(r, 608.631578947369);
        let v = stats::variance(&values);
        assert_eq!(v, 8.008310249307486);
    }

    #[test]
    fn test_root_mean_square() {
        let values = [-1., 1., -1., 1.];
        let v = stats::rootmeansquare(&values);
        assert_eq!(v, 1.);
    }

    #[test]
    fn test_median() {
        // Number of values is odd:
        let values = [1., 3., 3., 6., 7., 8., 9.];
        let median = stats::median(&values);
        assert_eq!(median, 6.);
        // Number of values is even:
        let values = [1., 2., 3., 4., 5., 6., 8., 9.];
        let median = stats::median(&values);
        assert_eq!(median, 4.5);
    }

    #[test]
    fn test_harmonic_mean() {
        let values = [2., 3.];
        let v = stats::harmonic_mean(&values).unwrap();
        assert_approx_eq!(v, 2.4);
    }

    #[test]
    fn test_geometric_mean() {
        let values = [1., 8., 9., 7., 6., 8., 19., 32.];
        let res = stats::geometric_mean(&values).unwrap();
        assert_eq!(res, 7.869496003150113);
    }
}
