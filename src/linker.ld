/* 
 * 链接器脚本 (Linker Script)
 * 作用：规定如何将输入文件中的各个“段” (Sections) 合并并安排到输出的可执行文件中。
 */

/* 设定目标架构为 RISC-V */
OUTPUT_ARCH(riscv)

/* 设定程序的入口点为 _start 标签（通常在 entry.S 中定义） */
ENTRY(_start)

/* 
 * 定义常量 
 * BASE_ADDRESS: 内核运行时的虚拟起始地址（高位地址，Higher Half Kernel）
 * OFFSET: 虚拟地址与物理地址之间的偏移量。
 *         物理地址 = 虚拟地址 - OFFSET
 *         例如：0xFFFFFFFF80200000 - 0xFFFFFFFF00000000 = 0x80200000 (实际物理内存地址)
 */
BASE_ADDRESS = 0xFFFFFFFF80200000;
OFFSET = 0xFFFFFFFF00000000;

SECTIONS {
    /* 设置当前虚拟地址指针 (.) 为内核起始地址 */
    . = BASE_ADDRESS;

    /* 记录内核起始位置的符号，可在 C/Rust 代码中通过 extern 引用 */
    skernel = .;

    /* --- .text 段：存放代码 (Instruction) --- */
    stext = .;
    /* 
     * AT(...) 指定 LMA (加载地址)，即该段在物理内存中的实际存放位置。
     * 虽然程序认为自己在 BASE_ADDRESS 运行，但它实际被加载到物理地址 0x80200000。
     */
    .text : AT(ADDR(.text) - OFFSET) {
        *(.text.entry)     /* 确保引导代码（如汇编跳转）放在最前面 */
        *(.text .text.*)   /* 所有的代码段 */
    }

    /* 将地址对齐到 4KB（页大小），确保下一个段从新的物理页开始 */
    . = ALIGN(4K);
    etext = .; /* 代码段结束地址 */

    /* --- .rodata 段：存放只读数据 (Read-Only Data) --- */
    srodata = .;
    .rodata : AT(ADDR(.rodata) - OFFSET) {
        *(.rodata .rodata.*) /* 全局常量、字符串常量等 */
    }

    . = ALIGN(4K);
    erodata = .; /* 只读数据段结束地址 */

    /* --- .data 段：存放已初始化的全局变量 (Initialized Data) --- */
    sdata = .;
    .data : AT(ADDR(.data) - OFFSET) {
        *(.data .data.*)
        *(.sdata .sdata.*)   /* RISC-V 特有的近端数据段 */
        *(.data.preinit)     /* 某些特殊的初始化数据，如启动页表 */
    }

    . = ALIGN(4K);
    edata = .; /* 数据段结束地址 */

    /* --- .bss 段：存放未初始化的全局变量 (Uninitialized Data) --- */
    /* NOLOAD 告诉链接器：这个段在二进制镜像中不占用空间，运行时由内核手动清零 */
    .bss (NOLOAD) : {
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        ebss = .;

        /* 在 .bss 的末尾预留内核栈空间 */
        . = ALIGN(4K);
        *(.stack) 
    }

    . = ALIGN(4K);
    ebss_final = .;
    ekernel = .; /* 内核整体结束地址 */

    /* --- 丢弃段 --- */
    /DISCARD/ : {
        *(.eh_frame) /* 丢弃掉 C++ 异常处理相关的段，内核通常不需要 */
    }
}

