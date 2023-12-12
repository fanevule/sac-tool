# sac-tool

[![license](https://img.shields.io/npm/l/napi-nanoid?color=cyan)](https://revolunet.mit-license.org/) [![npm](https://img.shields.io/npm/v/napi-nanoid?color=orange)](https://www.npmjs.com/package/napi-nanoid)

`The math tool written in Rust`

`一款基于 Rust 实现的 数据计算工具`

## Install

```shell
$ npm i npm i @nevule/sac-tool
```

## Usage

```js
import * as sac from '@nevule/sac-tool';
// 计算离群值
sac.outliers([1, 2, 3, 4, 3, 1, 2, 4, 10, 15]); // => [10, 15]
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
