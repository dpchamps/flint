## 01/26/2025

Bootstrapping the entire thing was fairly easy. I sunk a lot of time into figuring out why UART wasn't displaying 
correctly. This lead me to using some tools that I haven't used in a while.

#### Tool 1 - `riscv64-unknown-elf-objdump`

disassemble the binary. usefull to see how things are getting layed out after compilation

#### Tool 2, good old gdb 

I wouldn't have been able to determine what was failing with uart without gdb.
Mac ARM chips don't ship with gdb, but because I'm compiling to riscv, I was able to use

- `riscv64-elf-gdb`
- `qemu -d guest_errors,exec`

### Debugging

UART was not working as expected. A key observation was learning that when stepping through the program
with gdb, I would eventually get to `kmain`. But when running normally nothing would happen.

After inspecting the `exec` output of the qemu.debug logs, I noticed that the program
was getting stuck here: `Trace 0: 0x1055f5400 [00000000/0000000080000080/0b024003/ff020000]`

Looking at the objdump: (`riscv64-unknown-elf-objdump -d ./target/riscv-flint-os/debug/flint_os`)


```
0000000080000080 <asm_trap_vector>:
    80000080:   30200073                mret
```

I saw that this was in the asm trap vector, which meant that something funky was going on when running normally.

Chat gpt was able to help me shed some light on this:

1. `(gdb) p/x $mcause`
    a. Documented here: https://five-embeddev.com/riscv-priv-isa-manual/Priv-v1.12/machine.html#sec:mcause
    b.   
2. `(gdb) p/x $mepc` 
    a. Documented here: https://five-embeddev.com/riscv-priv-isa-manual/Priv-v1.12/machine.html#machine-exception-program-counter-mepc

The output of the above commands was

```
(gdb) p/x $mcause
$1 = 0x8000000000000007
(gdb) p/x $mepc
$2 = 0x8000006c

// dasm
0x8000006c <_start+108>                 auipc   ra,0x1
```

It's helpful here to just capypasta gpts response:


```
1. mcause: 0x8000000000000007
This value indicates:

Interrupt Flag (0x8000000000000000):
The most significant bit (0x8000000000000000) indicates that this is an interrupt, not an exception.
Interrupt Code (0x7):
Interrupts have codes that correspond to their source. For RISC-V:
0x7 = Machine Timer Interrupt (MTI).
This means the timer interrupt was triggered.
2. mepc: 0x8000006c
The mepc register points to the instruction where the trap occurred:

0x8000006c: Address of the auipc ra,0x1 instruction.
This instruction is part of _start, which means your program was interrupted while executing early startup code.
```

I don't fully understand why exactly this program execution failed when running normally. I'm going to 
guess that when stepping incrementally, the Machine Timer is somehow prevented from executing 
as it would normally.

Because I'm not handling that interrupt yet, I'd just get dumped into the asm_trap_vector. 

The temporary workaround was this diff here:

```asm
    csrw	mtvec, t2
    # li	t3, (1 << 3) | (1 << 7) | (1 << 11)
## into
	li		t3, (1 << 3) | (1 << 11)
```

Disabling the Machine's timer interrupt-enable bit