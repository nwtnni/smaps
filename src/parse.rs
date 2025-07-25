use core::iter;
use core::iter::Peekable;
use core::ops::BitOr;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use crate::Device;
use crate::Mapping;
use crate::Permissions;
use crate::Usage;
use crate::VmFlags;

pub struct Parser<R: BufRead, S> {
    iter: iter::Peekable<std::io::Lines<R>>,
    _state: S,
}

pub struct ParseMapping;
pub struct ParseUsage;

impl Parser<BufReader<File>, ParseMapping> {
    pub fn open(path: &Path) -> std::io::Result<Self> {
        File::open(path)
            .map(BufReader::new)
            .map(BufReader::lines)
            .map(Iterator::peekable)
            .map(|iter| Self {
                iter,
                _state: ParseMapping,
            })
    }
}

impl<R: BufRead> Parser<R, ParseMapping> {
    pub fn next(mut self) -> std::io::Result<(Parser<R, ParseUsage>, Option<Mapping>)> {
        let mapping = self
            .iter
            .next()
            .transpose()?
            .as_deref()
            .and_then(Mapping::parse);

        Ok((self.with_state(ParseUsage), mapping))
    }
}

impl<R: BufRead> Parser<R, ParseUsage> {
    pub fn next(mut self) -> std::io::Result<(Parser<R, ParseMapping>, Option<Usage>)> {
        let usage = Usage::parse(&mut self.iter)?;
        Ok((self.with_state(ParseMapping), usage))
    }

    pub fn skip(mut self) -> Parser<R, ParseMapping> {
        while self
            .iter
            .peek()
            .map(Result::as_ref)
            .is_some_and(|line| line.is_ok_and(|line| !line.contains('-')))
        {
            self.iter.next();
        }

        self.with_state(ParseMapping)
    }
}

impl<R: BufRead, S> Parser<R, S> {
    fn with_state<T>(self, state: T) -> Parser<R, T> {
        Parser {
            iter: self.iter,
            _state: state,
        }
    }
}

impl Usage {
    fn parse(
        iter: &mut Peekable<impl Iterator<Item = std::io::Result<String>>>,
    ) -> std::io::Result<Option<Self>> {
        let mut usage = Self::default();

        while let Some(line) =
            iter.next_if(|line| line.as_ref().is_ok_and(|line| !line.contains('-')))
        {
            let line = line?;

            if line.starts_with("VmFlags") {
                usage.vm_flags =
                    VmFlags::parse(line.trim_start_matches("VmFlags:").trim_ascii_start());
                continue;
            }

            let Some((key, value)) = Self::parse_line(&line) else {
                return Ok(None);
            };

            match key {
                "Size" => usage.size = value,
                "KernelPageSize" => usage.kernel_page_size = value,
                "MMUPageSize" => usage.mmu_page_size = value,
                "Rss" => usage.rss = value,
                "Pss" => usage.pss = value,
                "Pss_Dirty" => usage.pss_dirty = value,
                "Shared_Clean" => usage.shared_clean = value,
                "Shared_Dirty" => usage.shared_dirty = value,
                "Private_Clean" => usage.private_clean = value,
                "Private_Dirty" => usage.private_dirty = value,
                "Referenced" => usage.referenced = value,
                "Anonymous" => usage.anonymous = value,
                "KSM" => usage.ksm = value,
                "LazyFree" => usage.lazy_free = value,
                "AnonHugePages" => usage.anon_huge_pages = value,
                "ShmemPmdMapped" => usage.shmem_pmd_mapped = value,
                "FilePmdMapped" => usage.file_pmd_mapped = value,
                "Shared_Hugetlb" => usage.shared_hugetlb = value,
                "Private_Hugetlb" => usage.private_hugetlb = value,
                "Swap" => usage.swap = value,
                "SwapPss" => usage.swap_pss = value,
                "Locked" => usage.locked = value,
                "THPeligible" => usage.thp_eligible = value != 0,
                "ProtectionKey" => usage.protection_key = Some(value),
                key => panic!("Unrecognized key: {}", key),
            }
        }

        Ok(Some(usage))
    }

