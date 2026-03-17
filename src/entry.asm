# 
.section .text.entry
.global _start

_start:
  # 在链接器中设定了内核在高位地址，但 OpenSBI 将内核加载的地位地址
  # 拿到临时页表的实际物理地址
  la t0, boot_page_table
  slli t0, t0, 32
  srli t0, t0, 32

  # 手工捏造 PTE，建立 1G 的巨页映射
  # 映射到的物理基地址是 0x8000_0000
  li t1, 0x200000cf
  # 将临时 PTE 写入页表，在进入 rust_main 后要清除掉
  sd t1, 16(t0)

  # 建立高半区映射
  # 将 0xFFFF_FFFF_8000_0000 映射到内核实际的物理地址 0x8000_0000
  li t3, 4080
  add t3, t0, t3
  sd t1, 0(t3)

  # 配置 satp 寄存器
  srli t2, t0, 12     # t2 = boot_page_table 的 PPN
  li t1, 8            # t1 = 8 (SV39 模式)
  slli t1, t1, 60     # 把 8 推到最高 4 位
  or t2, t2, t1
  csrw satp, t2
  sfence.vma

  la sp, boot_stack_top

  # 跳往 rust 的 main 函数
  la t0, main
  jr t0

  j .

  .section .bss.uninit
  .align 12
boot_page_table:
  .space 4096

  .section .bss.stack
  .global boot_stack_top

  .align 4
boot_stack:
  .space 4096 * 16
boot_stack_top:
