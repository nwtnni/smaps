use bitflags::bitflags;

mod parse;

pub use parse::Parser;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mapping {
    pub start: usize,
    pub end: usize,
    pub permissions: Permissions,
    pub offset: usize,
    pub device: Device,
    pub inode: usize,
    pub path: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Usage {
    pub size: usize,
    pub kernel_page_size: usize,
    pub mmu_page_size: usize,
    pub rss: usize,
    pub pss: usize,
    pub pss_dirty: usize,
    pub shared_clean: usize,
    pub shared_dirty: usize,
    pub private_clean: usize,
    pub private_dirty: usize,
    pub referenced: usize,
    pub anonymous: usize,
    pub ksm: usize,
    pub lazy_free: usize,
    pub anon_huge_pages: usize,
    pub shmem_huge_pages: usize,
    pub shmem_pmd_mapped: usize,
    pub file_pmd_mapped: usize,
    pub shared_hugetlb: usize,
    pub private_hugetlb: usize,
    pub swap: usize,
    pub swap_pss: usize,
    pub locked: usize,
    pub thp_eligible: bool,
    pub protection_key: Option<usize>,
    pub vm_flags: VmFlags,
}

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct Permissions: u8 {
        const X = 1 << 0;
        const W = 1 << 1;
        const R = 1 << 2;
        const S = 1 << 3;
        const P = 1 << 4;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Device {
    pub major: u32,
    pub minor: u32,
}

bitflags! {
    #[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
    pub struct VmFlags: u32 {
        /// readable
        const RD = 1 << 0;
        /// writable
        const WR = 1 << 1;
        /// executable
        const EX = 1 << 2;
        /// shared
        const SH = 1 << 3;
        /// may read
        const MR = 1 << 4;
        /// may write
        const MW = 1 << 5;
        /// may execute
        const ME = 1 << 6;
        /// may share
        const MS = 1 << 7;
        /// stack segment grows down
        const GD = 1 << 8;
        /// pure PFN range
        const PF = 1 << 9;
        /// disabled write to the mapped file
        const DW = 1 << 10;
        /// pages are locked in memory
        const LO = 1 << 11;
        /// memory mapped I/O area
        const IO = 1 << 12;
        /// sequential read advise provided
        const SR = 1 << 13;
        /// random read advise provided
        const RR = 1 << 14;
        /// do not copy area on fork
        const DC = 1 << 15;
        /// do not expand area on remapping
        const DE = 1 << 16;
        /// area is accountable
        const AC = 1 << 17;
        /// swap space is not reserved for the area
        const NR = 1 << 18;
        /// area uses huge tlb pages
        const HT = 1 << 19;
        /// perform synchronous page faults (since Linux 4.15)
        const SF = 1 << 20;
        /// non-linear mapping (removed in Linux 4.0)
        const NL = 1 << 21;
        /// architecture specific flag
        const AR = 1 << 22;
        /// wipe on fork (since Linux 4.14)
        const WF = 1 << 23;
        /// do not include area into core dump
        const DD = 1 << 24;
        /// soft-dirty flag (since Linux 3.13)
        const SD = 1 << 25;
        /// mixed map area
        const MM = 1 << 26;
        /// huge page advise flag
        const HG = 1 << 27;
        /// no-huge page advise flag
        const NH = 1 << 28;
        /// mergeable advise flag
        const MG = 1 << 29;
        /// userfaultfd missing pages tracking (since Linux 4.3)
        const UM = 1 << 30;
        /// userfaultfd wprotect pages tracking (since Linux 4.3)
        const UW = 1 << 31;
    }
}
