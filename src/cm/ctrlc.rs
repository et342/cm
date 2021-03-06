static mut CTRLC: bool = false;

// TODO(#181): ctrlc module is not implemented for windows

#[cfg(unix)]
extern "C" fn callback(_signum: i32) {
    unsafe {
        CTRLC = true;
    }
}

pub fn init() {
    if cfg!(unix) {
        unsafe {
            // TODO(#182): Explore portability issues of using signal(2)
            libc::signal(libc::SIGINT, callback as libc::sighandler_t);
        }
    }
}

pub fn poll() -> bool {
    if cfg!(unix) {
        unsafe {
            let result = CTRLC;
            if CTRLC {
                CTRLC = false;
            }
            result
        }
    } else {
        false
    }
}
