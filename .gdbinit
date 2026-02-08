  file /home/dvorak/Projects/kernel/target/riscv64gc-unknown-none-elf/debug/kernel
  set arch riscv:rv64
  target remote localhost:1234
  break *0x80200000
  break main
  c
