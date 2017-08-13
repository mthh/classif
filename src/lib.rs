extern crate num_traits;

mod jenks;
mod classif;
pub use classif::{Classification, BoundsInfo};
pub use jenks::get_breaks as get_jenks_breaks;
pub use classif::{get_quantiles, get_equal_interval, get_head_tail_breaks};

#[cfg(test)]
mod tests {
    use ::*;

    #[test]
    fn test_head_tail() {
        let mut values = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 3.0,
                              3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 2.0, 2.0, 2.0, 2.0, 1.0,
                              1.0, 12.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 5.0, 6.0,
                              7.0, 6.0, 5.0, 6.0, 7.0, 8.0, 8.0, 9.0, 8.0, 7.0, 6.0, 7.0, 8.0,
                              9.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 3.0, 3.0,
                              3.0, 4.0, 4.0, 4.0, 3.0, 2.0, 2.0, 2.0, 1.0, 1.0, 1.0];
        let breaks = get_head_tail_breaks(&mut values);
        assert_eq!(breaks.as_slice(), [1., 7., 9.090909090909092, 11., 12.]);
    }
}
