# dir
CAR = $(HOME)/.cargo/bin

# tool
CURL   = curl -L -o
RUSTUP = $(CAR)/rustup
CARGO  = $(CAR)/cargo

# doc
.PHONY: doc
doc: doc/The_Rust_Programming_Language.pdf

doc/The_Rust_Programming_Language.pdf: $(HOME)/doc/Rust/The_Rust_Programming_Language.pdf
	cd doc ; ln -fs ../../doc/Rust/The_Rust_Programming_Language.pdf The_Rust_Programming_Language.pdf

$(HOME)/doc/Rust/The_Rust_Programming_Language.pdf:
	$(CURL) $@ https://www.scs.stanford.edu/~zyedidia/docs/rust/rust_book.pdf

# install
.PHONY: install update ref gz
install: doc ref gz
	$(MAKE) update
update: $(RUSTUP)
	sudo apt update
	sudo apt install `cat apt.txt`
	$(RUSTUP) self update ; $(RUSTUP) update
ref:
gz:

$(RUSTUP):
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
