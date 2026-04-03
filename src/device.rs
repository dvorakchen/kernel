use core::str::FromStr;

use ::alloc::{boxed::Box, string::String};
use anyhow::Result;
use fdt::Fdt;

use crate::mm::PAGE_SIZE;

pub struct DeviceTree {
    dt: Fdt<'static>,
    pub dtb_pa: usize,
    // Total size of the devicetree in bytes
    pub total_size: usize,
    pub memory: Memory,
    pub cpu: CPU,
}

impl DeviceTree {
    pub fn new(dtb_pa: usize) -> Result<Self> {
        let fdt = unsafe { Fdt::from_ptr(dtb_pa as *const u8).expect("parse dtb_pa failed") };

        let cpu = Self::cpu(&fdt)?;

        let dt = Self {
            dt: fdt,
            dtb_pa,
            total_size: fdt.total_size(),
            memory: Self::memory(&fdt),
            cpu,
        };

        Ok(dt)
    }

    /// 提取 CPU 设备信息
    fn cpu(fdt: &Fdt) -> Result<CPU> {
        let cpu = fdt
            .cpus()
            .filter(|cpu| {
                let status = cpu
                    .property("status")
                    .expect("[Device Tree] CPU has not property \"status\"")
                    .as_str()
                    .expect("[Device Tree] CPU unknow status");

                status == "okay"
            })
            .next()
            .expect("[Device Tree] CPU has not property \"status\"");

        // 获取 CPU isa
        let isa = cpu
            .property("riscv,isa")
            .expect("[Device Tree] CPU has not property \"riscv,isa\"")
            .as_str()
            .expect("[Device Tree] CPU unknow riscv,isa");

        crate::println!("[DEVICE TREE] riscv,isa: {}", isa);

        // 获取频率
        let freq = cpu.timebase_frequency();
        crate::println!("[DEVICE TREE] riscv timebase_frequency: {}", freq);

        Ok(CPU {
            // isa: Box::leak(String::from_str(isa)?.into_boxed_str()),
            timebase_freq: freq as u64,
        })
    }

    fn memory(fdt: &Fdt) -> Memory {
        let m = fdt
            .memory()
            .regions()
            .next()
            .expect("[DEVICE TREE] must has memory");

        let start = m.starting_address as usize;
        let size = m.size.expect("[DEVICE TREE] memory must has size");
        crate::println!("[DEVICE TREE] Device Memory start address: {:#x}", start);
        crate::println!("[DEVICE TREE] Device Memory size: {:#x}", size);

        assert!(size > PAGE_SIZE, "[DEVICE TREE] memory not a valid page");

        Memory { start, size }
    }
}

/// 设备树上的内存信息
/// 地址信息是物理地址
#[derive(Clone, Copy)]
pub struct Memory {
    /// 内存开始地址
    pub start: usize,
    /// 内存结束地址
    pub size: usize,
}

/// 设备树上的 CPU 信息
#[derive(Copy, Clone)]
pub struct CPU {
    // isa: &'static str,
    pub timebase_freq: u64,
}
