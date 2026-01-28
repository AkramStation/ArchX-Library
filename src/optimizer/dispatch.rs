enum ExecPath {
    Avx2,
    Avx,
    Scalar,
}

fn select_path(cpu: &CpuInfo) -> ExecPath {
    if cpu.features.avx2 {
        ExecPath::Avx2
    } else if cpu.features.avx {
        ExecPath::Avx
    } else {
        ExecPath::Scalar
    }
}
