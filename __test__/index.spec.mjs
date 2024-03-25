import test from 'ava'

import { Report, Calculate } from '../index.js'

const report = new Report()
const math = new Calculate()

report.loop(1, (err, result) => {
  console.log(err, result)
});

test('sum', async (t) => {
  report.avg('b', 100)
  t.is(await math.sum(1, 2), 3)
})

test('quartile', async (t) => {
  report.avg('b', 58)
  report.incr('a')
  t.is(await math.quartile(90, new Float64Array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])), 45 / 9)
})

test('pearson', async (t) => {
  report.incr('a')
  t.is(await math.pearson(new Float64Array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), new Float64Array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])), 1)
})

test('spearmanRankCorrelation', async (t) => {
  report.avg('b', 56)
  report.incr('a')
  t.is(await math.spearmanRankCorrelation(new Float64Array([1, 2, 2, 4, 5, 6, 7, 8, 9, 10]), new Float64Array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])), 1)
})

test('kendallTau', async (t) => {
  report.incr('a')
  t.is(await math.kendallTau(new Float64Array([1, 2, 3, 4, 4.5, 6.2, 7, 8, 9, 10]), new Float64Array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])), 1)
})

test('delay', async (t) => {
  report.incr('a')
  await new Promise((resolve) => setTimeout(resolve, 1000))
  t.is(3, 3)
})