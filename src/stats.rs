use num_traits::{Float, NumAssignOps};

pub fn mean<T>(values: &[T]) -> T
    where T: Float + NumAssignOps
{
    let mut sum: T = T::zero();
    values.iter().map(|v| sum += *v).collect::<Vec<_>>();
    sum / T::from(values.len()).unwrap()
}

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

pub fn variance<T>(values: &[T]) -> T
    where T: Float + NumAssignOps
{
    sum_pow_deviations(values, 2) / T::from(values.len()).unwrap()
}
