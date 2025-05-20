//! 生产级移动平均算法库
//!
//! 提供三种移动平均算法实现：
//! - 简单移动平均(SMA)
//! - 指数移动平均(EMA) 
//! - 滚动移动平均(RMA)
//!
//! ## 特性
//! - `no_std`兼容
//! - 线程安全设计
//! - 零成本抽象
//! - 完整文档测试
//!
//! ## 示例
//! ```
//! use moving_averages::{Sma, Ema, Rma, MovingAverage};
//!
//! // SMA示例
//! let mut sma = Sma::<f64, 3>::new();
//! println!("SMA: {}", sma.next(10.0));
//!
//! // 使用trait统一接口
//! let mut calculator: Box<dyn MovingAverage<f64>> = Box::new(sma);
//! println!("Average: {}", calculator.next(15.0));
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

pub mod ema;
pub mod sma;
pub mod rma;

pub use ema::Ema;
pub use sma::Sma;
pub use rma::Rma;

use num_traits::{Float, Zero};

/// 移动平均算法统一trait
pub trait MovingAverage<T> {
    /// 添加新值并返回当前平均值
    fn next(&mut self, value: T) -> T;

    /// 重置计算器状态
    fn reset(&mut self);
}

impl<T, const N: usize> MovingAverage<T> for Sma<T, N>
where
    T: Float + Zero + Default + Copy,
{
    fn next(&mut self, value: T) -> T {
        self.next(value)
    }

    fn reset(&mut self) {
        self.reset()
    }
}

impl<T> MovingAverage<T> for Ema<T>
where
    T: Float,
{
    fn next(&mut self, value: T) -> T {
        self.next(value)
    }

    fn reset(&mut self) {
        self.reset()
    }
}

impl<T, const N: usize> MovingAverage<T> for Rma<T, N>
where
    T: Float + Zero + Default + Copy,
{
    fn next(&mut self, value: T) -> T {
        self.next(value)
    }

    fn reset(&mut self) {
        self.reset()
    }
}
