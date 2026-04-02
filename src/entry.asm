/*
 * 内核入口汇编 (Entry Point)
 * 职责：从物理地址模式切换到虚拟分页模式，设置栈指针，最后跳转到 Rust/C 编写的 main 函数。
 */

.section .text.entry
.global _start

_start:
    /* 
     * 1. 获取临时页表 (boot_page_table) 的物理地址 
     * 由于此时还没开启分页，la 指令拿到的是物理地址。
     */
    la t0, boot_page_table
    /* 
     * 处理 64 位地址：清除高位，确保拿到的是纯物理地址。
     * (slli/srli 32 位是常见的清除高位的技巧，确保地址在低 32 位范围内)
     */
    slli t0, t0, 32
    srli t0, t0, 32

    /* 
     * 2. 手工填充页表项 (PTE - Page Table Entry) 
     * 目标：建立 1GB 的“巨页”映射。
     * 映射关系：虚拟地址 0x8000_0000 -> 物理地址 0x8000_0000 (恒等映射)
     * 0x200000cf 的含义：
     *   - PPN = 0x80000 (指向物理地址 0x8000_0000)
     *   - Flags = 0xCF (V=1, R=1, W=1, X=1, A=1, D=1) 表示有效、可读写执行、已访问、已修改
     */
    li t1, 0x200000cf
    /* 将 PTE 写入页表的第 2 项 (16 字节偏移)，对应 0x8000_0000 的起始范围 */
    sd t1, 16(t0)

    /* 
     * 3. 建立高半区 (Higher Half) 映射 
     * 目标：虚拟地址 0xFFFF_FFFF_8000_0000 -> 物理地址 0x8000_0000
     * 0xFFFF_FFFF_8000_0000 在 SV39 下对应页表的第 510 个项 (偏移 4080 字节)
     */
    li t3, 4080
    add t3, t0, t3
    sd t1, 0(t3)

    /* 
     * 4. 开启分页模式 
     * 设置 satp 寄存器：
     *   - Mode: 8 (表示 SV39 模式)
     *   - PPN: 指向 boot_page_table 的物理页号 (地址 >> 12)
     */
    srli t2, t0, 12     # t2 = PPN
    li t1, 8            # SV39 模式标志
    slli t1, t1, 60     # 将 Mode 移至最高位 (60-63位)
    or t2, t2, t1       # 合并 Mode 和 PPN
    csrw satp, t2       # 写入 satp 寄存器
    
    /* 刷新 TLB (转换后备缓冲区)，确保新的页表生效 */
    sfence.vma

    /* 
     * 5. 跳转到高位虚拟地址运行 
     * 此时分页已开启，我们需要从当前的“物理地址代码”跳转到“虚拟地址代码”。
     */
    la t0, main_vma
    ld t1, 0(t0)        # 从内存加载 main 函数的绝对虚拟地址
    
    /* 设置内核栈指针 (Stack Pointer) */
    la t0, boot_stack_top_vma
    /* 空出 16 字节，好“栈底藏尸” */
    ld sp, 0(t0)        # 加载栈顶的绝对虚拟地址
    addi sp,sp, -16

    /* 执行绝对跳转，此时 PC (程序计数器) 将进入 0xFFFFFFFF8020... 范围 */
    jr t1

/* 
 * 临时启动页表 
 * 放入 .data.preinit 节，确保它被加载到内存中且不被 BSS 清零。
 */
.section .data.preinit, "aw"
    .align 12           # 必须按页对齐 (2^12 = 4096 字节)
boot_page_table:
    .zero 4096          # 预留 4KB 空间

/* 
 * 内核初始栈 
 */
.section .stack, "aw", @nobits
    .global boot_stack_top
    .align 12
boot_stack:
    .space 4096 * 16    # 预留 64KB 栈空间
boot_stack_top:

/* 
 * 辅助常量：跨越虚拟地址空间的“跳板”
 * 
 * 为什么需要这些常量？
 * 1. 编译后，符号 `main` 和 `boot_stack_top` 的值都是高半区虚拟地址（0xFFFFFFFF8020...）。
 * 2. 在开启分页的一瞬间，CPU 的 PC（程序计数器）还在物理地址（0x8020...）。
 * 3. 此时我们无法直接跳转，因为 RISC-V 的普通跳转指令（如 j, jal）通常是基于 PC 的相对跳转，
 *    无法跨越物理地址到虚拟地址之间巨大的“鸿沟”。
 * 
 * 解决策略：
 * 我们在只读数据段中预留几个 64 位宽的槽位，存放目标的“绝对虚拟地址”。
 * 即使当前 PC 在物理地址，我们也可以通过 `la` 拿到这些槽位的地址，
 * 然后用 `ld`（Load Doubleword）指令把里面存的 64 位绝对虚拟地址“抠”出来，
 * 最后通过 `jr`（Jump Register）实现长距离的绝对跳转。
 */
.section .rodata
main_vma:
    .dword main         # 存储 main 函数的 64 位绝对虚拟地址
boot_stack_top_vma:
    .dword boot_stack_top # 存储内核栈顶的 64 位绝对虚拟地址
