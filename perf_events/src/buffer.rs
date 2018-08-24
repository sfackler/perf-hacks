use libc;
use perf_events_sys::*;
use std::io;
use std::mem;
use std::ptr;
use std::slice;
use std::arch::x86_64::{_mm_mfence, _mm_lfence};

pub struct EventMmap {
    ptr: *mut u8,
    len: usize,
    base_offset: usize,
    mask: u64,
    tail: u64,
}

impl Drop for EventMmap {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.ptr as *mut libc::c_void, self.len);
        }
    }
}

impl EventMmap {
    pub fn new(fd: libc::c_int, data_pages: usize) -> io::Result<EventMmap> {
        unsafe {
            assert_eq!(data_pages.count_ones(), 1);
            let page_size = libc::sysconf(libc::_SC_PAGESIZE) as usize;
            let len = (data_pages + 1) * page_size;

            let ptr = libc::mmap(
                ptr::null_mut(),
                len,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                fd,
                0,
            );
            if ptr == libc::MAP_FAILED {
                return Err(io::Error::last_os_error());
            }

            let mask = data_pages * page_size - 1;

            Ok(EventMmap {
                ptr: ptr as *mut u8,
                len,
                base_offset: page_size,
                mask: mask as u64,
                tail: 0,
            })
        }
    }

    unsafe fn head(&self) -> u64 {
        // This is dubious, but matches up with what perf itself does. It should really be an
        // atomic acquire load but we need AtomicU64 first.
        let head = ptr::read_volatile(self.ptr.add(1024) as *const u64);
        _mm_lfence();
        head
    }

    unsafe fn write_tail(&self) {
        // Again, this should be an atomic store.
        _mm_mfence();
        ptr::write_volatile(self.ptr.add(1024 + 8) as *mut u64, self.tail);
    }

    pub fn iter<'a>(&'a mut self, buf: &'a mut Vec<u8>) -> Events<'a> {
        unsafe {
            Events {
                head: self.head(),
                mmap: self,
                buf,
            }
        }
    }
}

pub struct Event<'a> {
    pub type_: u32,
    pub misc: u16,
    pub data: &'a [u8],
}

pub struct Events<'a> {
    mmap: &'a mut EventMmap,
    buf: &'a mut Vec<u8>,
    head: u64,
}

impl<'a> Drop for Events<'a> {
    fn drop(&mut self) {
        unsafe {
            self.mmap.write_tail();
        }
    }
}

impl<'a> Events<'a> {
    pub fn next<'b>(&'b mut self) -> Option<Event<'b>> {
        unsafe {
            if self.mmap.tail >= self.head {
                return None;
            }

            let base_ptr = self.mmap.ptr.add(self.mmap.base_offset);
            let end_ptr = self.mmap.ptr.add(self.mmap.len);

            let entry_ptr = base_ptr.add((self.mmap.tail & self.mmap.mask) as usize);

            // entries are 8 byte aligned and the header is 8 bytes so this will never be wrapped.
            let header = ptr::read(entry_ptr as *const perf_event_header);
            let header_size = mem::size_of::<perf_event_header>();
            assert!(header.size as usize > header_size);

            let data_len = header.size as usize - header_size;
            let data_ptr = entry_ptr.add(header_size);

            let data = if data_ptr.wrapping_add(data_len) > end_ptr {
                self.buf.clear();
                self.buf.reserve(data_len);
                let first_len = end_ptr as usize - data_ptr as usize;
                self.buf.extend_from_slice(slice::from_raw_parts(data_ptr, first_len));
                let second_len = data_len - first_len;
                self.buf.extend_from_slice(slice::from_raw_parts(base_ptr, second_len));
                &self.buf
            } else {
                slice::from_raw_parts(
                    entry_ptr.add(header_size),
                    header.size as usize - header_size,
                )
            };

            self.mmap.tail += header.size as u64;

            Some(Event {
                type_: header.type_,
                misc: header.misc,
                data,
            })
        }
    }
}
