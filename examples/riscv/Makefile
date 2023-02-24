arch ?= riscv64
target := $(arch)gc-unknown-none-elf
mode := release
kernel := target/$(target)/$(mode)/riscv
img := target/$(target)/$(mode)/img

ifeq ($(mode), release)
	BUILD_ARGS += --release
endif

kernel:
	cargo build $(BUILD_ARGS)

qemu: kernel $(img)
	qemu-system-$(arch) \
		-machine virt \
		-serial mon:stdio \
		-bios default \
		-kernel $(kernel) \
		-global virtio-mmio.force-legacy=false \
		-drive file=$(img),if=none,format=raw,id=x0 \
		-device virtio-blk-device,drive=x0 \
		-device virtio-gpu-device \
		-device virtio-mouse-device


$(img):
	dd if=/dev/zero of=$@ bs=512 count=32
