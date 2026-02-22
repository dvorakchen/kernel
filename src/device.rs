use fdt::Fdt;

pub struct DeviceTree {
    dt: Fdt<'static>,
    // Total size of the devicetree in bytes
    pub total_size: usize,
}

impl DeviceTree {
    pub fn new(dtb_pa: usize) -> Self {
        let fdt = unsafe { Fdt::from_ptr(dtb_pa as *const u8).expect("parse dtb_pa failed") };

        let dt = Self {
            dt: fdt,
            total_size: fdt.total_size(),
        };

        dt.exts();

        dt
    }

    fn exts(&self) {
        let cpu = self
            .dt
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
}

pub fn parse_dtb(dtb_pa: usize) {
    let fdt = unsafe { Fdt::from_ptr(dtb_pa as *const u8).expect("parse dtb_pa failed") };

    let cpu_len = fdt.cpus().count();
    crate::println!("cpu len: {}", cpu_len);

    fdt.cpus().for_each(|cpu| {
        crate::println!("cpu timebase frequency: {}", cpu.timebase_frequency());
    });
}
