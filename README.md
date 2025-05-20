# Moving Averages

[![Crates.io](https://img.shields.io/crates/v/moving_averages.svg)](https://crates.io/crates/moving_averages)
[![Documentation](https://docs.rs/moving_averages/badge.svg)](https://docs.rs/moving_averages)
[![License](https://img.shields.io/badge/license-WTFPL-blue.svg)](https://github.com/0ldm0s/moving_averages/blob/master/LICENSE)

生产级移动平均算法库，支持多种移动平均算法实现。

## 特性

- 支持三种移动平均算法：
  - 简单移动平均(SMA)
  - 指数移动平均(EMA)
  - 递归移动平均(RMA)
- `no_std` 兼容
- 线程安全设计
- 高性能实现

## 安装

在`Cargo.toml`中添加依赖：

```toml
[dependencies]
moving_averages = "0.1"
```

## 使用示例

```rust
use moving_averages::{Sma, Ema, Rma, MovingAverage};

let mut sma = Sma::<f64, 3>::new(); // 3期简单移动平均
let mut ema = Ema::new(0.5); // alpha=0.5的指数移动平均

sma.next(1.0);
ema.next(1.0);
```

## 文档

完整API文档请参考: [docs.rs](https://docs.rs/moving_averages)

## 许可证

本项目采用 [WTFPL](http://www.wtfpl.net/) 许可证。