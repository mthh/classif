use num_traits::{Float, NumAssignOps};
use error::{ClassifError, ClassifResult, MayFail};

/// Compute the mean of a list of values.
pub fn mean<T>(values: &[T]) -> T
    where T: Float + NumAssignOps
{
    let mut sum: T = T::zero();
    values.iter().map(|v| sum += *v).collect::<Vec<_>>();
    sum / T::from(values.len()).unwrap()
}

/// Compute the median value, ie. the middle number of a list a value,
/// ie. the value corresponding to the 0.5 quantile.
pub fn median<T>(values: &[T]) -> T
    where T: Float
{
    let mut v = values.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = values.len();
    let m = (n as f64 / 2.).ceil() as usize;
    if n % 2 != 0 {
        v[m - 1usize]
    } else {
        (v[m - 1usize] + v[m]) / T::from(2.0).unwrap()
    }
}

/// Compute the kurtosis value of list of values.
/// The implementation is based on Fischer's definition (normal ==> 0.0)
/// and use unbiased estimators.
pub fn kurtosis<T>(values: &[T]) -> T
    where T: Float + NumAssignOps
{
    let nb_elem = values.len();
    let mean = mean(&values);
    let mut temp_value;
    let mut second_central_moment = T::zero();
    let mut fourth_central_moment = T::zero();
    for i in 0..nb_elem {
        temp_value = values[i] - mean;
        second_central_moment += temp_value * temp_value;
        fourth_central_moment += temp_value * temp_value * temp_value * temp_value;
    }
    let n = T::from(nb_elem).unwrap();
    (n - T::from(1).unwrap()) / ((n - T::from(2).unwrap()) * (n - T::from(3).unwrap())) *
    (n * (n + T::from(1).unwrap()) * fourth_central_moment /
     (second_central_moment * second_central_moment) -
     T::from(3).unwrap() * (n - T::from(1).unwrap()))
}

/// Compute the sum of deviations to the Nth power.
/// (i.e. sum of squared deviations when n=2, sum of cubed deviations when n=3, etc.)
pub fn sum_pow_deviations<T>(values: &[T], n: i32) -> T
    where T: Float + NumAssignOps
{
    let mean = mean(&values);
    let nb_elem = values.len();
    let mut sum = T::zero();
    for i in 0..nb_elem {
        sum += (values[i] - mean).powi(n);
    }
    sum
}

/// Compute the variance of a list of values.
/// The variance is the sum of squared deviations from the mean.
pub fn variance<T>(values: &[T]) -> T
    where T: Float + NumAssignOps
{
    sum_pow_deviations(values, 2) / T::from(values.len()).unwrap()
}

/// Compute the standard deviation of a list of values.
pub fn standard_deviation<T>(values: &[T]) -> T
    where T: Float + NumAssignOps
{
    T::sqrt(variance(&values))
}

/// Compute the root mean square of list of values.
pub fn rootmeansquare<T>(values: &[T]) -> T
    where T: Float + NumAssignOps
{
    let sum: T = values
        .iter()
        .fold(T::zero(), |mut s, v| {
            s += v.powi(2);
            s
        });
    (sum / T::from(values.len()).unwrap()).sqrt()
}

/// This mean is calculated by taking the reciprocal of the arithmetic mean
/// of the reciprocals of the input numbers.
///
/// ```
/// # #[macro_use]
/// # extern crate assert_approx_eq;
/// # extern crate num_traits;
/// # extern crate classif;
/// #
/// # use num_traits::float::Float;
/// # use classif::stats;
/// #
/// # fn main() {
/// let values = [2., 3.];
/// let res = stats::harmonic_mean(&values).unwrap();
/// // -> 2.4
/// # assert_approx_eq!(res, 2.4, 0.00001);
/// # }
/// ```
pub fn harmonic_mean<T>(values: &[T]) -> ClassifResult<T>
    where T: Float + NumAssignOps
{
    let mut reciprocal_sum = T::zero();
    for v in values {
        if *v <= T::zero() {
            return Err(ClassifError::OnlyPositive(MayFail::HarmonicMean));
        }
        reciprocal_sum += T::from(1.0).unwrap() / *v
    }
    Ok(T::from(values.len()).unwrap() / reciprocal_sum)
}

/// Compute the central number in a geometric progression,
/// also calculable as the nth root of a product of n numbers.
///
/// ```
/// # #[macro_use]
/// # extern crate assert_approx_eq;
/// # extern crate num_traits;
/// # extern crate classif;
/// #
/// # use num_traits::float::Float;
/// # use classif::stats;
/// #
/// # fn main() {
/// let values = [3., 9., 27.];
/// let res = stats::geometric_mean(&values).unwrap();
/// // -> 9.
/// # assert_approx_eq!(res, 9., 0.00001);
/// # }
/// ```
pub fn geometric_mean<T>(values: &[T]) -> ClassifResult<T>
    where T: Float + NumAssignOps
{
    let mut val = T::one();
    for v in values {
        if *v <= T::zero() {
            return Err(ClassifError::OnlyPositive(MayFail::GeometricMean));
        }
        val *= *v;
    }
    Ok(val.powf(T::one() / T::from(values.len()).unwrap()))
}
