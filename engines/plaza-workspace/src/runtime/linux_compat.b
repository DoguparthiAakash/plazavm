implement LinuxCompat;

include "sys.m";
	sys: Sys;
include "draw.m";

LinuxCompat: module {
	init: fn(nil: ref Draw->Context, args: list of string);
};

init(nil: ref Draw->Context, args: list of string)
{
	sys = load Sys Sys->PATH;

	sys->print("PlazaVM Linux Compatibility Layer (Skeleton)\n");
	
	if (len args < 2) {
		sys->print("Usage: linux_compat /path/to/elf/binary [args...]\n");
		return;
	}

	# 1. Parse ELF header
	# 2. Map ELF segments into memory
	# 3. Set up stack with Linux-style argc/argv/envp
	# 4. Handle SYSCALL (int 0x80 or syscall instruction) via Inferno exception handler
	#    Translating Linux syscalls (open, read, write) to Inferno sys->open, sys->read
	
	sys->print("Error: ELF loader and Syscall translation not fully implemented.\n");
}
