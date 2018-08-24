extern crate backtrace;
extern crate libc;
extern crate perf_events_sys;

use perf_events_sys::*;
use std::io;
use std::mem;
use std::ptr;
use std::slice;
use std::sync::mpsc;
use std::thread;

use buffer::EventMmap;

mod buffer;

fn main() {
    unsafe {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let tid = libc::syscall(186);
            tx.send(tid).unwrap();
            println!("{:?}", backtrace::Backtrace::new());
            loop {
                std::sync::atomic::spin_loop_hint();
                std::sync::atomic::spin_loop_hint();
                std::sync::atomic::spin_loop_hint();
                std::sync::atomic::spin_loop_hint();
                std::sync::atomic::spin_loop_hint();
            }
        });
        let tid = rx.recv().unwrap() as i32;

        let mut attr = mem::zeroed::<perf_event_attr>();
        attr.type_ = PERF_TYPE_SOFTWARE;
        attr.size = mem::size_of::<perf_event_attr>() as u32;
        attr.config = PERF_COUNT_SW_CPU_CLOCK;
        attr.union1.sample_freq = 100;
        attr.sample_type = PERF_SAMPLE_CALLCHAIN;
        attr.set_exclude_kernel(true);
        attr.set_freq(true);
        attr.union2.wakeup_events = 10;

        let fd = libc::syscall(298, &attr, tid, -1, -1, 0) as libc::c_int;
        if fd == -1 {
            panic!("{}", io::Error::last_os_error());
        }

        let mut mmap = EventMmap::new(fd, 32).unwrap();

        let mut readfds = mem::uninitialized();
        libc::FD_ZERO(&mut readfds);
        libc::FD_SET(fd, &mut readfds);

        let ret = libc::select(
            fd + 1,
            &mut readfds,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
        if ret < 0 {
            panic!("{}", io::Error::last_os_error());
        }
        assert!(libc::FD_ISSET(fd, &mut readfds));

        let mut buf = vec![];
        let mut events = mmap.iter(&mut buf);

        while let Some(event) = events.next() {
            println!("type {}", event.type_);

            if event.type_ == PERF_RECORD_SAMPLE {
                assert!(event.data.as_ptr() as usize % mem::align_of::<u64>() == 0);
                let data = slice::from_raw_parts(
                    event.data.as_ptr() as *const u64,
                    event.data.len() / mem::size_of::<u64>(),
                );
                assert_eq!(data[0], data.len() as u64 - 1);

                for &ip in &data[1..] {
                    if ip > PERF_CONTEXT_MAX {
                        continue;
                    }

                    let ip = ip - 1;
                    println!("{:#x}", ip);
                    backtrace::resolve(ip as *mut _, |symbol| {
                        println!(
                            "   {:?} {:?} {:?}",
                            symbol.name(),
                            symbol.filename(),
                            symbol.lineno()
                        );
                    });
                }
            }
        }
    }
}
