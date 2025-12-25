# Makefile for bare-metal AArch64 kernel

TARGET := aarch64-unknown-none
KERNEL_ELF := target/$(TARGET)/release/start
KERNEL_BIN := kernel.bin

# 工具链
OBJCOPY := rust-objcopy
QEMU := qemu-system-aarch64

# QEMU 参数
QEMU_OPTS := -machine virt \
             -cpu cortex-a72 \
             -nographic \
             -kernel $(KERNEL_BIN)

.PHONY: all build bin run clean debug

all: bin

# 构建 ELF
build:
	cargo build --release --target $(TARGET)

# 生成二进制镜像
bin: build
	$(OBJCOPY) --strip-all $(KERNEL_ELF) -O binary $(KERNEL_BIN)
	rust-objdump -d $(KERNEL_ELF) > kernel.disasm

# 在 QEMU 中运行
run: bin
	$(QEMU) $(QEMU_OPTS)

# 调试模式运行（等待 GDB 连接）
debug: bin
	$(QEMU) $(QEMU_OPTS) -s -S

# 清理构建产物
clean:
	cargo clean
	rm -f $(KERNEL_BIN)
