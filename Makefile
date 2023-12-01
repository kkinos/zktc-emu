ASM_DIR:=test/asm
ASMS:=$(sort $(wildcard $(ASM_DIR)/*.asm))

TARGET:=$(patsubst %.asm, test/mem/%.mem, $(notdir $(ASMS)))

.PHONEY: test
test: $(TARGET)
	cargo test

test/mem/%.mem: test/asm/%.asm
	zktc-asm $< -o $@ -b 0x8000

.PHONEY: clean
clean:
	rm -rf test/mem/*.mem