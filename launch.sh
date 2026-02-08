qemu-system-riscv64 -machine virt \
  -nographic \
  -bios default \
  -kernel /home/dvorak/Projects/kernel/target/riscv64gc-unknown-none-elf/debug/kernel \
  -smp 4
