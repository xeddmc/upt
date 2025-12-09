use upt::vendor::init_vendor;

fn main() {
    // Test that all new package managers can be initialized
    let vendors = ["pip", "npm", "cargo", "gem", "composer"];

    for vendor_name in vendors {
        match init_vendor(vendor_name) {
            Ok(vendor) => {
                println!("{} initialized successfully", vendor.name());
                println!("  - Install: {}", vendor.install.help().unwrap_or_default());
                println!("  - Remove: {}", vendor.remove.help().unwrap_or_default());
                println!("  - Upgrade: {}", vendor.upgrade.help().unwrap_or_default());
            }
            Err(e) => {
                eprintln!("Failed to initialize {}: {}", vendor_name, e);
                std::process::exit(1);
            }
        }
    }

    println!("\nAll new package managers initialized successfully!");
}