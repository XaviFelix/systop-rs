use crate::models::data_type_parse::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CoreInfo {
    idx: usize,
    processor: Option<u32>,
    mhz: Option<f32>,
    core_id: Option<u32>,
    cache_kb: Option<u32>,
    model_name: Option<String>,
    vendor_id: Option<String>,
}

//NOTE: My "constructor"""
pub fn to_core_info(idx: usize, map: &HashMap<&str, &str>) -> CoreInfo {
    let cache_kb = parse_u32(map, "cache size");

    CoreInfo {
        idx,
        processor: parse_u32_v2(map, "processor"),
        mhz: parse_f32(map, "cpu MHz"),
        core_id: parse_u32_v2(map, "core id"),
        cache_kb,
        model_name: parse_string(map, "model name"),
        vendor_id: parse_string(map, "vendor_id"),
    }
}

#[macro_export]
macro_rules! print_opt {
    ($label:literal, $opt:expr) => {
        if let Some(v) = &$opt {
            println!("{:<12}: {}", $label, v);
        }
    };
}

pub fn print_all_cores(all: &Vec<CoreInfo>) {
    for c in all {
        println!("CPU #{:02}: ", c.idx);
        print_opt!("Processor: ", c.processor);
        print_opt!("Frequency (MHz): ", c.mhz);
        print_opt!("Core ID: ", c.core_id);
        print_opt!("Cache (KB): ", c.cache_kb);
        print_opt!("Model: ", c.model_name);
        print_opt!("Vendor: ", c.vendor_id);
        println!();
    }
}
