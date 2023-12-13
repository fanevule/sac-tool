import test from 'ava'

import { sum, quartile } from '../index.js'

test('sum', (t) => {
  t.is(sum(1, 2), 3)
})

test('quartile', (t) => {
  t.is(quartile(90, new Float64Array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])), 45 / 9)
})
