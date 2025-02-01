fn main() {
    // Define the kernel package
    let kernel_package = "kernel_with_bootloader";

    // Define the bootloader package
    let bootloader_package = "bootloader";

    // Define the target triple
    let target = "x86_64-unknown-none";

    // Build the kernel
    let status = std::process::Command::new("cargo")
        .args(&["build", "--package", kernel_package, "--target", target])
        .status()
        .expect("Failed to build kernel");

    if !status.success() {
        panic!("Kernel build failed");
    }

    // Build the bootloader
    let status = std::process::Command::new("cargo")
        .args(&["build", "--package", bootloader_package, "--target", target])
        .status()
        .expect("Failed to build bootloader");

    if !status.success() {
        panic!("Bootloader build failed");
    }

    // Create the disk image
    let status = std::process::Command::new("cargo")
        .args(&["bootimage", "--package", kernel_package, "--target", target])
        .status()
        .expect("Failed to create bootable disk image");

    if !status.success() {
        panic!("Disk image creation failed");
    }

    println!("Bootable disk image created successfully");
}
