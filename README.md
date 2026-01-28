# ArchX 🚀

**ArchX** is a primitive but solid CPU-aware optimization foundation for Rust. It provides a clean, modular architecture for detecting CPU features at runtime and dispatching optimized execution paths, ensuring performance without sacrificing safety or portability.

> [!NOTE]
> This is **v0.9** (Smart Adaptive). The "Intelligence" release introduces a **hardware-aware optimization engine**, **dynamic resource-capping**, and **power-efficient modes** for superior performance across varied system loads.

---

## ✨ Features

- **🔍 Intelligent CPU Detection**: Auto-detects architecture (x86_64, AArch64, etc.), bitness, and instruction set extensions (SSE2, AVX, AVX2).
- **🛤️ Auto-Dispatch System**: Internal logic that automatically selects the most efficient implementation for the host CPU.
- **🛡️ Safe Fallbacks**: Built-in scalar implementations ensure your code runs everywhere, even on dated hardware.
- **🧩 Modular Architecture**: Clean separation between detection, optimization, and dispatching.
- **🚫 Zero Dependencies**: Uses `std` for core detection logic.

## 🚀 Quick Start

Add `archx` to your project and start leveraging CPU-aware operations:

```rust
use archx;

fn main() {
    let a = vec![1.0, 2.0, 3.0, 4.0];
    let b = vec![5.0, 6.0, 7.0, 8.0];
    let mut out = vec![0.0; 4];

    // High-level API: Automatically chooses the best implementation
    archx::add(&a, &b, &mut out);

    println!("Result: {:?}", out); // [6.0, 8.0, 10.0, 12.0]
    
    // Inspect detected CPU capabilities
    let info = archx::get_info();
    println!("Running on: {:?} (AVX2: {})", info.arch, info.features.avx2);
}
```

## 🏗️ Project Structure

```text
src/
├── lib.rs          # Public API gateway
├── system.rs       # High-level API & CPU aggregation
├── cpu/            # CPU detection logic (arch, bits, features)
├── dispatch/       # Execution path selection & dispatcher
└── optimizer/      # Actual implementations (scalar fallback)
```

## 🛠️ Roadmap

- [x] **v0.1**: Core modular architecture, runtime detection, and scalar fallback.
- [ ] **v0.2**: SSE2 & AVX optimized implementations for common math operations.
- [ ] **v0.3**: AVX2 & AVX-512 support.
- [ ] **v0.4**: Cross-platform thread-pool integration for large-scale optimizations.

## 🤝 Contributing

Contributions are welcome! Whether it's adding a new SIMD path or improving detection logic, feel free to open a PR.

## ⚖️ License

Distributed under the MIT License. See `LICENSE` for more information.
