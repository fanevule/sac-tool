/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

/**
 * 求和
*/
export function sum(a: number, b: number): number
/**
 * 求余
*/
export function remainder(a: number, b: number): number
/**
 * 减法
*/
export function subtract(a: number, b: number): number
/**
 * 乘法
*/
export function multiply(a: number, b: number): number
/**
 * 除法
*/
export function divide(a: number, b: number): number
/**
 * 求平均值
*/
export function mean(numbers: Float64Array): number
/**
 * 求中位数
*/
export function median(numbers: Float64Array): number
/**
 * 求众数
*/
export function mode(numbers: Int32Array): number
/**
 * 求最大值
*/
export function max(numbers: Float64Array): number
/**
 * 求最小值
*/
export function min(numbers: Float64Array): number
/**
 * 求极差
*/
export function range(numbers: Float64Array): number
/**
 * 求四分位距
*/
export function interquartileRange(numbers: Float64Array): number
/**
 * 斐波那契数列
*/
export function fibonacci(n: number): number
/**
 * 计算分位数，比如P25，P50，P75
 * eg: quartile(25, [1.0, 2.0, 3.0, 4.0, 5.0]) => 2.0
*/
export function quartile(p: number, numbers: Float64Array): number
/**
 * 计算四分位数
*/
export function quartiles(numbers: Float64Array): number[]
/**
 * 计算方差
*/
export function variance(numbers: Float64Array): number
/**
 * 计算标准差
*/
export function standardDeviation(numbers: Float64Array): number
/**
 * 计算离群值
*/
export function outliers(numbers: Float64Array): Array<number>
/**
 * 计算变异系数
*/
export function coefficientOfVariation(numbers: Float64Array): number
/**
 * 线性计数
*/
export function linearCounting(numbers: Int32Array): number
/**
 * 概率计数
*/
export function probabilisticCounting(numbers: Int32Array): number
/**
 * 计算皮尔逊相关系数
*/
export function pearson(x: Float64Array, y: Float64Array): number
/**
 * 计算斯皮尔曼等级相关系数
*/
export function spearmanRankCorrelation(x: Float64Array, y: Float64Array): number
/**
 * 计算肯德尔等级相关系数
*/
export function kendallTau(x: Float64Array, y: Float64Array): number
export class Report {
  constructor()
  incr(code: string, key: string): void
  /**
  * @param secs 间隔时间（秒）
  * @param callback 回调函数
  */
  loop(secs: number, callback: (err, result: { code: string, data: { [key: string]: number } }) => void): void
}
