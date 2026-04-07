# src/trap.s
# RISC-V Trap Handler (U-mode -> S-mode)
#
#   1. 没有栈切换
#   2. 保存所有通用寄存器
#   3. 调用 Rust 处理函数
#   4. 恢复现场并返回
#
#   ## 时钟中断
#   当触发时钟中断的时候，应该是要切换进程的，如何触发呢
#   
#   1.  当进入 `kernel_trap_entry` 的时候，当前是任务 A
#       将寄存器存入任务 A 的栈，将 TrapFrame 给 a0
#       将 TrapFrame 作为第一个参数传递给 `kernel_handle_trap`
#       注意此时的栈是**任务A**的栈
#   2.  进入到 `kernel_handle_trap`
#       通过 `schedule` 挑选好任务 B 后，
#       使用 __switch 切换任务 A 和任务 B
#
#   ## __switch
#   __switch(current_task: *mut TaskContext, next_task: *mut TaskContext)
#
#   在 RISC-V 中，__switch 通常用汇编实现，它的本质是：把当前 CPU
  # 里的寄存器换成另一个任务预存好的寄存器。
  #
  # 1. 为什么只保存部分寄存器（ra, sp, s0-s11）？
  #
  #
  # 在 RISC-V 调用约定中，寄存器分为两类：
  #  * Caller-saved (调用者保存)：如 t0-t6, a0-a7。根据 Rust/C
  #    编译器规则，如果函数里要用这些寄存器，编译器会在调用 __switch
  #    之前就把它们压入栈中。
  #  * Callee-saved (被调用者保存)：如
  #    s0-s11。编译器假设函数调用后这些寄存器不变。
  #  * ra (返回地址)：决定了函数执行完去哪。
  #  * sp (栈指针)：决定了当前的栈在哪里。
#   
#   核心逻辑：由于 __switch 是被 schedule 函数调用的，编译器已经帮我们把 a 和 t
  # 系列寄存器存在栈上了。我们只需要手动保存那些编译器不负责的 s
  # 系列寄存器和最关键的 sp。
  #
  # 步骤:
  #
  # 1. 将 ra, sp, s0-s11 存入 current_task 的 TaskContext 中
  # 2. 将 next_task 的 TaskContext 的 ra, sp, s0-s11 恢复到寄存器里
  #
  # 此时就完成了任务 A 和任务 B 的栈切换


.section .text.trap
.align 6

kernel_trap_entry:

  addi sp, sp, -32*8

  # sd 指令 将寄存器的值存入内存
  sd x0,  0*8(sp)
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

  # 保存进入中断前的原始栈指针到 x2 (sp) 槽位
  addi t0, sp, 32*8
  sd t0, 2*8(sp)

  # 将当前栈指针（指向 TrapFrame）作为第一个参数传递给 a0
  mv a0, sp

  # 调用 Rust 处理函数
  #   fn kernel_handle_trap(trap_frame: TrapFrame)
  call kernel_handle_trap

  # ld 指令 将内存的值存入寄存器
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

  # 从 TrapFrame 恢复原始栈指针 (x2)
  # 这步操作会同时恢复 sp 的值并完成栈帧的释放
  ld sp, 2*8(sp)

  sret
