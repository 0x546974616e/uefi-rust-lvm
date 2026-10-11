# :set noexpandtab
# TODO: `cargo build --message-format=json`

BIN = BOOTx64.efi
TARGET = x86_64-unknown-uefi
PROFILE ?= debug

DTARGET  = target
DESP     = $(DTARGET)/ESP
DBIN     = $(DTARGET)/$(TARGET)/$(PROFILE)
DBOOT    = $(DESP)/EFI/BOOT

CARGO  = cargo
COPY   = /usr/bin/cp
MKDIR  = /usr/bin/mkdir -p
REMOVE = /usr/bin/rm --preserve-root=all -f
QEMU   = /usr/bin/qemu-system-x86_64
OVMF   = /usr/share/ovmf/OVMF.fd

QEMUFLAGS := -name TREFI

# The recommended way to use OVMF is with a pflash parameter.
# TODO: OVMF image can be splitted into CODE and VARS section.
QEMUFLAGS += -drive if=pflash,format=raw,unit=0,file=$(OVMF),readonly=on
# QEMUFLAGS += -bios /usr/share/ovmf/OVMF.fd

# QEMU can emulate a virtual drive with a FAT filesystem.
# https://en.wikibooks.org/wiki/QEMU/Devices/Storage#Virtual_FAT_filesystem_(VVFAT)
QEMUFLAGS += -drive file=fat:rw:$(DESP),media=disk,format=raw

# Ensure that the mouse cursor is visible when
# interacting with the virtual machine.
QEMUFLAGS += -display default,show-cursor=on

# Prevent recent versions of QEMU from attempting
# a PXE (network) boot when no boot disk is found.
QEMUFLAGS += -net none

.PHONY: fmt build run clean
.DEFAULT_GOAL := build

fmt:
	@$(CARGO) +nightly fmt

build:
ifneq ($(PROFILE),debug)
	@$(CARGO) build --profile $(PROFILE)
else
	@$(CARGO) build
endif

run: build
	@$(MKDIR) $(DBOOT)
	@$(COPY) $(DBIN)/$(BIN) $(DBOOT)
	@$(QEMU) $(QEMUFLAGS)

clean:
	@$(REMOVE) $(DBOOT)/$(BIN)
