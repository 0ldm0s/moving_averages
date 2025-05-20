//! 简单移动平均(SMA)实现
//!
//! 公式：SMA = (x1 + x2 + ... + xn) / n

#![no_std]

use num_traits::{Float, Zero};
use core::marker::PhantomData;

/// 简单移动平均计算器
#[derive(Debug)]
pub struct Sma<T, const N: usize> {
    buffer: [T; N],
    index: usize,
    sum: T,
    count: usize,
    _phantom: PhantomData<T>,
}

impl<T: Float + Zero + Default + Copy, const N: usize> Sma<T, N> {
    /// 创建新的SMA计算器
    ///
    /// # 示例
    /// ```
    /// use moving_averages::sma::Sma;
    /// let mut sma = Sma::<f64, 5>::new();
    /// ```
    pub fn new() -> Self {
        assert!(N > 0, "窗口大小必须大于0");
        Self {
            buffer: [T::zero(); N],
            index: 0,
            sum: T::zero(),
            count: 0,
            _phantom: PhantomData,
        }
    }

    /// 添加新值并返回当前SMA
    pub fn next(&mut self, value: T) -> T {
        if self.count < N {
            self.sum = self.sum + value;
            self.count += 1;
        } else {
            self.sum = self.sum - self.buffer[self.index] + value;
        }

        self.buffer[self.index] = value;
        self.index = (self.index + 1) % N;

        self.sum / T::from(self.count).unwrap()
    }

    /// 重置计算器状态
    pub fn reset(&mut self) {
        self.buffer = [T::zero(); N];
        self.index = 0;
        self.sum = T::zero();
        self.count = 0;
    }
}

// 保证线程安全
unsafe impl<T: Send, const N: usize> Send for Sma<T, N> {}
unsafe impl<T: Sync, const N: usize> Sync for Sma<T, N> {}
