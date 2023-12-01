# zktc-emu

zktc-emu は Rust で実装された ZKTC エミュレータです。

# インストール

```bash
git clone https://github.com/kinpoko/zktc-emu.git
cargo build --release
```

# 使い方

[zktc-asm](https://github.com/kinpoko/zktc-asm)を使って mem ファイルを作成してください。

```bash
zktc-asm rom_file.asm -o rom_file.mem
```

mem ファイルを ROM にロードして実行します。`--ram`オプションで RAM にロードする mem ファイルを指定することもできます。

```bash
zktc-emu rom_file.mem
zktc-emu >>
```

# コマンド

実行中は以下のコマンドが使用できます。

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

# テスト

[zktc-asm](https://github.com/kinpoko/zktc-asm)をビルドしてパスを通した状態で以下のコマンドでテストを行うことができます。

```bash
make test
```
