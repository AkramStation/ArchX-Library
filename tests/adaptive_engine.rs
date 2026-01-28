use archx::*;
use archx::hardware::HardwareInfo;
use archx::adaptive::{AdaptiveEngine, Strategy};

#[test]
fn test_adaptive_heuristics() {
    let info = HardwareInfo::detect();
    let mut hints = WorkloadHints::default();

    // 1. Scalar Fallback for tiny data (< 1000)
    let s_tiny = AdaptiveEngine::choose_strategy(500, &hints, &info);
    assert_eq!(s_tiny, Strategy::ScalarFallback);

    // 2. Single-threaded SIMD for medium data
    let s_med = AdaptiveEngine::choose_strategy(10000, &hints, &info);
    assert_eq!(s_med, Strategy::SingleThreadSimd);

    // 3. Power Saving strategy
    hints.power_mode = PowerMode::PowerSaving;
    let s_ps = AdaptiveEngine::choose_strategy(100_000, &hints, &info);
    assert_eq!(s_ps, Strategy::SingleThreadSimd);

    // 4. Resource capping
    hints.power_mode = PowerMode::Balanced;
    hints.max_cpu_usage = Some(0.1);
    let s_cap = AdaptiveEngine::choose_strategy(1_000_000, &hints, &info);
    
    if let Strategy::ParallelSimd(n) = s_cap {
        let expected_max = (info.logical_processors as f32 * 0.1).max(1.0) as usize;
        assert!(n <= expected_max + 1, "Thread count {} exceeded expected max {}", n, expected_max + 1);
    } else if info.logical_processors == 1 {
        // If system only has 1 core, it might stay SingleThreadSimd depending on thresholds
        assert!(matches!(s_cap, Strategy::SingleThreadSimd | Strategy::ParallelSimd(_)));
    } else {
        panic!("Expected ParallelSimd for 1M elements on multicore, got {:?}", s_cap);
    }
}
