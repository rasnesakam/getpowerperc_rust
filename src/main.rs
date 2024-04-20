use std::fs;



fn find_bat(suply_path: &str) -> Result<String, String> {
    match fs::read_dir(&suply_path) {
        Ok(supply_dir) => {
            for entry in supply_dir {
                match entry {
                    Ok(dir_entry) => {
                        let path = dir_entry.path();
                        if path.is_dir() && path.file_name().unwrap().to_str().unwrap().starts_with("BAT"){
                            return Ok(String::from(path.to_str().unwrap()))
                        }
                    },
                    Err(_) => {

                    }
                }
            }
        }
        Err(_) => {
            return Err(format!("Unable to find file {}.", suply_path))
        }
    }
    Err(String::from("Unable to find BAT file."))
}

fn main() {
    match find_bat("/sys/class/power_supply") {
        Ok(path_battery) => {
            let path_energy_now = format!("{}/energy_now", path_battery);
            let path_energy_full = format!("{}/energy_full", path_battery);
            let str_energy_now = fs::read_to_string(&path_energy_now).expect(format!("Path '{path_energy_now}' could not open").as_str());
            let str_energy_full = fs::read_to_string(&path_energy_full).expect(format!("Path '{path_energy_full}' could not open").as_str());
            let energy_now: u64 = str_energy_now.trim().parse().expect("Expected numeric content");
            let energy_full: u64 = str_energy_full.trim().parse().expect("Expected numeric content.");
            let energy_percent: f32 = (energy_now as f32 / energy_full as f32) * 100.0;
            println!("{energy_percent}%")
        }
        Err(_) => {
            panic!("Couldn't read power supply")
        }
    }
}
