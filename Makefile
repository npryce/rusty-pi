
pi=192.168.1.13

deployed: out/blink out/button out/hello
	rsync $^ $(pi):

out/%: src/%.rs src/pi.rs
	@mkdir -p $(dir $@)
	rustc -o $@ -L . --target arm-unknown-linux-gnueabihf -C linker=arm-linux-gnueabihf-g++ $<

clean:
	rm -rf out/

again: clean deployed
