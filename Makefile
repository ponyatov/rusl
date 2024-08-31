# var
MODULE = $(notdir $(CURDIR))

# dir
CAR = $(HOME)/.cargo/bin

# tool
CURL   = curl -L -o
RUSTUP = $(CAR)/rustup
CARGO  = $(CAR)/cargo

# src
R += $(wildcard src/*.rs)

# all
.PHONY: run all
all: bin/$(MODULE)
run: lib/$(MODULE).ini $(R)
	$(CARGO) run -- $<
# RUST_BACKTRACE=1

# format
.PHONY: format
format: tmp/format_rs
tmp/format_rs: $(R)
	$(CARGO) fmt && touch $@

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
	$(MAKE) update
	$(RUSTUP) component add rustfmt
update: $(RUSTUP)
	sudo apt update
	sudo apt install `cat apt.txt`
	$(RUSTUP) self update ; $(RUSTUP) update
ref:
gz:

$(RUSTUP):
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
