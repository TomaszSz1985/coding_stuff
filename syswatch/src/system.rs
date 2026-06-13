use crate::app::App;
use sysinfo::System;

pub fn update_stats(app: &mut App, sys: &mut System) {
    app.cpu_usage = sys.global_cpu_usage();
    app.ram_used = sys.used_memory() / 1024 / 1024;
    app.ram_total = sys.total_memory() / 1024 / 1024;
    app.processes = sys
        .processes()
        .iter()
        .take(10)
        .map(|(pid, process)| format!("{} {}", process.name().to_string_lossy(), pid.as_u32()))
        .collect()
}
