# 
.section .text.entry
.global _start

_start:
  # 拿到临时页表的实际物理地址
  la t0, boot_page_table
  slli t0, t0, 32
  srli t0, t0, 32

  # 手工捏造 PTE，建立 1G 的巨页映射
  # 映射到的物理基地址是 0x8000_0000
  li t1, 0x200000cf
  # 将临时 PTE 写入页表
  sd t1, 16(t0)

  # 建立高半区映射
  # 将 0xFFFF_FFFF_8000_0000 映射到物理地址 0x8000_0000
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

  # 此时开启了分页，通过加载存储在内存中的绝对地址来“跳往”高地址
  la t0, main_vma
  ld t1, 0(t0)        # t1 = main 的绝对虚拟地址
  
  la t0, boot_stack_top_vma
  ld sp, 0(t0)        # sp = boot_stack_top 的绝对虚拟地址

  jr t1               # 绝对跳转，PC 进入高位

  j .

  # 启动页表：放入 .data.preinit，它是 progbits，且不属于 BSS
.section .data.preinit, "aw"
  .align 12
boot_page_table:
  .zero 4096

  # 栈：放入专门的 .stack 节，并在链接脚本中避开 sbss/ebss
.section .stack, "aw", @nobits
  .global boot_stack_top
  .align 4
boot_stack:
  .space 4096 * 16
boot_stack_top:

  .section .rodata
main_vma:
  .dword main
boot_stack_top_vma:
  .dword boot_stack_top
