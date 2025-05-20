use moving_averages::{Sma, Ema, Rma, MovingAverage};
use float_cmp::approx_eq;

#[test]
fn test_trait_implementation() {
    let mut calculators: Vec<Box<dyn MovingAverage<f64>>> = vec![
        Box::new(Sma::<f64, 3>::new()),
        Box::new(Ema::new(0.5)),
        Box::new(Rma::<f64, 3>::new()),
    ];

    for calc in &mut calculators {
        assert!(approx_eq!(f64, calc.next(1.0), 1.0, epsilon = 1e-10));
        calc.reset();
    }
}

#[test]
fn test_mixed_usage() {
    let mut sma = Sma::<f64, 2>::new();
    let mut ema = Ema::new(0.3);
    
    assert!(approx_eq!(f64, sma.next(1.0), 1.0, epsilon = 1e-10));
    assert!(approx_eq!(f64, ema.next(1.0), 1.0, epsilon = 1e-10));
    
    assert!(approx_eq!(f64, sma.next(2.0), 1.5, epsilon = 1e-10));
    assert!(approx_eq!(f64, ema.next(2.0), 1.3, epsilon = 1e-10));
}