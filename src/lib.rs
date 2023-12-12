#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::collections::{HashSet, BTreeMap, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};

/**
 * 计算分位数，比如P25，P50，P75
 * eg: quartile(25, [1.0, 2.0, 3.0, 4.0, 5.0]) => 2.0
 */
pub fn quartile(mut p: i32, numbers: &[f64]) -> f64 {
  if p > 100 {
    p = 100;
  }
  let mut sorted_numbers = numbers.to_vec(); // 克隆一份
  sorted_numbers.sort_by(|a, b| a.partial_cmp(b).unwrap()); // 反向排序，使得最大的数在前
  let index = (p as f64 / 100.0 * sorted_numbers.len() as f64) as usize; // 计算索引
  let mut sum = 0.0; for i in 0..index {
    sum += sorted_numbers[i];
  }
  sum / index as f64 // 计算平均值
}

/**
 * 计算四分位数
 */
pub fn quartiles(numbers: Vec<f64>) -> (f64, f64, f64) {
  let q1 = quartile(25, &numbers);
  let q2 = quartile(50, &numbers);
  let q3 = quartile(75, &numbers);
  (q1, q2, q3)
}

/**
 * 计算方差
 */
pub fn variance(numbers: &[f64]) -> f64 {
  let mean = numbers.iter().sum::<f64>() / numbers.len() as f64;
  let variance = numbers.iter().map(|&num| {
      let deviation = num - mean;
      deviation * deviation  // square of deviation
  }).sum::<f64>() / numbers.len() as f64;
  variance
}

pub fn standard_deviation(numbers: &[f64]) -> f64 {
  let mean = numbers.iter().sum::<f64>() / numbers.len() as f64;
  let variance = numbers.iter().map(|&num| {
      let deviation = num - mean;
      deviation * deviation  // square of deviation
  }).sum::<f64>() / numbers.len() as f64;
  variance.sqrt()
}

/**
 * 计算离群值
 */
#[napi]
pub fn outliers(numbers: Vec<f64>) -> Vec<f64> {
  let q1 = quartile(25, &numbers);
  let q3 = quartile(75, &numbers);
  let iqr = q3 - q1;
  let lower_bound = q1 - 1.5 * iqr;
  let upper_bound = q3 + 1.5 * iqr;
  numbers.into_iter().filter(|&num| num < lower_bound || num > upper_bound).collect()
}

/**
 * 计算变异系数
 */
#[napi]
pub fn coefficient_of_variation(numbers: Vec<f64>) -> f64 {
  let mean = numbers.iter().sum::<f64>() / numbers.len() as f64;
  let variance = numbers.iter().map(|&num| {
      let deviation = num - mean;
      deviation * deviation  // square of deviation
  }).sum::<f64>() / numbers.len() as f64;
  let standard_deviation = variance.sqrt();
  (standard_deviation / mean).abs() * 100.0  // CV as a percentage
}

pub fn linear_counting(numbers: &[i32]) -> f64 {
  let n = numbers.len() as f64;  // total number of elements
  let m = numbers.iter().collect::<HashSet<_>>().len() as f64;  // number of unique elements
  n * (-((1.0 - m / n).ln()))  // linear counting estimate
}

pub fn group_by_range(numbers: &[f64], range: f64) -> BTreeMap<i32, Vec<f64>> {
  let mut groups: BTreeMap<i32, Vec<f64>> = BTreeMap::new();
  for &num in numbers {
      let key = (num / range).floor() as i32;
      groups.entry(key).or_default().push(num);
  }
  groups
}

pub fn probabilistic_counting(numbers: &[i32]) -> f64 {
  let mut bitmap: u64 = 0;
  for &num in numbers {
      let mut hasher = DefaultHasher::new();
      num.hash(&mut hasher);
      let hash = hasher.finish();
      let bit_index = hash.trailing_zeros() as u64;
      bitmap |= 1 << bit_index;
  }
  let m = bitmap.count_ones() as f64;
  2f64.powf(m) / 0.77351  // probabilistic counting estimate
}