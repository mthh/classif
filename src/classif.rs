use std::str::FromStr;
use num_traits::{Float, NumAssignOps};

use stats::mean;
use jenks::get_jenks_breaks;

#[derive(PartialEq, Debug)]
pub enum Classification {
    EqualInterval,
    HeadTail,
    TailHead,
    JenksNaturalBreaks,
    Quantiles,
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
            _ => Err("Invalid classification name"),
        }
    }
}

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

    pub fn get_class_index(&self, value: T) -> Option<u32> {
        for i in 0..self.bounds.len() - 1 {
            if value <= self.bounds[i + 1usize] {
                return Some(i as u32);
            }
        }
        None
    }
}


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
