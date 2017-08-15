use num_traits::{Float, NumAssignOps};

/// Compute the mean of a list of values.
pub fn mean<T>(values: &[T]) -> T
    where T: Float + NumAssignOps
{
    let mut sum: T = T::zero();
    values.iter().map(|v| sum += *v).collect::<Vec<_>>();
    sum / T::from(values.len()).unwrap()
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
