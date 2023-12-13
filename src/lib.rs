#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::collections::{HashSet, BTreeMap, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};

use napi::bindgen_prelude::{Float64Array, Int32Array};

/**
 * 求和
 */
#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

/**
 * 求余
 */
#[napi]
pub fn remainder(a: i32, b: i32) -> i32 {
  a % b
}

/**
 * 减法
 */
#[napi]
pub fn subtract(a: i32, b: i32) -> i32 {
  a - b
}

/**
 * 乘法
 */
#[napi]
pub fn multiply(a: i32, b: i32) -> i32 {
  a * b
}

/**
 * 除法
 */
#[napi]
pub fn divide(a: i32, b: i32) -> i32 {
  a / b
}

/**
 * 求平均值
 */
#[napi]
pub fn mean(numbers: Float64Array) -> f64 {
  numbers.iter().sum::<f64>() / numbers.len() as f64
}

/**
 * 求中位数
 */
#[napi]
pub fn median(numbers: Float64Array) -> f64 {
  let mut sorted_numbers = numbers;
  sorted_numbers.sort_by(|a, b| a.partial_cmp(b).unwrap()); // 排序，使得最小的数在前
  let mid = sorted_numbers.len() / 2;
  if sorted_numbers.len() % 2 == 0 {
    (sorted_numbers[mid] + sorted_numbers[mid - 1]) / 2.0
  } else {
    sorted_numbers[mid]
  }
}

/**
 * 求众数
 */
#[napi]
pub fn mode(numbers: Int32Array) -> i32 {
  let mut counts = BTreeMap::new();
  for &num in numbers[0..].iter() {
    *counts.entry(num).or_insert(0) += 1;
  }
  let mut max = 0;
  let mut mode = 0;
  for (&num, &count) in counts.iter() {
    if count > max {
      max = count;
      mode = num;
    }
  }
  mode
}

/**
 * 求最大值
 */
#[napi(js_name = "max")]
pub fn max_js(numbers: Float64Array) -> f64 {
  max(&numbers)
}
pub fn max(numbers: &Float64Array) -> f64 {
  let mut max = numbers[0];
  for &num in numbers[0..].iter() {
    if num > max {
      max = num;
    }
  }
  max
}

/**
 * 求最小值
 */
#[napi(js_name = "min")]
pub fn min_js(numbers: Float64Array) -> f64 {
  min(&numbers)
}

pub fn min(numbers: &Float64Array) -> f64 {
  let mut min = numbers[0];
  for &num in numbers[0..].iter() {
    if num < min {
      min = num;
    }
  }
  min
}

/**
 * 求极差
 */
#[napi(js_name = "range")]
pub fn range_js(numbers: Float64Array) -> f64 {
  range(&numbers)
}

pub fn range(numbers: &Float64Array) -> f64 {
  max(&numbers) - min(&numbers)
}

/**
 * 求四分位距
 */
#[napi]
pub fn interquartile_range(numbers: Float64Array) -> f64 {
  let mut sorted_numbers = numbers;
  sorted_numbers.sort_by(|a, b| a.partial_cmp(b).unwrap()); // 排序，使得最小的数在前
  let q1 = quartile_sorted(25, &sorted_numbers);
  let q3 = quartile_sorted(75, &sorted_numbers);
  q3 - q1
}

/**
 * 斐波那契数列
 */
#[napi]
pub fn fibonacci(n: u32) -> u32 {
  match n {
    1 | 2 => 1,
    _ => fibonacci(n - 1) + fibonacci(n - 2),
  }
}


fn quartile_sorted(mut p: i32, sorted_numbers: &Float64Array) -> f64 {
  if p > 100 {
    p = 100;
  }
  let index = (p as f64 / 100.0 * sorted_numbers.len() as f64) as usize; // 计算索引
  let mut sum = 0.0;
  for i in 0..index {
    sum += sorted_numbers[i];
  }
  sum / index as f64 // 计算平均值
}

/**
 * 计算分位数，比如P25，P50，P75
 * eg: quartile(25, [1.0, 2.0, 3.0, 4.0, 5.0]) => 2.0
 */
