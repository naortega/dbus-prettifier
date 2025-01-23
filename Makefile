# Copyright (C) 2025 Nicolás Ortega Froysa <nicolas@ortegas.org> All rights reserved.
# Author: Nicolás Ortega Froysa <nicolas@ortegas.org>
#
# This software is provided 'as-is', without any express or implied
# warranty. In no event will the authors be held liable for any damages
# arising from the use of this software.
#
# Permission is granted to anyone to use this software for any purpose,
# including commercial applications, and to alter it and redistribute it
# freely, subject to the following restrictions:
#
# 1. The origin of this software must not be misrepresented; you must not
#    claim that you wrote the original software. If you use this software
#    in a product, an acknowledgment in the product documentation would be
#    appreciated but is not required.
#
# 2. Altered source versions must be plainly marked as such, and must not be
#    misrepresented as being the original software.
#
# 3. This notice may not be removed or altered from any source
#    distribution.

DEBUG=0
INCFLAGS=
LDFLAGS=
DEFS=
CFLAGS=$(INCFLAGS) -std=gnu99 -Wall -Wextra -Wfatal-errors -Werror
HDRS=
OBJS=src/main.o
VERSION=1.0

ifeq ($(PREFIX),)
	PREFIX := /usr/local
endif

ifeq ($(DEBUG),1)
	CFLAGS+=-g -O0
else
	CFLAGS+=-O2 -DNDEBUG
endif

%.o:%.cpp $(HDRS)
	$(CC) -c -o $@ $< $(CFLAGS) -DVERSION=\"$(VERSION)\"

dbus-prettifier: $(OBJS)
	$(CC) -o $@ $^ $(LDFLAGS)

.PHONY: clean distclean install

clean:
	$(RM) $(OBJS)

distclean: clean
	$(RM) dbus-prettifier

install: dbus-prettifier
	install -Dm755 dbus-prettifier $(PREFIX)/bin/
