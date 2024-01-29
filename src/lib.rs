#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::collections::{HashSet, BTreeMap, hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};
use napi::{
  bindgen_prelude::*,
  threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode, ThreadSafeCallContext},
};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

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

/**
 * 计算皮尔逊相关系数
 */
#[napi]
pub fn pearson(x: Float64Array, y: Float64Array) -> f64 {
  let n = x.len() as f64;
  let mut sum_x = 0.0;
  let mut sum_y = 0.0;
  let mut sum_xy = 0.0;
  let mut sum_x2 = 0.0;
  let mut sum_y2 = 0.0;
  for i in 0..x.len() {
      sum_x += x[i];
      sum_y += y[i];
      sum_xy += x[i] * y[i];
      sum_x2 += x[i] * x[i];
      sum_y2 += y[i] * y[i];
  }
  let molecule = sum_xy - (sum_x * sum_y) / n;
  let denominator = ((sum_x2 - (sum_x * sum_x) / n) * (sum_y2 - (sum_y * sum_y) / n)).sqrt();
  molecule / denominator
}

/**
 * 计算斯皮尔曼等级相关系数
 */
#[napi]
pub fn spearman_rank_correlation(x: Float64Array, y: Float64Array) -> f64 {
  assert_eq!(x.len(), y.len());
  let n = x.len() as f64;

  let rank_x = rank(&x);
  let rank_y = rank(&y);

  let diff: Vec<_> = rank_x.iter().zip(rank_y.iter()).map(|(a, b)| a - b).collect();
  let diff_sq_sum: f64 = diff.iter().map(|d| d * d).sum();

  1.0 - 6.0 * diff_sq_sum / (n * (n * n - 1.0))
}

fn rank(data: &[f64]) -> Vec<f64> {
  let mut rank_data: Vec<_> = data.iter().enumerate().collect();
  rank_data.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

  let mut rank = vec![0.0; data.len()];
  for (i, &(idx, _)) in rank_data.iter().enumerate() {
      rank[idx] = i as f64 + 1.0;
  }
  rank
}

/**
 * 计算肯德尔等级相关系数
 */
#[napi]
pub fn kendall_tau(x: Float64Array, y: Float64Array) -> f64 {
  assert_eq!(x.len(), y.len());
  let n = x.len();

  let mut concordant = 0;
  let mut discordant = 0;

  for i in 0..n {
      for j in (i+1)..n {
          let xi_xj = x[i] - x[j];
          let yi_yj = y[i] - y[j];

          let pair = xi_xj * yi_yj;

          if pair > 0.0 {
              concordant += 1;
          } else if pair < 0.0 {
              discordant += 1;
          }
      }
  }

  (concordant as f64 - discordant as f64) / ((n * (n - 1)) / 2) as f64
}

/**
 * 计算点双序列相关系数
 */
pub fn point_biserial_correlation(x: &[f64], y: &[bool]) -> f64 {
  assert_eq!(x.len(), y.len());
  let n = x.len() as f64;

  let mean_x = x.iter().sum::<f64>() / n;
  let mean_y = y.iter().map(|&v| if v {1.0} else {0.0}).sum::<f64>() / n;

  let mut sum_x = 0.0;
  let mut sum_y = 0.0;

  for i in 0..x.len() {
      let xi = x[i];
      let yi = if y[i] {1.0} else {0.0};

      sum_x += (xi - mean_x).powi(2);
      sum_y += (yi - mean_y) * (xi - mean_x);
  }

  sum_y / ((n - 1.0) * sum_x.sqrt())
}

#[napi]
#[allow(dead_code)]
struct Report {
  task: tokio::runtime::Runtime,
  tree: Arc<Mutex<BTreeMap<String, BTreeMap<String, u32>>>>,
}

#[napi]
impl Report {
  #[napi(constructor)]
  #[allow(dead_code)]
  pub fn new() -> Self {
    Report {
      task: tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap(),
      tree: Arc::new(Mutex::new(BTreeMap::new())),
    }
  }

  #[napi]
  #[allow(dead_code)]
  pub fn incr(&self, code: String, key: String) {
    let tree = Arc::clone(&self.tree);
    self.task.spawn(async move {
      let mut tree = tree.lock().unwrap();
      let code_map = tree.entry(code).or_default();
      code_map.entry(key).and_modify(|v| *v += 1).or_insert(1);
    });
  }

  /**
   * @param secs 间隔时间（秒）
   * @param callback 回调函数
   */
  #[napi(js_name = "loop", ts_args_type = "secs: number, callback: (err, result: { code: string, data: { [key: string]: number } }) => void")]
  #[allow(dead_code)]
  pub fn call(&self, secs: i32, callback: napi::JsFunction)  -> Result<()> {
    let tsfn: ThreadsafeFunction<Arc<Mutex<BTreeMap<String, BTreeMap<String, u32>>>>, ErrorStrategy::CalleeHandled> = callback
      .create_threadsafe_function(0, |ctx: ThreadSafeCallContext<Arc<Mutex<BTreeMap<String, BTreeMap<String, u32>>>>>| {
        let mut map = ctx.value.lock().unwrap();
        let mut obj = ctx.env.create_object()?;
        loop {
          match map.pop_first() {
            Some((code, map)) => {
              let mut data = ctx.env.create_object()?;
              for (key, val) in map.iter() {
                data.set_named_property(key, ctx.env.create_uint32(*val)?)?;
              }
              obj.set_named_property(&code, data)?;
            }
            None => break
          }
        }
        Ok(vec![obj])
      })?;
    let tree = Arc::clone(&self.tree);
    self.task.spawn(async move {
      let mut last_instant = Instant::now();
      loop {
          let len = tree.lock().unwrap().len();
          if len > 0 {
            tsfn.call(Ok(Arc::clone(&tree)), ThreadsafeFunctionCallMode::NonBlocking);
          }
          let now = Instant::now();
          let elapsed = now.duration_since(last_instant);
          let secs = Duration::from_secs(secs as u64);
          let sleep_duration = if elapsed >= secs {
            Duration::from_secs(0)
          } else {
            secs - elapsed
          };
          tokio::time::sleep(sleep_duration).await;
          last_instant = now + sleep_duration;
      }
    });
    Ok(())
  }
}