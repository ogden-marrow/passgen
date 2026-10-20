PREFIX  ?= /usr/local
BINDIR  ?= $(PREFIX)/bin
MANDIR  ?= $(PREFIX)/share/man/man1
CARGO   ?= cargo

.PHONY: all build install uninstall clean test

all: build

build:
	$(CARGO) build --release

test:
	$(CARGO) test

install: build
	install -d $(DESTDIR)$(BINDIR) $(DESTDIR)$(MANDIR)
	install -m 755 target/release/passgen $(DESTDIR)$(BINDIR)/passgen
	install -m 644 passgen.1 $(DESTDIR)$(MANDIR)/passgen.1

uninstall:
	rm -f $(DESTDIR)$(BINDIR)/passgen $(DESTDIR)$(MANDIR)/passgen.1

clean:
	$(CARGO) clean
