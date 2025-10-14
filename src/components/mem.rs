use crate::print_opt;
use procfs::{Meminfo, ProcError};

#[derive(Debug, Clone)]
pub struct MemInfo {
    total: u64,
    free: u64,
    availalbe: Option<u64>,
    mem_per_cpu: Option<u64>,
}

pub fn to_mem_info(meminfo: &Result<Meminfo, ProcError>) -> MemInfo {
    let meminfo = meminfo.as_ref();
    MemInfo {
        total: meminfo.unwrap().mem_total,
        free: meminfo.unwrap().mem_free,
        availalbe: meminfo.unwrap().mem_available,
        mem_per_cpu: meminfo.unwrap().per_cpu,
    }
}

pub fn print_mem_info(meminfo: &MemInfo) {
    println!("Total Memory: {}", meminfo.total);
    println!("Free Memory: {}", meminfo.free);
    print_opt!("Available Memory", meminfo.availalbe);
    print_opt!("Memory per CPU", meminfo.mem_per_cpu);
}
