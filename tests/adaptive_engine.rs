use archx::*;
use archx::hardware::HardwareInfo;
use archx::adaptive::{AdaptiveEngine, Strategy};

#[test]
fn test_adaptive_heuristics() {
    let info = HardwareInfo::detect();
    let mut hints = WorkloadHints::default();

    // Strategy for tiny data
    assert_eq!(AdaptiveEngine::choose_strategy(500, &hints, &info), Strategy::ScalarFallback);

    // Strategy for medium data
    assert_eq!(AdaptiveEngine::choose_strategy(10000, &hints, &info), Strategy::SingleThreadSimd);

    // Power Saving strategy
    hints.power_mode = PowerMode::PowerSaving;
    assert_eq!(AdaptiveEngine::choose_strategy(100_000, &hints, &info), Strategy::SingleThreadSimd);

    // Resource capping
    hints.power_mode = PowerMode::Balanced;
    hints.max_cpu_usage = Some(0.1);
    let s = AdaptiveEngine::choose_strategy(1_000_000, &hints, &info);
    if let Strategy::ParallelSimd(n) = s {
        assert!(n <= (info.logical_processors as f32 * 0.1).max(1.0) as usize + 1);
    } else {
        panic!("Should have been ParallelSimd");
    }
}
