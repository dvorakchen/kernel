# src/trap.s
# RISC-V Trap Handler (U-mode -> S-mode)
#
#   1. 栈切换：用户栈 → 内核栈
#   2. 保存所有通用寄存器
#   3. 调用 Rust 处理函数
#   4. 恢复现场并返回


.section .text.trap
.global trap_entry
.align 6

trap_entry:

  # 栈切换: 用户站 -> 内核栈
  # 此时：
  #   sp        =   用户栈指针
  #   sscratch  =   内核栈指针
  # 
  # csrrw sp, sscratch, sp 执行后：
  #   sp        =   内核栈顶
  #   sscratch  =   原用户栈
  //csrrw sp, sscratch, sp

  addi sp, sp, -32*8

  sd x1,  1*8(sp)
  sd x3,  3*8(sp)
  sd x4,  4*8(sp)
  sd x5,  5*8(sp)
  sd x6,  6*8(sp)
  sd x7,  7*8(sp)
  sd x8,  8*8(sp)
  sd x9,  9*8(sp)
  sd x10, 10*8(sp)
  sd x11, 11*8(sp)
  sd x12, 12*8(sp)
  sd x13, 13*8(sp)
  sd x14, 14*8(sp)
  sd x15, 15*8(sp)
  sd x16, 16*8(sp)
  sd x17, 17*8(sp)
  sd x18, 18*8(sp)
  sd x19, 19*8(sp)
  sd x20, 20*8(sp)
  sd x21, 21*8(sp)
  sd x22, 22*8(sp)
  sd x23, 23*8(sp)
  sd x24, 24*8(sp)
  sd x25, 25*8(sp)
  sd x26, 26*8(sp)
  sd x27, 27*8(sp)
  sd x28, 28*8(sp)
  sd x29, 29*8(sp)
  sd x30, 30*8(sp)
  sd x31, 31*8(sp)

  # 保存用户栈指针到 t0
  //csrr t0, sscratch
  sd t0, 2*8(sp)

  # 读取关键 csr 传递给 Rust
  csrr a0, sepc
  csrr a1, scause
  csrr a2, stval
  csrr a3, sstatus

  # 调用 Rust 处理函数
  #   fn handle_trap(sepc: usize, scause: usize, stval: usize, sstatus: usize)
  call handle_trap

  ld x1,  1*8(sp)
  ld x3,  3*8(sp)
  ld x4,  4*8(sp)
  ld x5,  5*8(sp)
  ld x6,  6*8(sp)
  ld x7,  7*8(sp)
  ld x8,  8*8(sp)
  ld x9,  9*8(sp)
  ld x10, 10*8(sp)
  ld x11, 11*8(sp)
  ld x12, 12*8(sp)
  ld x13, 13*8(sp)
  ld x14, 14*8(sp)
  ld x15, 15*8(sp)
  ld x16, 16*8(sp)
  ld x17, 17*8(sp)
  ld x18, 18*8(sp)
  ld x19, 19*8(sp)
  ld x20, 20*8(sp)
  ld x21, 21*8(sp)
  ld x22, 22*8(sp)
  ld x23, 23*8(sp)
  ld x24, 24*8(sp)
  ld x25, 25*8(sp)
  ld x26, 26*8(sp)
  ld x27, 27*8(sp)
  ld x28, 28*8(sp)
  ld x29, 29*8(sp)
  ld x30, 30*8(sp)
  ld x31, 31*8(sp)               

  # 恢复 sscratch 用户栈指针
  ld t0, 2*8(sp)
  //csrw sscratch, t0
  
  # 释放栈帧
  addi sp, sp, 32*8

  # 栈切换
  //csrrw sp, sscratch, sp
  sret
