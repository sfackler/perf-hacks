pub const PERF_TYPE_HARDWARE: u32 = 0;
pub const PERF_TYPE_SOFTWARE: u32 = 1;
pub const PERF_TYPE_TRACEPOINT: u32 = 2;
pub const PERF_TYPE_HW_CACHE: u32 = 3;
pub const PERF_TYPE_RAW: u32 = 4;

pub const PERF_COUNT_HW_CPU_CYCLES: u64 = 0;
pub const PERF_COUNT_HW_INSTRUCTIONS: u64 = 1;
pub const PERF_COUNT_HW_CACHE_REFERENCES: u64 = 2;
pub const PERF_COUNT_HW_CACHE_MISSES: u64 = 3;
pub const PERF_COUNT_HW_BRANCH_INSTRUCTIONS: u64 = 4;
pub const PERF_COUNT_HW_BRANCH_MISSES: u64 = 5;
pub const PERF_COUNT_HW_BUS_CYCLES: u64 = 6;
pub const PERF_COUNT_HW_STALLED_CYCLES_FRONTEND: u64 = 7;
pub const PERF_COUNT_HW_STALLED_CYCLES_BACKEND: u64 = 8;
pub const PERF_COUNT_HW_REF_CPU_CYCLES: u64 = 9;

pub const PERF_COUNT_HW_CACHE_L1D: u64 = 0;
pub const PERF_COUNT_HW_CACHE_L1I: u64 = 1;
pub const PERF_COUNT_HW_CACHE_LL: u64 = 2;
pub const PERF_COUNT_HW_CACHE_DTLB: u64 = 3;
pub const PERF_COUNT_HW_CACHE_ITLB: u64 = 4;
pub const PERF_COUNT_HW_CACHE_BPU: u64 = 5;
pub const PERF_COUNT_HW_CACHE_NODE: u64 = 6;

pub const PERF_COUNT_HW_CACHE_OP_READ: u64 = 0;
pub const PERF_COUNT_HW_CACHE_OP_WRITE: u64 = 1;
pub const PERF_COUNT_HW_CACHE_OP_PREFETCH: u64 = 2;

pub const PERF_COUNT_HW_CACHE_RESULT_ACCESS: u64 = 0;
pub const PERF_COUNT_HW_CACHE_RESULT_MISS: u64 = 1;

pub const PERF_COUNT_SW_CPU_CLOCK: u64 = 0;
pub const PERF_COUNT_SW_TASK_CLOCK: u64 = 1;
pub const PERF_COUNT_SW_PAGE_FAULTS: u64 = 2;
pub const PERF_COUNT_SW_CONTEXT_SWITCHES: u64 = 3;
pub const PERF_COUNT_SW_CPU_MIGRATIONS: u64 = 4;
pub const PERF_COUNT_SW_PAGE_FAULTS_MIN: u64 = 5;
pub const PERF_COUNT_SW_PAGE_FAULTS_MAX: u64 = 6;
pub const PERF_COUNT_SW_ALIGNMENT_FAULTS: u64 = 7;
pub const PERF_COUNT_SW_EMULATION_FAULTS: u64 = 8;
pub const PERF_COUNT_SW_DUMMY: u64 = 9;
pub const PERF_COUNT_SW_BPF_OUTPUT: u64 = 10;

pub const PERF_SAMPLE_IP: u64 = 1 << 0;
pub const PERF_SAMPLE_TID: u64 = 1 << 1;
pub const PERF_SAMPLE_TIME: u64 = 1 << 2;
pub const PERF_SAMPLE_ADDR: u64 = 1 << 3;
pub const PERF_SAMPLE_READ: u64 = 1 << 4;
pub const PERF_SAMPLE_CALLCHAIN: u64 = 1 << 5;
pub const PERF_SAMPLE_ID: u64 = 1 << 6;
pub const PERF_SAMPLE_CPU: u64 = 1 << 7;
pub const PERF_SAMPLE_PERIOD: u64 = 1 << 8;
pub const PERF_SAMPLE_STREAM_ID: u64 = 1 << 9;
pub const PERF_SAMPLE_RAW: u64 = 1 << 10;
pub const PERF_SAMPLE_BRANCH_STACK: u64 = 1 << 11;
pub const PERF_SAMPLE_REGS_USER: u64 = 1 << 12;
pub const PERF_SAMPLE_STACK_USER: u64 = 1 << 13;
pub const PERF_SAMPLE_WEIGHT: u64 = 1 << 14;
pub const PERF_SAMPLE_DATA_SRC: u64 = 1 << 15;
pub const PERF_SAMPLE_IDENTIFIER: u64 = 1 << 16;
pub const PERF_SAMPLE_TRANSACTION: u64 = 1 << 17;
pub const PERF_SAMPLE_REGS_INTR: u64 = 1 << 18;
pub const PERF_SAMPLE_PHYS_ADDR: u64 = 1 << 19;

