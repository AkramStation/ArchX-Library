# Hardware Detection & Sovereign Awareness

ArchX Sovereign v3.0 uses a multi-layered hardware detection strategy to ensure maximum performance across diverse architectures while maintaining device safety.

## 🚀 SIMD Hierarchy

The engine utilizes runtime feature detection to select the widest available instruction set:

1.  **AVX-512**: 512-bit registers. Processes 16 floats per lane. Supported on high-end server and workstation CPUs.
2.  **AVX2**: 256-bit registers with FMA (Fused Multiply-Add). The standard high-performance path for modern x86_64.
3.  **AVX**: 256-bit registers. Legacy high-performance path.
4.  **SSE2**: 128-bit registers (4 floats per lane). The baseline requirement for 64-bit x86 systems.
5.  **Neon**: Advanced SIMD for ARM64 (Apple Silicon, Raspberry Pi 4+, AWS Graviton).

## 🕵️ Sovereign Awareness (v3.0)

Unlike previous versions, Sovereign v3.0 implements **Deep Awareness**, which goes beyond raw bits:

### 1. CPU Brand & Micro-Architecture
ArchX now retrieves the full brand string (e.g., *"AMD Ryzen 5 PRO 5650G"*) and recognizes core topologies. It understands the difference between high-power and efficiency cores, optimizing loop unrolling accordingly.

### 2. Integrated GPU (iGPU) Recognition
ArchX Sovereign v3.0 features **Predictive iGPU Detection**:
- **API Metadata**: Queries active backends (Vulkan/OpenCL) for device names.
- **Brand Parsing**: Parses the CPU brand for keywords (Radeon, Intel UHD, Iris) to identify integrated graphics even before an API is initialized.
- **Shared Memory Awareness**: Automatically detects when the GPU shares system RAM, adjusting buffer strategy to minimize copy overhead.

### 3. Load & Resource Monitoring
Integrated with `sysinfo`, the engine monitors real-time system pressure:
- **CPU Usage %**: Throttles parallel task distribution if the machine is under heavy load.
- **VRAM Availability**: The GPU Manager queries available video memory to prevent task failures.

## 💤 Power & Battery Awareness
When running on mobile devices, ArchX detects the power source. If on battery, it automatically lowers the `PowerMode` to conserve energy by prioritizing efficiency cores and reducing the frequency of GPU offloads.

---
*ArchX Sovereign v3.0: Hardware-First Acceleration.*
