ASM_DIR:=asm
ASMS:=$(sort $(wildcard $(ASM_DIR)/*.asm))

TARGET:=$(patsubst %.asm, mem/%.mem, $(notdir $(ASMS)))

.PHONEY: test
test: $(TARGET)
	cargo test

mem/%.mem: asm/%.asm
	zktc-asm $< -o $@ -b 0x8000

.PHONEY: clean
clean:
	rm -rf mem/*.mem