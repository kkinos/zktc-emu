ASMS:=$(sort $(wildcard test/asm/*.asm))
TARGET:=$(patsubst %.asm, test/mem/%.mem, $(notdir $(ASMS)))

.PHONY: test
test: $(TARGET)
	cargo test

test/mem/%.mem: test/asm/%.asm
	zktc-asm $< -o $@ -b 0xb000

.PHONY: clean
clean:
	rm -rf test/mem/*.mem