pub const PERF_SAMPLE_BRANCH_USER_SHIFT: u32 = 0;
pub const PERF_SAMPLE_BRANCH_KERNEL_SHIFT: u32 = 1;
pub const PERF_SAMPLE_BRANCH_HV_SHIFT: u32 = 2;

pub const PERF_SAMPLE_BRANCH_ANY_SHIFT: u32 = 3;
pub const PERF_SAMPLE_BRANCH_ANY_CALL_SHIFT: u32 = 4;
pub const PERF_SAMPLE_BRANCH_ANY_RETURN_SHIFT: u32 = 5;
pub const PERF_SAMPLE_BRANCH_IND_CALL_SHIFT: u32 = 6;
pub const PERF_SAMPLE_BRANCH_ABORT_TX_SHIFT: u32 = 7;
pub const PERF_SAMPLE_BRANCH_IN_TX_SHIFT: u32 = 8;
pub const PERF_SAMPLE_BRANCH_NO_TX_SHIFT: u32 = 9;
pub const PERF_SAMPLE_BRANCH_COND_SHIFT: u32 = 10;

pub const PERF_SAMPLE_BRANCH_CALL_STACK_SHIFT: u32 = 11;
pub const PERF_SAMPLE_BRANCH_IND_JUMP_SHIFT: u32 = 12;
pub const PERF_SAMPLE_BRANCH_CALL_SHIFT: u32 = 13;

pub const PERF_SAMPLE_BRANCH_NO_FLAGS_SHIFT: u32 = 14;
pub const PERF_SAMPLE_BRANCH_NO_CYCLES_SHIFT: u32 = 15;

pub const PERF_SAMPLE_BRANCH_TYPE_SAVE_SHIFT: u32 = 16;

pub const PERF_SAMPLE_BRANCH_USER: u64 = 1 << PERF_SAMPLE_BRANCH_USER_SHIFT;
pub const PERF_SAMPLE_BRANCH_KERNEL: u64 = 1 << PERF_SAMPLE_BRANCH_KERNEL_SHIFT;
pub const PERF_SAMPLE_BRANCH_HV: u64 = 1 << PERF_SAMPLE_BRANCH_HV_SHIFT;

pub const PERF_SAMPLE_BRANCH_ANY: u64 = 1 << PERF_SAMPLE_BRANCH_ANY_SHIFT;
pub const PERF_SAMPLE_BRANCH_ANY_CALL: u64 = 1 << PERF_SAMPLE_BRANCH_ANY_CALL_SHIFT;
pub const PERF_SAMPLE_BRANCH_ANY_RETURN: u64 = 1 << PERF_SAMPLE_BRANCH_ANY_RETURN_SHIFT;
pub const PERF_SAMPLE_BRANCH_IND_CALL: u64 = 1 << PERF_SAMPLE_BRANCH_IND_CALL_SHIFT;
pub const PERF_SAMPLE_BRANCH_ABORT_TX: u64 = 1 << PERF_SAMPLE_BRANCH_ABORT_TX_SHIFT;
pub const PERF_SAMPLE_BRANCH_IN_TX: u64 = 1 << PERF_SAMPLE_BRANCH_IN_TX_SHIFT;
pub const PERF_SAMPLE_BRANCH_NO_TX: u64 = 1 << PERF_SAMPLE_BRANCH_NO_TX_SHIFT;
pub const PERF_SAMPLE_BRANCH_COND: u64 = 1 << PERF_SAMPLE_BRANCH_COND_SHIFT;

pub const PERF_SAMPLE_BRANCH_NO_FLAGS: u64 = 1 << PERF_SAMPLE_BRANCH_NO_FLAGS_SHIFT;
pub const PERF_SAMPLE_BRANCH_NO_CYCLES: u64 = 1 << PERF_SAMPLE_BRANCH_NO_CYCLES_SHIFT;

pub const PERF_SAMPLE_BRANCH_TYPE_SAVE: u64 = 1 << PERF_SAMPLE_BRANCH_TYPE_SAVE_SHIFT;

