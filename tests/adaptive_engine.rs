use archx::*;
use archx::hardware::SystemInfo;
use archx::adaptive::{AdaptiveEngine, Strategy};

#[test]
fn test_adaptive_heuristics() {
    let info = SystemInfo::detect();
    let hints = WorkloadHints::default();

    // 1. Scalar Fallback for tiny data (< 1024 in v2.0)
    let s_tiny = AdaptiveEngine::choose_strategy(500, &hints, &info);
    assert_eq!(s_tiny, Strategy::ScalarFallback);

    // 2. Single-threaded SIMD for medium data
    let s_med = AdaptiveEngine::choose_strategy(10000, &hints, &info);
    assert_eq!(s_med, Strategy::SingleThreadSimd);

    // 3. Power Saving strategy
    let hints_ps = WorkloadHints {
        power_mode: PowerMode::PowerSaving,
        ..Default::default()
    };
    let s_ps = AdaptiveEngine::choose_strategy(100_000, &hints_ps, &info);
    assert_eq!(s_ps, Strategy::SingleThreadSimd);

    // 4. Resource capping
    let hints_cap = WorkloadHints {
        max_cpu_usage: Some(0.1),
        ..Default::default()
    };
    let s_cap = AdaptiveEngine::choose_strategy(1_000_000, &hints_cap, &info);
    
    if let Strategy::ParallelSimd(n) = s_cap {
        let expected_max = (info.cpu.logical_processors as f32 * 0.1).max(1.0) as usize;
        assert!(n <= expected_max + 1, "Thread count {} exceeded expected max {}", n, expected_max + 1);
    } else if info.cpu.logical_processors == 1 {
        assert!(matches!(s_cap, Strategy::SingleThreadSimd | Strategy::ParallelSimd(_)));
    } else {
        panic!("Expected ParallelSimd for 1M elements on multicore, got {:?}", s_cap);
    }
}
