use std::env;

fn main() {
    if boolean_env_var("ORT_ALLOW_DEBUG_LOGGING") {
        println!("cargo:rustc-cfg=allow_debug_logging");
    }
    println!("cargo:rerun-if-env-changed=ORT_ALLOW_DEBUG_LOGGING");
}

fn boolean_env_var(name: &str) -> bool {
    // Same as `ORT_USE_CUDA`.
    let var = env::var(name).unwrap_or_default();
    matches!(&*var.to_lowercase(), "1" | "yes" | "true" | "on")
}
