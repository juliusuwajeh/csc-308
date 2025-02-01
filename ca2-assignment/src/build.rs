use std::process::Command;

fn main() {
    // Build the kernel
    Command::new("cargo")
        .arg("build")
        .arg("--package")
        .arg("kernel_with_bootloader")
        .status()
        .expect("Failed to build kernel");

    // Add other build steps if needed
}
