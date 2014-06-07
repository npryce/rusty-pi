
pi=192.168.1.13

linker=../tools/arm-bcm2708/arm-bcm2708hardfp-linux-gnueabi/bin/arm-bcm2708hardfp-linux-gnueabi-g++

asciidoc:=$(wildcard doc/*.asciidoc)

all: deployed book

book: out/html/book.html

deployed: out/blink out/button out/hello out/signals
	rsync $^ $(pi):

out/%: src/%.rs src/pi.rs
	@mkdir -p $(dir $@)
	rustc -o $@ -L . --target arm-unknown-linux-gnueabihf -C linker=$(linker) $<

out/html/book.html: $(asciidoc)
	@mkdir -p $(dir $@)
	asciidoc -o $@ doc/book.asciidoc

clean:
	rm -rf out/

again: clean deployed
