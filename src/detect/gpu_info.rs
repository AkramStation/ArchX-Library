use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct GpuDetails {
    pub name: String,
    pub vendor: String,
    pub is_integrated: bool,
    pub shared_memory: bool,
}

pub fn detect_gpu(cpu_brand: Option<&str>) -> Option<GpuDetails> {
    // 1. Try to find a registered/active backend
    if let Some(name) = crate::gpu::get_active_backend_name() {
        return Some(GpuDetails {
            name: name.clone(),
            vendor: "Detected API".to_string(),
            is_integrated: false, // Could be determined by API specifics
            shared_memory: name == "OpenGL" || name == "OpenCL",
        });
    }

    // 2. Fallback to predictive iGPU detection based on CPU brand
    if let Some(brand) = cpu_brand {
        let brand_lower = brand.to_lowercase();
        if brand_lower.contains("radeon") || brand_lower.contains("vega") || brand_lower.contains("graphics") {
            return Some(GpuDetails {
                name: "Integrated Radeon Graphics".to_string(),
                vendor: "AMD".to_string(),
                is_integrated: true,
                shared_memory: true,
            });
        }
        if brand_lower.contains("intel") || brand_lower.contains("iris") || brand_lower.contains("uhd") {
            return Some(GpuDetails {
                name: "Integrated Intel Graphics".to_string(),
                vendor: "Intel".to_string(),
                is_integrated: true,
                shared_memory: true,
            });
        }
    }

    None
}
