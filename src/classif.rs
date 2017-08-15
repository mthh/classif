use std::str::FromStr;
use num_traits::{Float, NumAssignOps};

use stats::mean;
use jenks::get_jenks_breaks;

#[derive(PartialEq, Debug)]
/// The various type of classification methods availables.
pub enum Classification {
    EqualInterval,
    HeadTail,
    TailHead,
    JenksNaturalBreaks,
    Quantiles,
    Arithmetic,
}

impl FromStr for Classification {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "JenksNaturalBreaks" => Ok(Classification::JenksNaturalBreaks),
            "Quantiles" => Ok(Classification::Quantiles),
            "EqualInverval" => Ok(Classification::EqualInterval),
            "HeadTail" => Ok(Classification::HeadTail),
            "TailHead" => Ok(Classification::TailHead),
            "Arithmetic" => Ok(Classification::Arithmetic),
            _ => Err("Invalid classification name"),
        }
    }
}

/// A struct containing the bounds computed at its creation and some basic
/// statistical informations : minimum, maximum and mean value.
///
/// The main field of `BoundsInfo` struct is the `bounds` field: a `Vec<T>`
/// containing the bounds. The lower bound is the minimum of the input values
/// and the upper bound is the maximum.
///
/// The `get_class_index` method allows to retrieve the index of the cluster in which
/// belongs an input value.
///
/// ### Example :
///
/// ```rust,ignore
/// let values = vec![1.0, 1.3, 2.4, 5.0, 2.1, 5.3, 4.0, 3.0, 1.3, 4.3, 6.0, 2.1];
/// let bounds_info = BoundsInfo::new(4, values, Classification::EqualInterval).unwrap();
/// // The first bounds value is the minimum:
/// assert_almost_equal(bounds_info.bounds[0], bounds_info.min);
/// // And the last bounds value is the maximum:
/// assert_almost_equal(bounds_info.bounds.last().unwrap(), bounds_info.max);
/// // So for 4 class we need a vector of 5 values :
/// assert_eq!(bounds_info.bounds.len(), 5);
/// // In which class belong the value 4.4 ?
/// let ix = bounds_info.get_class_index(4.4).unwrap();
/// ```
pub struct BoundsInfo<T> {
    pub type_classif: Classification,
    pub nb_class: u32,
    pub bounds: Vec<T>,
    pub min: T,
    pub max: T,
    pub mean: T,
}

impl<T> BoundsInfo<T>
    where T: Float + NumAssignOps
{
    pub fn new(nb_class: u32,
               values: &[T],
               type_classif: Classification)
               -> Result<Self, &'static str> {
        let nb_elem = values.len();
        if nb_elem < 2 {
            return Err("Too small number of values!".into());
        } else if !(type_classif == Classification::HeadTail ||
                    type_classif == Classification::TailHead) &&
                  (nb_class < 2 || nb_class > nb_elem as u32) {
            return Err("Invalid number of class");
        }
        let mut v = values.to_vec();
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let breaks = match type_classif {
            Classification::JenksNaturalBreaks => get_jenks_breaks(&v, nb_class),
            Classification::Quantiles => get_quantiles(&v, nb_class),
            Classification::EqualInterval => get_equal_interval(&v, nb_class),
            Classification::HeadTail => get_head_tail_breaks(&v),
            Classification::TailHead => get_tail_head_breaks(&v),
            Classification::Arithmetic => get_arithmetic_breaks(&v, nb_class),
        };
        Ok(BoundsInfo {
               type_classif: type_classif,
               nb_class: (breaks.len() - 1) as u32,
               bounds: breaks,
               min: v[0],
               max: v[v.len() - 1],
               mean: mean(&v),
           })
    }

    /// Returns the index of the class to which the `value` belongs, wrapped
    /// in an Option. Returns None if the value is outside the serie range.
    pub fn get_class_index(&self, value: T) -> Option<u32> {
        for i in 0..self.bounds.len() - 1 {
            if value <= self.bounds[i + 1usize] && value > self.bounds[i] {
                return Some(i as u32);
            }
        }
        None
    }
}

/// Compute the equal interval breaks on a list of sorted values.
pub fn get_equal_interval<T>(sorted_values: &[T], nb_class: u32) -> Vec<T>
    where T: Float + NumAssignOps
{
    // values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    // let nb_elem = values.len();
    let min = sorted_values.first().unwrap();
    let max = sorted_values.last().unwrap();
    let interval = (*max - *min) / T::from(nb_class).unwrap();
    let mut breaks = Vec::new();
    let mut val = *min;
    for _ in 0..(nb_class + 1) {
        breaks.push(val);
        val += interval;
    }
    {
        let last = breaks.last_mut().unwrap();
        *last = *max;
    }
    breaks
}

/// Compute the quantiles breaks on a list of sorted values.
pub fn get_quantiles<T>(sorted_values: &[T], nb_class: u32) -> Vec<T>
    where T: Float
{
    let nb_elem: usize = sorted_values.len();
    let mut breaks = Vec::new();
    breaks.push(sorted_values[0]);
    let step = nb_elem as f64 / nb_class as f64;
    for i in 1..nb_class {
        let qidx = (i as f64 * step + 0.49).floor() as usize;
        breaks.push(sorted_values[qidx - 1]);
    }
    breaks.push(sorted_values[nb_elem - 1]);
    breaks
}

/// Compute the "Head-Tail" breaks on a list of sorted values
/// (to be used on heavily right skewed distributions).
pub fn get_head_tail_breaks<T>(sorted_values: &[T]) -> Vec<T>
    where T: Float + NumAssignOps
{
    let mut _mean = mean(&sorted_values);
    let mut breaks = Vec::new();
    let mut t;
    breaks.push(sorted_values[0]);
    loop {
        t = sorted_values
            .iter()
            .filter(|&v| *v > _mean)
            .cloned()
            .collect::<Vec<T>>();
        _mean = mean(&t);
        breaks.push(_mean);
        if t.len() < 2 {
            break;
        }
    }
    breaks
}

/// Compute the "Tail-Head" breaks on a list of sorted values.
/// (its actually just the inverse of the Head-Tail method,
/// to be used on heavily left skewed distributions).
pub fn get_tail_head_breaks<T>(sorted_values: &[T]) -> Vec<T>
    where T: Float + NumAssignOps
{
    let mut _mean = mean(&sorted_values);
    let mut breaks = Vec::new();
    let mut t;
    breaks.push(*sorted_values.last().unwrap());
    loop {
        t = sorted_values
            .iter()
            .filter(|&v| *v < _mean)
            .cloned()
            .collect::<Vec<T>>();
        _mean = mean(&t);
        breaks.push(_mean);
        if t.len() < 2 {
            break;
        }
    }
    breaks.reverse();
    breaks
}

/// Compute the "arithmetic progression" breaks on a list of sorted values.
pub fn get_arithmetic_breaks<T>(sorted_values: &[T], nb_class: u32) -> Vec<T>
    where T: Float + NumAssignOps
{
    let mut denominator = T::zero();
    for i in 1..nb_class + 1 {
        denominator += T::from(i).unwrap();
    }
    let mut breaks = Vec::new();
    let tmp_min = sorted_values[0];
    let tmp_max = sorted_values[sorted_values.len() - 1];
    let interval = (tmp_max - tmp_min) / denominator;
    breaks.push(tmp_min);
    for i in 1..nb_class + 1 {
        let v = breaks[(i - 1) as usize];
        breaks.push(v + (T::from(i).unwrap() * interval));
    }
    breaks
}
