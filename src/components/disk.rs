use blockdev::{BlockDevice, get_devices};
use procfs::DiskStat;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
pub struct DiskInfo {
    pub name: String,
    pub size: String,
    pub disk_type: String,
    pub mnt_point: Vec<Option<String>>,
    pub partitions: Option<Vec<BlockDevice>>,
    pub reads: u64,
    pub sectors_read: u64,
    pub sectors_written: u64,
    pub time_reading: u64,
    pub writes: u64,
    pub writes_merged: u64,
    pub time_writing: u64,
}

impl DiskInfo {
    pub fn is_parent(&self) -> bool {
        self.disk_type == "disk" || self.partitions.as_ref().map_or(false, |v| !v.is_empty())
    }
}

impl Display for DiskInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Device: {}  (type: {}, size: {}, read: {}, sectors read: {}, writes: {}, writes merged: {})",
            self.name,
            self.disk_type,
            self.size,
            self.reads,
            self.sectors_read,
            self.writes,
            self.writes_merged
        )?;
        if let Some(parts) = self.partitions.as_ref() {
            for p in parts {
                writeln!(f, "\t|\n\t----Partition: {}", p.name)?;
            }
        }
        Ok(())
    }
}

pub fn build_device(stat: DiskStat) -> io::Result<Option<DiskInfo>> {
    let devices = get_devices().map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    fn to_info(d: BlockDevice, s: &DiskStat) -> DiskInfo {
        DiskInfo {
            name: d.name,
            size: d.size,
            disk_type: d.device_type,
            mnt_point: d.mountpoints,
            partitions: d.children,
            reads: s.reads,
            sectors_read: s.sectors_read,
            sectors_written: s.sectors_written,
            time_reading: s.time_reading,
            writes: s.writes,
            writes_merged: s.writes_merged,
            time_writing: s.time_writing,
        }
    }

    for mut d in devices {
        if d.name == stat.name {
            return Ok(Some(to_info(d, &stat))); // parent match
        }
        if let Some(children) = d.children.take() {
            if let Some(child) = children.into_iter().find(|c| c.name == stat.name) {
                return Ok(Some(to_info(child, &stat))); // partition match
            }
        }
    }
    Ok(None)
}

pub fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
