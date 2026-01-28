use archx;

#[test]
fn test_cpu_detection_accuracy() {
    let info = archx::get_info();
    
    println!("\n--- ArchX CPU Detection Report ---");
    println!("Architecture: {:?}", info.arch);
    println!("Bitness:     {:?}", info.bits);
    println!("SSE2:        {}", info.features.sse2);
    println!("AVX:         {}", info.features.avx);
    println!("AVX2:        {}", info.features.avx2);
    println!("-----------------------------------\n");

    // Assertions for sane values on most CI/Dev machines
    // We don't assert true for AVX/AVX2 because they might not be available
    // but we can assert that architecture is not Unknown.
    assert_ne!(info.arch, archx::cpu::arch::CpuArch::Unknown, "CPU Architecture should be detectable");
}
