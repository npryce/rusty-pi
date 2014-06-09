
pi=192.168.1.13

linker=../tools/arm-bcm2708/arm-bcm2708hardfp-linux-gnueabi/bin/arm-bcm2708hardfp-linux-gnueabi-g++

asciidoc:=$(wildcard doc/*.asciidoc)
book_images:=$(wildcard doc/*.svg doc/*.jpg doc/*.png)

version:=$(shell git describe --tags --always --dirty=-local --match='v*' | sed -e 's/^v//')


all: book

book: out/html/book.html

deployed: out/blink out/button out/hello out/signals
	rsync $^ $(pi):

out/%: src/%.rs src/pi.rs
	@mkdir -p $(dir $@)
	rustc -o $@ -L . --target arm-unknown-linux-gnueabihf -C linker=$(linker) $<

out/html/book.html: $(asciidoc) $(book_images:doc/%=out/html/%)
	@mkdir -p $(dir $@)
	asciidoc \
		-a icons \
		-a version="$(version)" \
		-o $@ doc/book.asciidoc

out/html/%: doc/%
	@mkdir -p $(dir $@)
	cp $< $@

clean:
	rm -rf out/

again: clean deployed

.PHONY: all deployed book clean again
