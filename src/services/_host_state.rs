use rocket::serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
pub struct HostState {
    disk_total: u64,
    disk_free: u64,

    cpu_load_one: f64,
    cpu_load_five: f64,
    cpu_load_fifteen: f64,

    mem_total: u64,
    mem_free: u64,
    mem_avail: u64,
    mem_buffers: u64,
    mem_cached: u64,
    mem_swap_total: u64,
    mem_swap_free: u64,
}

pub fn new() -> HostState {
    let x = HostState {
        disk_total: 0,
        disk_free: 0,
        cpu_load_one: 0.0,
        cpu_load_five: 0.0,
        cpu_load_fifteen: 0.0,
        mem_total: 0,
        mem_free: 0,
        mem_avail: 0,
        mem_buffers: 0,
        mem_cached: 0,
        mem_swap_total: 0,
        mem_swap_free: 0,
    };
    x
}

impl HostState {
    pub fn refresh(&mut self) {
        if let Ok(t) = sys_info::disk_info() {
            self.disk_total = t.total;
            self.disk_free = t.free;
        };

        if let Ok(c) = sys_info::loadavg() {
            self.cpu_load_one = c.one;
            self.cpu_load_five = c.five;
            self.cpu_load_fifteen = c.fifteen;
        };

        if let Ok(m) = sys_info::mem_info() {
            self.mem_total = m.total;
            self.mem_free = m.free;
            self.mem_avail = m.avail;
            self.mem_buffers = m.buffers;
            self.mem_cached = m.cached;
            self.mem_swap_total = m.swap_total;
            self.mem_swap_free = m.swap_free;
        };
    }
}