pub const PERF_BR_UNKNOWN: u32 = 0;
pub const PERF_BR_COND: u32 = 1;
pub const PERF_BR_UNCOND: u32 = 2;
pub const PERF_BR_IND: u32 = 3;
pub const PERF_BR_CALL: u32 = 4;
pub const PERF_BR_IND_CALL: u32 = 5;
pub const PERF_BR_RET: u32 = 6;
pub const PERF_BR_SYSCALL: u32 = 7;
pub const PERF_BR_SYSRET: u32 = 8;
pub const PERF_BR_COND_CALL: u32 = 9;
pub const PERF_BR_COND_RET: u32 = 10;

pub const PERF_SAMPLE_BRANCH_PLM_ALL: u64 =
    PERF_SAMPLE_BRANCH_USER | PERF_SAMPLE_BRANCH_KERNEL | PERF_SAMPLE_BRANCH_HV;

pub const PERF_SAMPLE_REGS_ABI_NONE: u64 = 0;
pub const PERF_SAMPLE_REGS_ABI_32: u64 = 1;
pub const PERF_SAMPLE_REGS_ABI_64: u64 = 2;

pub const PERF_TXN_ELISION: u64 = 1 << 0;
pub const PERF_TXN_TRANSACTION: u64 = 1 << 1;
pub const PERF_TXN_SYNC: u64 = 1 << 2;
pub const PERF_TXN_ASYNC: u64 = 1 << 3;
pub const PERF_TXN_RETRY: u64 = 1 << 4;
pub const PERF_TXN_CONFLICT: u64 = 1 << 5;
pub const PERF_TXN_CAPACITY_WRITE: u64 = 1 << 6;
pub const PERF_TXN_CAPACITY_READ: u64 = 1 << 7;

pub const PERF_TXN_ABORT_MASK: u64 = 0xffffffff << 32;
pub const PERF_TXN_ABORT_SHIFT: u32 = 32;

pub const PERF_FORMAT_TOTAL_TIME_ENABLED: u32 = 1 << 0;
pub const PERF_FORMAT_TOTAL_TIME_RUNNINT: u32 = 1 << 1;
pub const PERF_FORMAT_ID: u32 = 1 << 2;
pub const PERF_FORMAT_GROUP: u32 = 1 << 3;

pub const PERF_ATTR_SIZE_VER0: u32 = 64;
pub const PERF_ATTR_SIZE_VER1: u32 = 72;
pub const PERF_ATTR_SIZE_VER2: u32 = 80;
pub const PERF_ATTR_SIZE_VER3: u32 = 96;
pub const PERF_ATTR_SIZE_VER4: u32 = 104;
pub const PERF_ATTR_SIZE_VER5: u32 = 112;

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
    pub union1: perf_event_attr_union1,
    pub sample_type: u64,
    pub read_format: u64,
    flags: u64,
    pub union2: perf_event_attr_union2,
    pub bp_type: u32,
    pub union3: perf_event_attr_union3,
    pub union4: perf_event_attr_union4,
    pub branch_sample_type: u64,
    pub sample_regs_use: u64,
    pub sample_stack_user: u32,
    pub clockid: i32,
    pub sample_regs_intr: u64,
    pub aux_watermark: u32,
    pub sample_max_stack: u16,
    pub __reserved_2: u16,
}

impl perf_event_attr {
    #[inline]
    pub fn set_disabled(&mut self, disabled: bool) {
        self.set_flag(0, disabled)
    }

    #[inline]
    pub fn set_inherit(&mut self, inherit: bool) {
        self.set_flag(1, inherit)
    }

    #[inline]
    pub fn set_pinned(&mut self, pinned: bool) {
        self.set_flag(2, pinned);
    }

    #[inline]
    pub fn set_exclusive(&mut self, exclusive: bool) {
        self.set_flag(3, exclusive);
    }

    #[inline]
    pub fn set_exclude_user(&mut self, exclude_user: bool) {
        self.set_flag(4, exclude_user);
    }

    #[inline]
    pub fn set_exclude_kernel(&mut self, exclude_kernel: bool) {
        self.set_flag(5, exclude_kernel);
    }

    #[inline]
    pub fn set_exclude_hv(&mut self, exclude_hv: bool) {
        self.set_flag(6, exclude_hv);
    }

    #[inline]
    pub fn set_exclude_idle(&mut self, exclude_idle: bool) {
        self.set_flag(7, exclude_idle);
    }

    #[inline]
    pub fn set_mmap(&mut self, mmap: bool) {
        self.set_flag(8, mmap);
    }

    #[inline]
    pub fn set_comm(&mut self, comm: bool) {
        self.set_flag(9, comm);
    }

    #[inline]
    pub fn set_freq(&mut self, freq: bool) {
        self.set_flag(10, freq);
    }

