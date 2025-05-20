//! 滚动移动平均(RMA)实现
//!
//! 公式：RMA = (当前值 + 前n-1个RMA) / n

#![no_std]

use num_traits::{Float, Zero};
use core::marker::PhantomData;

/// 滚动移动平均计算器
#[derive(Debug)]
pub struct Rma<T, const N: usize> {
    buffer: [T; N],
    index: usize,
    initialized: bool,
    current: T,
    _phantom: PhantomData<T>,
}

impl<T: Float + Zero + Default + Copy, const N: usize> Rma<T, N> {
    /// 创建新的RMA计算器
    ///
    /// # 示例
    /// ```
    /// use moving_averages::rma::Rma;
    /// let mut rma = Rma::<f64, 5>::new();
    /// ```
    pub fn new() -> Self {
        assert!(N > 0, "窗口大小必须大于0");
        Self {
            buffer: [T::zero(); N],
            index: 0,
            initialized: false,
            current: T::zero(),
            _phantom: PhantomData,
        }
    }

    /// 添加新值并返回当前RMA
    pub fn next(&mut self, value: T) -> T {
        if !self.initialized {
            self.buffer.fill(value);
            self.current = value;
            self.initialized = true;
            return value;
        }

        self.buffer[self.index] = value;
        self.index = (self.index + 1) % N;

        let sum = self.buffer.iter().fold(T::zero(), |acc, &x| acc + x);
        self.current = sum / T::from(N).unwrap();
        self.current
    }

    /// 重置计算器状态
    pub fn reset(&mut self) {
        self.buffer = [T::zero(); N];
        self.index = 0;
        self.initialized = false;
        self.current = T::zero();
    }
}

// 保证线程安全
unsafe impl<T: Send, const N: usize> Send for Rma<T, N> {}
unsafe impl<T: Sync, const N: usize> Sync for Rma<T, N> {}
