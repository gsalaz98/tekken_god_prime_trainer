use sysinfo;
use sysinfo::{ProcessExt, SystemExt};

/// Gets the Tekken 7 process ID
pub fn get_pid() -> Result<i32, &'static str> {
    let mut system = sysinfo::System::new();
    let mut tekken_pid = None;

    // Get all currently running processes
    system.refresh_all();

    for (pid, proc_) in system.get_process_list() {
        if proc_.name() == crate::EXECUTABLE_NAME {
            tekken_pid = Some(pid);
        }
    }

    tekken_pid
        .map(|pid| *pid)
        .ok_or("Tekken is not running. Please launch Tekken then run this again")
}