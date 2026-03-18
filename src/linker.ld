OUTPUT_ARCH(riscv)
ENTRY(_start)

BASE_ADDRESS = 0xFFFFFFFF80200000;

OFFSET = 0xFFFFFFFF00000000;

SECTIONS {
  . = BASE_ADDRESS;

  skernel = .;
	stext = .;
	.text : AT(ADDR(.text) - OFFSET) {
		*(.text.entry)
		*(.text .text.*)
	}
	
	. = ALIGN(4K);
	etext = .;
	srodata = .;
	.rodata : AT(ADDR(.rodata) - OFFSET) {
		*(.rodata .rodata.*)
	}

	. = ALIGN(4K);
	erodata = .;
	sdata = .;
	.data : AT(ADDR(.data) - OFFSET) {
		*(.data .data.*)
		*(.sdata .sdata.*)
    *(.data.preinit) /* 启动页表在这里，它是 progbits，被加载器填入 RAM */
	}

	. = ALIGN(4K);
	edata = .;
	.bss (NOLOAD) : {
		sbss = .;
		*(.bss .bss.*)
		*(.sbss .sbss.*)
		ebss = .;

		. = ALIGN(4K);
		*(.stack) /* 栈在 .bss 的末尾，但不包含在 sbss-ebss 范围内 */
	}

	. = ALIGN(4K);
	ebss_final = .;
	ekernel = .;
	
	/DISCARD/ : {
		*(.eh_frame)
	}
}
