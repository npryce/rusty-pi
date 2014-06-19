
pi=192.168.1.13

linker=../tools/arm-bcm2708/arm-bcm2708hardfp-linux-gnueabi/bin/arm-bcm2708hardfp-linux-gnueabi-g++

rust_src:=$(shell find src/ -name *.rs)
book_src:=$(wildcard doc/*.asciidoc)
book_images:=$(wildcard doc/*.svg doc/*.jpg doc/*.png)

version:=$(shell git describe --tags --always --dirty=-local --match='v*' | sed -e 's/^v//')

asciidoc_icondir=/usr/share/asciidoc/icons
asciidoc_icons:=$(shell find $(asciidoc_icondir) -type f -name '*.*')

all: pdf html
pdf: out/pdf/book.pdf
html: out/html/book.html

deployed: out/blink out/button out/hello out/signals out/raw-blink
	rsync $^ $(pi):

out/%: src/%.rs src/pi.rs
	@mkdir -p $(dir $@)
	rustc -o $@ -L . --target arm-unknown-linux-gnueabihf -C linker=$(linker) $<

out/pdf/book.pdf: out/docbook/book.xml
	@mkdir -p $(dir $@)
	dblatex -o $@ --fig-path=doc -P latex.encoding=utf8 $<

out/docbook/book.xml: $(book_src) $(rust_src)
	@mkdir -p $(dir $@)
	asciidoc \
		-a icons \
		-a version="$(version)" \
		-b docbook45 \
		-o $@ doc/book.asciidoc

out/html/book.html: $(book_src) $(rust_src)
out/html/book.html: $(book_images:doc/%=out/html/%)
out/html/book.html: $(asciidoc_icons:$(asciidoc_icondir)/%=out/html/images/icons/%)
out/html/book.html:
	@mkdir -p $(dir $@)
	asciidoc \
		-a icons \
		-a version="$(version)" \
		-o $@ doc/book.asciidoc

out/html-chunked/index.html: out/docbook/book.xml

out/html/%: doc/%
	@mkdir -p $(dir $@)
	cp $< $@

out/html/images/icons/%: $(asciidoc_icondir)/%
	@mkdir -p $(dir $@)
	cp $< $@

clean:
	rm -rf out/

again: clean deployed

.PHONY: all html pdf deployed clean again
