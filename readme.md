# Flint

Exploring bare metal rust on Risc-V.


## Development requirements (in addition to the rust toolchain)

- qemu
- rscv toolchain (`riscv-gnu-toolchain` on mac)
- `riscv64-elf-gdb`

## Notes

Everything is setup through the cargo build system. Projects have individual configurations, so running them from the
workspace will not work. The main point of interest is `src/kernel`:

```bash
$ cd src/kernel
$ cargo build
$ cargo run

>  Running `qemu-system-riscv64 -D qemu.debug -machine virt -bios none -serial 'mon:stdio' -kernel /path/to/flint/target/riscv-flint-os/debug/kernel`

```


### gdb debugging

The flags `-S -gdb tcp::<port>` will pause qemu and wait for gdb to connect. Using your riscv flavored gdb:

```bash 
$ riscv64-elf-gdb ../../target/riscv-flint-os/debug/flint_os
$ (gdb) target remote localhost:<port>
```

There are various configs in the config.toml that can be uncommented as needed. Eventually, I'd like to add a `DEBUG=true`
flag.