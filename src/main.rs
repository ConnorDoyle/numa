extern crate libc;
use libc::c_int;
use std::mem;

fn main() {
    #[link(name="numa")]
    extern {
        fn numa_num_configured_nodes() -> c_int;
        fn numa_num_task_cpus() -> c_int;
    }

    let num_nodes: i32 = unsafe {
        mem::transmute(numa_num_configured_nodes())
    };
    let num_cpus: i32 = unsafe {
        mem::transmute(numa_num_task_cpus())
    };
    println!("numa_num_configured_nodes: {}", num_nodes);
    println!("numa_num_task_cpus: {}", num_cpus);
}
