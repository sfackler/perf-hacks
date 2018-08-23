extern crate libc;
extern crate perf_events_sys;
extern crate backtrace;

use perf_events_sys::*;
use std::arch::x86_64::{_mm_lfence, _mm_mfence};
use std::io;
use std::mem;
use std::ptr;
use std::sync::mpsc;
use std::thread;

fn main() {
    unsafe {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let tid = libc::syscall(186);
            tx.send(tid).unwrap();
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

        let ptr = libc::mmap(
            ptr::null_mut(),
            4096 + 4096 * 32,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd,
            0,
        );
        if ptr == libc::MAP_FAILED {
            panic!("{}", io::Error::last_os_error());
        }
        let ptr = ptr as *mut libc::c_char;

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

        let data_head_ptr = ptr.add(1024) as *const u64;
        let data_tail_ptr = data_head_ptr.add(1);

        let data_head = ptr::read_volatile(data_head_ptr);
        _mm_lfence();

        let mut start = ptr.add(4096);
        let end = start.add(data_head as usize);

        while start < end {
            let mut entry = start;
            let header = ptr::read_volatile(entry as *mut perf_event_header);
            entry = entry.add(mem::size_of::<perf_event_header>());

            start = start.add(header.size as usize);

            println!("type {} size {}", header.type_, header.size);

            if header.type_ == PERF_RECORD_SAMPLE {
                let nr = ptr::read_volatile(entry as *mut u64);
                entry = entry.add(mem::size_of::<u64>());
                assert_eq!(4 + 2 + 2 + 8 + 8 * (nr as u16), header.size);

                for _ in 0..nr {
                    let ip = ptr::read_volatile(entry as *mut u64);
                    entry = entry.add(mem::size_of::<u64>());

                    if ip > PERF_CONTEXT_MAX {
                        continue;
                    }

                    println!("{:#x}", ip);
                    backtrace::resolve(ip as *mut _, |symbol| {
                        println!("   {:?} {:?} {:?}", symbol.name(), symbol.filename(), symbol.lineno());
                    });
                }
            }
        }
    }
}