    #[inline]
    pub fn set_inherit_stat(&mut self, inherit_stat: bool) {
        self.set_flag(11, inherit_stat);
    }

    #[inline]
    pub fn set_enable_on_exec(&mut self, enable_on_exec: bool) {
        self.set_flag(12, enable_on_exec);
    }

    #[inline]
    pub fn set_task(&mut self, task: bool) {
        self.set_flag(13, task);
    }

    #[inline]
    pub fn set_watermark(&mut self, watermark: bool) {
        self.set_flag(14, watermark);
    }

    #[inline]
    pub fn set_precise_ip(&mut self, precise_ip: u8) {
        assert!(precise_ip < 4);
        self.flags &= !(0b11 << 15);
        self.flags |= (precise_ip as u64) << 15;
    }

    #[inline]
    pub fn set_mmap_data(&mut self, mmap_data: bool) {
        self.set_flag(17, mmap_data);
    }

    #[inline]
    pub fn set_sample_id_all(&mut self, sample_id_all: bool) {
        self.set_flag(18, sample_id_all);
    }

    #[inline]
    pub fn set_exclude_host(&mut self, exclude_host: bool) {
        self.set_flag(19, exclude_host);
    }

    #[inline]
    pub fn set_exclude_guest(&mut self, exclude_guest: bool) {
        self.set_flag(20, exclude_guest);
    }

    #[inline]
    pub fn set_exclude_callchain_kernel(&mut self, exclude_callchain_kernel: bool) {
        self.set_flag(21, exclude_callchain_kernel);
    }

    #[inline]
    pub fn set_exclude_callchain_user(&mut self, exclude_callchain_user: bool) {
        self.set_flag(22, exclude_callchain_user);
    }

    #[inline]
    pub fn set_mmap2(&mut self, mmap2: bool) {
        self.set_flag(23, mmap2);
    }

    #[inline]
    pub fn set_comm_exec(&mut self, comm_exec: bool) {
        self.set_flag(24, comm_exec);
    }

    #[inline]
    pub fn set_use_clockid(&mut self, use_clockid: bool) {
        self.set_flag(25, use_clockid);
    }

    #[inline]
    pub fn set_context_switch(&mut self, context_switch: bool) {
        self.set_flag(26, context_switch);
    }

    #[inline]
    pub fn set_write_backward(&mut self, write_backward: bool) {
        self.set_flag(27, write_backward);
    }

    #[inline]
    pub fn set_namespaces(&mut self, namespaces: bool) {
        self.set_flag(28, namespaces);
    }

    #[inline]
    fn set_flag(&mut self, bit: u32, value: bool) {
        let mask = 1 << bit;
        if value {
            self.flags |= mask;
        } else {
            self.flags &= !mask;
        }
    }
}

#[repr(C)]
pub union perf_event_attr_union1 {
    pub sample_period: u64,
    pub sample_freq: u64,
}

#[repr(C)]
pub union perf_event_attr_union2 {
    pub wakeup_events: u32,
    pub wakeup_watermark: u32,
}

#[repr(C)]
pub union perf_event_attr_union3 {
    pub bp_addr: u64,
    pub kprobe_func: u64,
    pub uprobe_func: u64,
    pub config1: u64,
}

#[repr(C)]
pub union perf_event_attr_union4 {
    pub bp_len: u64,
    pub kprobe_addr: u64,
    pub probe_offset: u64,
    pub config2: u64,
}

#[repr(C)]
pub struct perf_event_header {
    pub type_: u32,
    pub misc: u16,
    pub size: u16,
}

pub const PERF_RECORD_MMAP: u32 = 1;
pub const PERF_RECORD_LOST: u32 = 2;
pub const PERF_RECORD_COMM: u32 = 3;
pub const PERF_RECORD_EXIT: u32 = 4;
pub const PERF_RECORD_THROTTLE: u32 = 5;
pub const PERF_RECORD_UNTHROTTLE: u32 = 6;
pub const PERF_RECORD_FORK: u32 = 7;
pub const PERF_RECORD_READ: u32 = 8;
pub const PERF_RECORD_SAMPLE: u32 = 9;
pub const PREF_RECORD_MMAP2: u32 = 10;
pub const PERF_RECORD_AUX: u32 = 11;
pub const PERF_RECORD_ITRACE_START: u32 = 12;
pub const PERF_RECORD_LOST_SAMPLES: u32 = 13;
pub const PERF_RECORD_SWITCH: u32 = 14;
pub const PERF_RECORD_SWITCH_CPU_WIDE: u32 = 15;
pub const PERF_RECORD_NAMESPACES: u32 = 16;
