import test from 'ava'

import { sum, quartile, spearmanRankCorrelation, pearson, kendallTau } from '../index.js'

test('sum', (t) => {
  t.is(sum(1, 2), 3)
})

test('quartile', (t) => {
  t.is(quartile(90, new Float64Array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])), 45 / 9)
})

test('pearson', (t) => {
  t.is(pearson(new Float64Array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), new Float64Array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])), 1)
})

test('spearmanRankCorrelation', (t) => {
  t.is(spearmanRankCorrelation(new Float64Array([1, 2, 2, 4, 5, 6, 7, 8, 9, 10]), new Float64Array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])), 1)
})

test('kendallTau', (t) => {
  t.is(kendallTau(new Float64Array([1, 2, 3, 4, 4.5, 6.2, 7, 8, 9, 10]), new Float64Array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])), 1)
})

test('delay', async (t) => {
  t.is(kendallTau(new Float64Array([1, 2, 3, 4, 4.5, 6.2, 7, 8, 9, 10]), new Float64Array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])), 1)
})