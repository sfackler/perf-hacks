use libc;
use std::io;
use std::ptr;

pub struct EventBuffer {
    ptr: *mut libc::c_void,
    len: usize,
    mask: u64,
}

impl Drop for EventBuffer {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.ptr, self.len);
        }
    }
}

impl EventBuffer {
    pub fn new(fd: libc::c_int, data_pages: usize) -> io::Result<EventBuffer> {
        unsafe {
            assert_eq!(data_pages.count_ones(), 1);
            let page_size = libc::sysconf(libc::_SC_PAGESIZE) as usize;
            let len = (data_pages + 1) * page_size;

            let ptr = libc::mmap(ptr::null_mut(), len, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED, fd, 0);
            if ptr == libc::MAP_FAILED {
                return Err(io::Error::last_os_error());
            }

            let mask = data_pages * page_size - 1;

            Ok(EventBuffer {
                ptr,
                len,
                mask: mask as u64,
            })
        }
    }

    pub fn iter<'a>(&'a mut self) -> Events<'a> {
        panic!()
    }
}

pub struct Events<'a> {
    buf: &'a mut EventBuffer,
}