#[napi]
pub fn quartile(p: i32, numbers: Float64Array) -> f64 {
  let mut sorted_numbers = numbers;
  sorted_numbers.sort_by(|a, b| a.partial_cmp(b).unwrap()); // 排序，使得最小的数在前
  quartile_sorted(p, &sorted_numbers)
}

/**
 * 计算四分位数
 */
#[napi]
pub fn quartiles(numbers: Float64Array) -> [f64; 3] {
  let mut sorted_numbers = numbers.clone();
  sorted_numbers.sort_by(|a, b| a.partial_cmp(b).unwrap()); // 排序，使得最小的数在前
  let q1 = quartile_sorted(25, &sorted_numbers);
  let q2 = quartile_sorted(50, &sorted_numbers);
  let q3 = quartile_sorted(75, &sorted_numbers);
  [q1, q2, q3]
}

/**
 * 计算方差
 */
#[napi]
pub fn variance(numbers: Float64Array) -> f64 {
  let mean = numbers.iter().sum::<f64>() / numbers.len() as f64;
  let variance = numbers.iter().map(|&num| {
      let deviation = num - mean;
      deviation * deviation  // square of deviation
  }).sum::<f64>() / numbers.len() as f64;
  variance
}

/** 
 * 计算标准差
 */
#[napi]
pub fn standard_deviation(numbers: Float64Array) -> f64 {
  variance(numbers).sqrt()
}

/**
 * 计算离群值
 */
#[napi]
pub fn outliers(numbers: Float64Array) -> Vec<f64> {
  let mut sorted_numbers = numbers.clone();
  sorted_numbers.sort_by(|a, b| a.partial_cmp(b).unwrap()); // 排序，使得最小的数在前
  let q1 = quartile_sorted(25, &sorted_numbers);
  let q3 = quartile_sorted(75, &sorted_numbers);
  let iqr = q3 - q1;
  let lower_bound = q1 - 1.5 * iqr;
  let upper_bound = q3 + 1.5 * iqr;
  sorted_numbers.to_vec().into_iter().filter(|&num| num < lower_bound || num > upper_bound).collect()
}

/**
 * 计算变异系数
 */
#[napi]
pub fn coefficient_of_variation(numbers: Float64Array) -> f64 {
  let mean = numbers.iter().sum::<f64>() / numbers.len() as f64;
  let variance = numbers.iter().map(|&num| {
      let deviation = num - mean;
      deviation * deviation  // square of deviation
  }).sum::<f64>() / numbers.len() as f64;
  let standard_deviation = variance.sqrt();
  (standard_deviation / mean).abs() * 100.0  // CV as a percentage
}

/**
 * 线性计数
 */
#[napi]
pub fn linear_counting(numbers: Int32Array) -> f64 {
  let n = numbers.len() as f64;  // total number of elements
  let m: f64 = numbers.iter().collect::<HashSet<_>>().len() as f64;  // number of unique elements
  n * (-((1.0 - m / n).ln()))  // linear counting estimate
}

/**
 * 数据按范围分组
 */
pub fn group_by_range(numbers: Float64Array, range: f64) -> BTreeMap<i32, Vec<f64>> {
  let mut groups: BTreeMap<i32, Vec<f64>> = BTreeMap::new();
  for &num in numbers[0..].iter() {
      let key = (num / range).floor() as i32;
      groups.entry(key).or_default().push(num);
  }
  groups
}

/**
 * 概率计数
 */
#[napi]
pub fn probabilistic_counting(numbers: Int32Array) -> f64 {
  let mut bitmap: u64 = 0;
  for &num in numbers[0..].iter() {
      let mut hasher = DefaultHasher::new();
      num.hash(&mut hasher);
      let hash = hasher.finish();
      let bit_index = hash.trailing_zeros() as u64;
      bitmap |= 1 << bit_index;
  }
  let m = bitmap.count_ones() as f64;
  2f64.powf(m) / 0.77351  // probabilistic counting estimate
}