    fn parse_line(line: &str) -> Option<(&str, usize)> {
        let mut iter = line.split_ascii_whitespace();
        let key = iter.next()?.trim_end_matches(":");
        let value = iter.next()?;
        let unit = match iter.next() {
            Some("kB") => 10,
            Some("mB") => 20,
            Some("gB") => 30,
            Some("tB") => 40,
            Some(unit) => panic!("Unrecognized unit: {}", unit),
            None => 0,
        };

        match iter.next() {
            Some(_) => None,
            None => Some((key, value.parse::<usize>().ok()? << unit)),
        }
    }
}

impl VmFlags {
    fn parse(data: &str) -> Self {
        data.split_ascii_whitespace()
            .map(|flag| match flag {
                "rd" => Self::RD,
                "wr" => Self::WR,
                "ex" => Self::EX,
                "sh" => Self::SH,
                "mr" => Self::MR,
                "mw" => Self::MW,
                "me" => Self::ME,
                "ms" => Self::MS,
                "gd" => Self::GD,
                "pf" => Self::PF,
                "dw" => Self::DW,
                "lo" => Self::LO,
                "io" => Self::IO,
                "sr" => Self::SR,
                "rr" => Self::RR,
                "dc" => Self::DC,
                "de" => Self::DE,
                "ac" => Self::AC,
                "nr" => Self::NR,
                "ht" => Self::HT,
                "sf" => Self::SF,
                "nl" => Self::NL,
                "ar" => Self::AR,
                "wf" => Self::WF,
                "dd" => Self::DD,
                "sd" => Self::SD,
                "mm" => Self::MM,
                "hg" => Self::HG,
                "nh" => Self::NH,
                "mg" => Self::MG,
                "um" => Self::UM,
                "uw" => Self::UW,
                flag => panic!("Unrecognized VM flag: {}", flag),
            })
            .fold(VmFlags::empty(), BitOr::bitor)
    }
}

impl Mapping {
    fn parse(line: &str) -> Option<Self> {
        let mut iter = line.split_ascii_whitespace();
        let (start, end) = iter.next().unwrap().split_once('-')?;
        let permissions = iter.next().and_then(Permissions::parse)?;
        let offset = iter.next()?;
        let device = iter.next().and_then(Device::parse)?;
        let inode = iter.next()?;
        let path = iter.next();

        Some(Self {
            start: parse_hex(start)?,
            end: parse_hex(end)?,
            permissions,
            offset: parse_hex(offset)?,
            device,
            inode: inode.parse().ok()?,
            path: path.map(str::to_owned),
        })
    }
}

impl Permissions {
    fn parse(data: &str) -> Option<Self> {
        let data: [u8; 4] = data.as_bytes().try_into().ok()?;
        let read = match data[0] {
            b'-' => Permissions::empty(),
            b'r' => Permissions::R,
            _ => return None,
        };

        let write = match data[1] {
            b'-' => Permissions::empty(),
            b'w' => Permissions::W,
            _ => return None,
        };

        let execute = match data[2] {
            b'-' => Permissions::empty(),
            b'x' => Permissions::X,
            _ => return None,
        };

        let shared = match data[3] {
            b's' => Permissions::S,
            b'p' => Permissions::P,
            _ => return None,
        };

        Some(read | write | execute | shared)
    }
}

impl Device {
    fn parse(data: &str) -> Option<Self> {
        let (major, minor) = data.split_once(':')?;
        Some(Self {
            major: u32::from_str_radix(major, 16).ok()?,
            minor: u32::from_str_radix(minor, 16).ok()?,
        })
    }
}

fn parse_hex(data: &str) -> Option<usize> {
    usize::from_str_radix(data, 16).ok()
}
