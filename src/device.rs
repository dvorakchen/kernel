use fdt::Fdt;

pub fn parse_dtb(dtb_pa: usize) {
    let fdt = unsafe { Fdt::from_ptr(dtb_pa as *const u8).expect("parse dtb_pa failed") };

    let cpu_len = fdt.cpus().count();
    crate::println!("cpu len: {}", cpu_len);

    fdt.cpus().for_each(|cpu| {
        crate::println!("cpu timebase frequency: {}", cpu.timebase_frequency());
    });
}
