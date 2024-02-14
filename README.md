# zktc-emu

zktc-emu is a [ZKTC](https://github.com/kinpoko/zktc) emulator implemented in Rust.

# Install

```bash
cargo install --git https://github.com/kinpoko/zktc-emu.git
```

# Usage

Create a `mem` file using [zktc-asm](https://github.com/kinpoko/zktc-asm).

```bash
zktc-asm rom_file.asm -o rom_file.mem
```

Loads a `mem` file into ROM and executes.

```bash
zktc-emu rom_file.mem
zktc-emu >>
```

You can also specify which a `mem` file to load into RAM with the `--ram` option.

```bash
zktc-emu rom_file.mem --ram ram_file.mem
```

# Commands

```bash
zktc-emu >> help
run, r        : continue to execute until break point

step, s       : step execute

breakpoint, b : set breakpoint (b 0x8000)

mem, m        : display data in memory (m 0x8000 10)

regsters, regs: display data in register

help          : show this message

exit          : exit
zktc-emu >>
```

# Tests

With [zktc-asm](https://github.com/kinpoko/zktc-asm) installed, the following commands can be used for testing.

```bash
make test
```
