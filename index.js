/* tslint:disable */
/* eslint-disable */
/* prettier-ignore */

/* auto-generated by NAPI-RS */

const { existsSync, readFileSync } = require('fs')
const { join } = require('path')

const { platform, arch } = process

let nativeBinding = null
let localFileExisted = false
let loadError = null

function isMusl() {
  // For Node 10
  if (!process.report || typeof process.report.getReport !== 'function') {
    try {
      const lddPath = require('child_process').execSync('which ldd').toString().trim()
      return readFileSync(lddPath, 'utf8').includes('musl')
    } catch (e) {
      return true
    }
  } else {
    const { glibcVersionRuntime } = process.report.getReport().header
    return !glibcVersionRuntime
  }
}

switch (platform) {
  case 'android':
    switch (arch) {
      case 'arm64':
        localFileExisted = existsSync(join(__dirname, 'sac-tool.android-arm64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./sac-tool.android-arm64.node')
          } else {
            nativeBinding = require('@nevule/sac-tool-android-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm':
        localFileExisted = existsSync(join(__dirname, 'sac-tool.android-arm-eabi.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./sac-tool.android-arm-eabi.node')
          } else {
            nativeBinding = require('@nevule/sac-tool-android-arm-eabi')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Android ${arch}`)
    }
    break
  case 'win32':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(
          join(__dirname, 'sac-tool.win32-x64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./sac-tool.win32-x64-msvc.node')
          } else {
            nativeBinding = require('@nevule/sac-tool-win32-x64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'ia32':
        localFileExisted = existsSync(
          join(__dirname, 'sac-tool.win32-ia32-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./sac-tool.win32-ia32-msvc.node')
          } else {
            nativeBinding = require('@nevule/sac-tool-win32-ia32-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'sac-tool.win32-arm64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./sac-tool.win32-arm64-msvc.node')
          } else {
            nativeBinding = require('@nevule/sac-tool-win32-arm64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Windows: ${arch}`)
    }
    break
  case 'darwin':
    localFileExisted = existsSync(join(__dirname, 'sac-tool.darwin-universal.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./sac-tool.darwin-universal.node')
      } else {
        nativeBinding = require('@nevule/sac-tool-darwin-universal')
      }
      break
    } catch {}
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(join(__dirname, 'sac-tool.darwin-x64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./sac-tool.darwin-x64.node')
          } else {
            nativeBinding = require('@nevule/sac-tool-darwin-x64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'sac-tool.darwin-arm64.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./sac-tool.darwin-arm64.node')
          } else {
            nativeBinding = require('@nevule/sac-tool-darwin-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on macOS: ${arch}`)
    }
    break
  case 'freebsd':
    if (arch !== 'x64') {
      throw new Error(`Unsupported architecture on FreeBSD: ${arch}`)
    }
    localFileExisted = existsSync(join(__dirname, 'sac-tool.freebsd-x64.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./sac-tool.freebsd-x64.node')
      } else {
        nativeBinding = require('@nevule/sac-tool-freebsd-x64')
      }
    } catch (e) {
      loadError = e
    }
    break
  case 'linux':
    switch (arch) {
      case 'x64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'sac-tool.linux-x64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./sac-tool.linux-x64-musl.node')
            } else {
              nativeBinding = require('@nevule/sac-tool-linux-x64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'sac-tool.linux-x64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./sac-tool.linux-x64-gnu.node')
            } else {
              nativeBinding = require('@nevule/sac-tool-linux-x64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'sac-tool.linux-arm64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./sac-tool.linux-arm64-musl.node')
            } else {
              nativeBinding = require('@nevule/sac-tool-linux-arm64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'sac-tool.linux-arm64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./sac-tool.linux-arm64-gnu.node')
            } else {
              nativeBinding = require('@nevule/sac-tool-linux-arm64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm':
        localFileExisted = existsSync(
          join(__dirname, 'sac-tool.linux-arm-gnueabihf.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./sac-tool.linux-arm-gnueabihf.node')
          } else {
            nativeBinding = require('@nevule/sac-tool-linux-arm-gnueabihf')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'riscv64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'sac-tool.linux-riscv64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./sac-tool.linux-riscv64-musl.node')
            } else {
              nativeBinding = require('@nevule/sac-tool-linux-riscv64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'sac-tool.linux-riscv64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./sac-tool.linux-riscv64-gnu.node')
            } else {
              nativeBinding = require('@nevule/sac-tool-linux-riscv64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      default:
        throw new Error(`Unsupported architecture on Linux: ${arch}`)
    }
    break
  default:
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
}

if (!nativeBinding) {
  if (loadError) {
    throw loadError
  }
  throw new Error(`Failed to load native binding`)
}

const { sum, remainder, subtract, multiply, divide, mean, median, mode, max, min, range, interquartileRange, fibonacci, quartile, quartiles, variance, standardDeviation, outliers, outliersGroup, coefficientOfVariation, linearCounting, probabilisticCounting, pearson, spearmanRankCorrelation, kendallTau, Calculate, Report } = nativeBinding

module.exports.sum = sum
module.exports.remainder = remainder
module.exports.subtract = subtract
module.exports.multiply = multiply
module.exports.divide = divide
module.exports.mean = mean
module.exports.median = median
module.exports.mode = mode
module.exports.max = max
module.exports.min = min
module.exports.range = range
module.exports.interquartileRange = interquartileRange
module.exports.fibonacci = fibonacci
module.exports.quartile = quartile
module.exports.quartiles = quartiles
module.exports.variance = variance
module.exports.standardDeviation = standardDeviation
module.exports.outliers = outliers
module.exports.outliersGroup = outliersGroup
module.exports.coefficientOfVariation = coefficientOfVariation
module.exports.linearCounting = linearCounting
module.exports.probabilisticCounting = probabilisticCounting
module.exports.pearson = pearson
module.exports.spearmanRankCorrelation = spearmanRankCorrelation
module.exports.kendallTau = kendallTau
module.exports.Calculate = Calculate
module.exports.Report = Report
