import test from 'ava'

import { sum, quartile, spearmanRankCorrelation, pearson, kendallTau, Report } from '../index.js'
const report = new Report()
report.loop(1, (err, result) => {
  console.log(err, result)
});

test('sum', (t) => {
  report.avg('b', 100)
  t.is(sum(1, 2), 3)
})

test('quartile', (t) => {
  report.avg('b', 58)
  report.incr('a')
  t.is(quartile(90, new Float64Array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])), 45 / 9)
})

test('pearson', (t) => {
  report.incr('a')
  t.is(pearson(new Float64Array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), new Float64Array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])), 1)
})

test('spearmanRankCorrelation', (t) => {
  report.avg('b', 56)
  report.incr('a')
  t.is(spearmanRankCorrelation(new Float64Array([1, 2, 2, 4, 5, 6, 7, 8, 9, 10]), new Float64Array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])), 1)
})

test('kendallTau', (t) => {
  report.incr('a')
  t.is(kendallTau(new Float64Array([1, 2, 3, 4, 4.5, 6.2, 7, 8, 9, 10]), new Float64Array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])), 1)
})

test('delay', async (t) => {
  await new Promise((resolve) => setTimeout(resolve, 1000))
  report.incr('a')
  t.is(kendallTau(new Float64Array([1, 2, 3, 4, 4.5, 6.2, 7, 8, 9, 10]), new Float64Array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])), 1)
})