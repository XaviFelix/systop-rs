//TODO: Currently using raw system data in KB, need to convert data to GB or MB
//TODO: Create a dedicated display decorator for the terminal
//TODO: Use Clap and combine it with tokio for interval refreshing

use procfs::{CpuInfo, Current, Meminfo};
use systop_rs::components::{cpu::*, mem::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //TODO: test CPU behavior
    let cpuinfo = CpuInfo::current()?;

    let mut all = Vec::new();
    for idx in 0..cpuinfo.num_cores() {
        if let Some(core_map) = cpuinfo.get_info(idx).as_ref() {
            let current_core_info = to_core_info(idx, core_map);
            all.push(current_core_info);
        }
    }
    print_all_cores(&all);

    //TODO: test mem behavior
    let meminfo = Meminfo::current();
    let current_mem_info = to_mem_info(&meminfo);
    print_mem_info(&current_mem_info);

    //TODO: test disk behaivor

    Ok(())
}
