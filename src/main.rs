use std::fs;

fn main() {
    let path_energy_now = "/sys/class/power_supply/BAT0/energy_now";
    let path_energy_full = "/sys/class/power_supply/BAT0/energy_full";
    let str_energy_now = fs::read_to_string(path_energy_now).expect(format!("Path '{path_energy_now}' could not open").as_str());
    let str_energy_full = fs::read_to_string(path_energy_full).expect(format!("Path '{path_energy_full}' could not open").as_str());
    let energy_now: u64 = str_energy_now.trim().parse().expect("Expected numeric content");
    let energy_full: u64 = str_energy_full.trim().parse().expect("Expected numeric content.");
    let energy_percent: f32 = (energy_now as f32 / energy_full as f32) * 100.0;
    println!("{energy_percent}%")
}
