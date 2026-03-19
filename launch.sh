cargo b
qemu-system-riscv64 -machine virt \
  -nographic \
  -bios default \
  -smp 4 \
  -kernel /home/dvorak/Projects/kernel/target/riscv64gc-unknown-none-elf/debug/kernel
