use fdt::Fdt;

pub struct DeviceTree {
    dt: Fdt<'static>,
    pub dtb_pa: usize,
    // Total size of the devicetree in bytes
    pub total_size: usize,
    pub memory: Memory,
}

impl DeviceTree {
    pub fn new(dtb_pa: usize) -> Self {
        let fdt = unsafe { Fdt::from_ptr(dtb_pa as *const u8).expect("parse dtb_pa failed") };

        let dt = Self {
            dt: fdt,
            dtb_pa,
            total_size: fdt.total_size(),
            memory: Self::memory(&fdt),
        };

        Self::exts(&fdt);

        dt
    }

    /// 提取 CPU 扩展
    fn exts(fdt: &Fdt) {
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

        let isa = cpu
            .property("riscv,isa")
            .expect("[Device Tree] CPU has not property \"riscv,isa\"")
            .as_str()
            .expect("[Device Tree] CPU unknow riscv,isa");

        crate::println!("riscv,isa: {}", isa);
    }

    fn memory(fdt: &Fdt) -> Memory {
        let m = fdt
            .memory()
            .regions()
            .next()
            .expect("[DEVICE TREE] must has memory");

        Memory {
            start: m.starting_address as usize,
            size: m.size.expect("[DEVICE TREE] memory must has size"),
        }
    }
}

/// 设备树上的内存信息
pub struct Memory {
    /// 内存开始地址
    pub start: usize,
    /// 内存大小
    pub size: usize,
}
