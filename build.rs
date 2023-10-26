fn main() {
    let target_vendor: String = std::env::var("CARGO_CFG_TARGET_VENDOR").expect("CARGO_CFG_TARGET_VENDOR is unavailable!");
    let target_family: String = std::env::var("CARGO_CFG_TARGET_FAMILY").expect("CARGO_CFG_TARGET_FAMILY is unavailable!");

    let mut target_platform: &'static str = "Unknown";

    if target_vendor == "apple" {
        target_platform = "Darwin"
    } else if target_family == "windows" {
        target_platform = "Windows"
    } else if target_family == "unix" {
        target_platform = "Unix"
    } else {
        panic!("[ERROR - build.rs] target vendor/family type is: {}", target_platform);
    }

    println!("Target vendor/family type is: {}", target_platform);
}