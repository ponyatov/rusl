# var
MODULE = $(notdir $(CURDIR))
REL    = $(shell git rev-parse --short=4    HEAD)
BRANCH = $(shell git rev-parse --abbrev-ref HEAD)
NOW    = $(shell date +%d%m%y)

# cross
HW = stm32vldiscovery
include   hw/$(HW).mk
include  cpu/$(CPU).mk
include arch/$(ARCH).mk

# version
JQUERY_VER = 3.7.1

# dir
CWD = $(CURDIR)
CAR = $(HOME)/.cargo/bin

# tool
CURL   = curl -L -o
RUSTUP = $(CAR)/rustup
CARGO  = $(CAR)/cargo
GITREF = git clone --depth 1

# src
R += $(wildcard src/*.rs)
R += $(wildcard config/src/*.rs)
R += $(wildcard server/src/*.rs)
R += $(wildcard media/src/*.rs)
R += $(wildcard gui/src/*.rs)
R += $(wildcard embed/src/*.rs)

# all
.PHONY: run all
all: bin/$(MODULE)
run: lib/$(MODULE).ini $(R)
	$(CARGO) run -- $<
# RUST_BACKTRACE=1

.PHONY: server
server: $(R)
	$(CARGO) run -p $@

.PHONY: media
media: $(R)
	$(CARGO) run -p $@

.PHONY: gui
gui: $(R)
	$(CARGO) run -p $@

.PHONY: embed
embed: $(R)
	$(CARGO) run -p $@

.PHONY: qemu
qemu:
	$(QEMU) $(QEMU_CFG) -S -s

# format
.PHONY: format
format: tmp/format_rs
tmp/format_rs: $(R)
	$(CARGO) check && $(CARGO) fmt && touch $@

# rule
bin/$(MODULE): $(R)
	$(CARGO) build

# doc
.PHONY: doc
doc: doc/The_Rust_Programming_Language.pdf

doc/The_Rust_Programming_Language.pdf: $(HOME)/doc/Rust/The_Rust_Programming_Language.pdf
	cd doc ; ln -fs ../../doc/Rust/The_Rust_Programming_Language.pdf The_Rust_Programming_Language.pdf

$(HOME)/doc/Rust/The_Rust_Programming_Language.pdf:
	$(CURL) $@ https://www.scs.stanford.edu/~zyedidia/docs/rust/rust_book.pdf

# install
.PHONY: install update ref gz
install: doc ref gz $(RUSTUP)
	$(MAKE) update rust
update: $(RUSTUP)
	sudo apt update
	sudo apt install `cat apt.txt`
	$(RUSTUP) self update ; $(RUSTUP) update
ref:
gz: cdn

$(RUSTUP):
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

.PHONY: rust
rust: $(RUSTUP) ref/cortex-m-quickstart cpu/STM32F100RB.ld
	$(RUSTUP) component add rustfmt
	$(RUSTUP) target add $(TARGET)

ref/cortex-m-quickstart:
	$(GITREF) https://github.com/rust-embedded/cortex-m-quickstart.git $@

cpu/STM32F100RB.ld:
	$(CURL) $@ https://robocraft.ru/files/stm32/lesson-1-ide/stm32f100rb.ld

# cdn
CDNJS = https://cdnjs.cloudflare.com/ajax/libs
.PHONY: cdn
cdn: \
	server/static/cdn/jquery.min.js
server/static/cdn/jquery.min.js:
	$(CURL) $@ $(CDNJS)/jquery/$(JQUERY_VER)/jquery.min.js
