# Kernel

This is just I learning OS

## Prerequisites

- rust tool-chains and bare-metal riscv64 target
- qemu-system-riscv64
- riscv64-linux-gnu-gdb

## Dev with GDB

Run command:

```sh

bash launch.sh
```


Debug command:

```sh
bash launch-debug.sh
```

Open other terminal:

```sh
riscv64-linux-gnu-gdb -x .gdbinit
```


