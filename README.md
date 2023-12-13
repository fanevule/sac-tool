# sac-tool

[![license](https://img.shields.io/npm/l/@nevule/sac-tool?color=cyan)](https://revolunet.mit-license.org/) [![npm](https://img.shields.io/npm/v/@nevule/sac-tool?color=orange)](https://www.npmjs.com/package/@nevule/sac-tool)

`The math tool written in Rust`

`一款基于 Rust 实现的 数据计算`

## Install

```shell
$ npm i @nevule/sac-tool
```

## Usage

```js
import * as sac from '@nevule/sac-tool';
// 获取离群元素
sac.outliers(new Float64Array([1, 2, 3, 4, 3, 1, 2, 4, 10, 15])); // => [10, 15]
```

## Test
```js
const numbers = (new Array(10000000).fill(1).map(() => Math.random() * 1000000));

console.time('js')
js.coefficientOfVariation(numbers)
console.timeEnd('js') // 1.119s

console.time('rust')
sac.coefficientOfVariation(new Float64Array(numbers))
console.timeEnd('rust') // 122.788ms

console.time('rust-wasm')
wasm.coefficientOfVariation(new Float64Array(numbers))
console.timeEnd('rust-wasm') // 164.442ms
```

## Support matrix


|                  | node14 | node16 | node18 |
| ---------------- | ------ | ------ | ------ |
| Windows x64      | ✓     | ✓     | ✓     |
| Windows x32      | ✓     | ✓     | ✓     |
| Windows arm64    | ✓     | ✓     | ✓     |
| macOS x64        | ✓     | ✓     | ✓     |
| macOS arm64      | ✓     | ✓     | ✓     |
| Linux x64 gnu    | ✓     | ✓     | ✓     |
| Linux x64 musl   | ✓     | ✓     | ✓     |
| Linux arm gnu    | ✓     | ✓     | ✓     |
| Linux arm64 gnu  | ✓     | ✓     | ✓     |
| Linux arm64 musl | ✓     | ✓     | ✓     |
| Android arm64    | ✓     | ✓     | ✓     |
| Android armv7    | ✓     | ✓     | ✓     |
| FreeBSD x64      | ✓     | ✓     | ✓     |

## License

[MIT](https://opensource.org/licenses/MIT)
