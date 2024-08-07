use std::ops::{Add, AddAssign, Div, Sub};

use crate::{atop_raw_file::TimestampData, types::UnixTimeStamp};

pub fn mean<T>(values: impl Iterator<Item = T>) -> Result<f64, String>
where
    T: Div<f64, Output = f64> + AddAssign + Default,
{
    let mut count: f64 = 0f64;
    let mut sum: T = Default::default();

    for value in values {
        count += 1f64;
        sum += value;
    }

    if count == 0f64 {
        return Err("Cannot calculate mean of empty list".to_string());
    }

    Ok(sum / count)
}

pub fn median<T>(values: impl Iterator<Item = T>) -> Result<T, String>
where
    T: Ord + Add<Output = T> + Div<u8, Output = T> + Clone,
{
    let values: Vec<T> = values.collect();

    if values.len() == 0 {
        return Err("Cannot calculate median of empty list".to_string());
    }

    if values.len() == 1 {
        return Ok(values[0].clone());
    }

    let mut values_copy = values.to_vec();
    let length = values.len();
    values_copy.sort();

    if length % 2 == 0 {
        let mid1 = values_copy[(length / 2) - 1].clone();
        let mid2 = values_copy[length / 2].clone();

        return Ok((mid1 + mid2) / 2);
    }

    Ok(values_copy[length / 2].clone())
}

pub fn moving_mean<T>(
    values: impl Iterator<Item = T>,
    window_size: usize,
) -> Result<Vec<f64>, String>
where
    T: Div<f64, Output = f64> + std::iter::Sum<T> + Into<f64> + Copy + Default + AddAssign,
{
    let values: Vec<T> = values.collect();

    if window_size <= 0 {
        return Err("Window size should be greater than 0".to_string());
    }

    if window_size > values.len() {
        return Err(format!("Window size should be less than {}", values.len()));
    }

    if window_size <= 1 {
        return Ok(values.iter().map(|&x| x.into()).collect());
    }

    let mut window_mean: Vec<f64> = vec![0.0; values.len() - 2];

    window_mean[0] = mean(values[0..window_size].iter().cloned())?;

    for i in 1..(values.len() - window_size) {
        window_mean[i] = window_mean[i - 1]
            + (values[i + window_size - 1].into() - values[i - 1].into()) / (window_size as f64);
    }

    Ok(window_mean)
}

pub fn exponential_weighted_moving_average<T>(
    values: impl Iterator<Item = T>,
    alpha: f64,
) -> Result<Vec<f64>, String>
where
    T: Into<f64> + Copy,
{
    let values: Vec<T> = values.collect();

    if alpha <= 0.0 || alpha >= 1.0 {
        return Err("Alpha should be between 0 and 1 exclusive".to_string());
    }

    if values.len() == 0 {
        return Err(
            "Cannot calculate exponential weighted moving average of empty list".to_string(),
        );
    }

    let mut ewma: Vec<f64> = vec![0.0; values.len()];

    ewma[0] = values[0].into();

    for i in 1..values.len() {
        ewma[i] = alpha * values[i].into() + (1.0 - alpha) * ewma[i - 1];
    }

    Ok(ewma)
}

pub fn is_above_threshold<T>(value: T, threshold: T) -> bool
where
    T: PartialOrd,
{
    match value.partial_cmp(&threshold) {
        Some(std::cmp::Ordering::Greater) => true,
        _ => false,
    }
}

pub fn differences<T>(values: impl Iterator<Item = T>) -> Result<Vec<f64>, String>
where
    T: Into<f64> + Copy,
{
    let values: Vec<T> = values.collect();

    if values.len() == 0 {
        return Err("Cannot calculate differences of empty list".to_string());
    }

    let mut diffs: Vec<f64> = vec![0.0; values.len() - 1];

    for i in 1..values.len() {
        diffs[i - 1] = values[i].into() - values[i - 1].into();
    }

    Ok(diffs)
}

pub fn divide<S, T>(
    dividend: impl Iterator<Item = S>,
    divisor: impl Iterator<Item = T>,
) -> Result<Vec<f64>, String>
where
    S: Div<T, Output = f64>,
    T: Copy,
{
    let mut result: Vec<f64> = Vec::new();
    let divisor: Vec<T> = divisor.collect();

    for (i, value) in dividend.enumerate() {
        result.push(value / divisor[i]);
    }

    Ok(result)
}

pub fn rate_of_change<T>(
    data: impl Iterator<Item = TimestampData<T>> + Clone,
) -> Result<Vec<f64>, String>
where
    T: Sub<Output = T> + Div<f64, Output = f64> + Copy,
{
    let values: Vec<T> = data.clone().map(|d| d.value).collect();
    let timesteps: Vec<UnixTimeStamp> = data.map(|d| d.timestamp).collect();

    let mut diffs: Vec<T> = Vec::with_capacity(values.len() - 1);
    let mut time_diffs: Vec<f64> = Vec::with_capacity(timesteps.len() - 1);

    for i in 0..timesteps.len() - 1 {
        diffs.push(values[i + 1] - values[i]);
        time_diffs.push((timesteps[i + 1] - timesteps[i]) as f64);
    }

    divide(diffs.into_iter(), time_diffs.into_iter())
}
