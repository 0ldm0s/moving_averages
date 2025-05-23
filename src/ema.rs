//! 指数移动平均(EMA)实现
//! 
//! 公式：EMA_today = (value * smoothing) + (EMA_yesterday * (1 - smoothing))

#![no_std]

use num_traits::{Float, Zero};

/// 指数移动平均计算器
#[derive(Debug)]
pub struct Ema<T> {
    current: Option<T>,
    alpha: T,
}

impl<T: Float> Ema<T> {
    /// 创建新的EMA计算器
    /// 
    /// # 参数
    /// - alpha: 平滑系数(0-1之间)
    /// 
    /// # 示例
    /// ```
    /// use moving_averages::ema::Ema;
    /// let mut ema = Ema::new(0.5);
    /// ```
    pub fn new(alpha: T) -> Self {
        assert!(alpha > T::zero() && alpha <= T::one(), "Alpha必须在(0,1]范围内");
        Self {
            current: None,
            alpha,
        }
    }

    /// 添加新值并返回当前EMA
    pub fn next(&mut self, value: T) -> T {
        self.current = Some(match self.current {
            Some(prev) => prev + (value - prev) * self.alpha, // 优化计算顺序
            None => value,
        });
        self.current.unwrap()
    }

    /// 重置计算器状态
    pub fn reset(&mut self) {
        self.current = None;
    }

    /// 获取当前alpha值
    pub fn alpha(&self) -> T {
        self.alpha
    }

    pub fn set_alpha(&mut self, alpha: T) {
        self.alpha = alpha;
    }
}

