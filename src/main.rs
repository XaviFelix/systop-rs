use procfs::{CpuInfo, Current};
use systop_rs::cpu::{print_all_cores, to_core_info};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //TODO: test main behavior for CPU
    let cpuinfo = CpuInfo::current()?;

    let mut all = Vec::new();
    for idx in 0..cpuinfo.num_cores() {
        if let Some(core_map) = cpuinfo.get_info(idx).as_ref() {
            let current_core_info = to_core_info(idx, core_map);
            all.push(current_core_info);
        }
    }
    print_all_cores(&all);

    Ok(())
}
