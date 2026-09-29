implement LinuxSyscall;

# Linux System Call Translation Layer for Inferno
#
# Translates Linux x86_64 syscalls to Inferno primitives.
# This enables running statically-linked Linux binaries inside
# the Inferno guest runtime with minimal memory overhead.
#
# Target: 1-15MB per workspace (vs Docker's ~50-200MB)
#
# Reference: Linux syscall table for x86_64
# https://blog.rchapman.org/posts/Linux_System_Call_Table_for_x86_64/

include "sys.m";
	sys: Sys;
include "draw.m";
include "styx.m";
	styx: Styx;
include "lock.m";
	lock: Lock;

# Syscall numbers (x86_64 Linux)
SYS_READ: con 0;
SYS_WRITE: con 1;
SYS_OPEN: con 2;
SYS_CLOSE: con 3;
SYS_STAT: con 4;
SYS_FSTAT: con 5;
SYS_LSTAT: con 6;
SYS_LSEEK: con 7;
SYS_MMAP: con 9;
SYS_MPROTECT: con 10;
SYS_MUNMAP: con 11;
SYS_BRK: con 12;
SYS_IOCTL: con 16;
SYS_PIPE: con 22;
SYS_DUP: con 32;
SYS_DUP2: con 33;
SYS_NANOSLEEP: con 35;
SYS_GETPID: con 39;
SYS_SOCKET: con 41;
SYS_CONNECT: con 42;
SYS_ACCEPT: con 43;
SYS_SENDTO: con 44;
SYS_RECVFROM: con 45;
SYS_EXECVE: con 59;
SYS_EXIT: con 60;
SYS_WAIT4: con 61;
SYS_KILL: con 62;
SYS_UNAME: con 63;
SYS_FCNTL: con 72;
SYS_GETCWD: con 79;
SYS_CHDIR: con 80;
SYS_MKDIR: con 83;
SYS_RMDIR: con 84;
SYS_LINK: con 86;
SYS_UNLINK: con 87;
SYS_CHMOD: con 90;
SYS_CHOWN: con 92;
SYS_UMASK: con 95;
SYS_GETTIMEOFDAY: con 96;
SYS_GETUID: con 102;
SYS_GETGID: con 104;
SYS_GETEUID: con 107;
SYS_GETEGID: con 108;
SYS_SETUID: con 105;
SYS_SETGID: con 106;
SYS_GETPPID: con 110;
SYS_SETSID: con 112;
SYS_SIGALTSTACK: con 131;
SYS_RT_SIGACTION: con 13;
SYS_RT_SIGPROCMASK: con 14;
SYS_GETRUSAGE: con 98;
SYS_SYSINFO: con 99;
SYS_MSYNC: con 26;
SYS_FORK: con 57;
SYS_VFORK: con 58;
SYS_CLONE: con 56;
SYS_GETTID: con 186;
SYS_TGKILL: con 234;
SYS_OPENAT: con 257;
SYS_MKDIRAT: con 258;
SYS_UNLINKAT: con 263;
SYS_RENAMEAT: con 264;
SYS_FSTATAT: con 262;
SYS_READLINKAT: con 267;
SYS_FCHMODAT: con 268;
SYS_FCHOWNAT: con 260;
SYS_GETRANDOM: con 318;
SYS_MEMFD_CREATE: con 319;
SYS_COPY_FILE_RANGE: con 326;
SYS_PREAD64: con 17;
SYS_PWRITE64: con 18;
SYS_PREADV: con 295;
SYS_PWRITEV: con 296;
SYS_ACCESS: con 21;
SYS_FACCESSAT: con 269;

LinuxSyscall: module {
	init: fn(nil: ref Draw->Context, args: list of string);

	# Translate a Linux syscall to Inferno operations
	handle_syscall: fn(syscall_num: int, arg1, arg2, arg3, arg4, arg5, arg6: int): int;
};

# State for a Linux process running under Inferno
LinuxProcess: adt {
	pid: int;
	ppid: int;
	uid: int;
	gid: int;
	cwd: string;
	root: string;
	env: list of string;
	fd_map: array of ref FD;
	heap_top: int;		# brk target
	stack_top: int;
};

# File descriptor mapping
FD: adt {
	inferno_path: string;
	fd: int;		# Inferno file descriptor
	flags: int;		# Linux open flags
	offset: int;		# Current file offset
};

proc_table: ref Table[ref LinuxProcess, int];	# pid -> process
next_pid: int;

init(nil: ref Draw->Context, args: list of string)
{
	sys = load Sys Sys->PATH;
	styx = load Styx Styx->PATH;
	lock = load Lock Lock->PATH;

	sys->print("Linux Syscall Translation Layer v0.1\n");
	sys->print("Target: 1-15MB per workspace\n");

	proc_table = Table[ref LinuxProcess, int].new(64);
	next_pid = 1;

	# Start with init process
	init_proc := ref LinuxProcess;
	init_proc.pid = 1;
	init_proc.ppid = 0;
	init_proc.uid = 0;
	init_proc.gid = 0;
	init_proc.cwd = "/workspace";
	init_proc.root = "/";
	init_proc.fd_map = array[256] of ref FD;
	init_proc.heap_top = 16r10000000;	# 256MB virtual (not physical!)
	init_proc.stack_top = 16r7ffffff000;

	proc_table.add(1, init_proc);

	sys->print("Syscall layer initialized. PID namespace ready.\n");
}

# Handle a Linux x86_64 syscall
# Returns the syscall return value
handle_syscall(syscall_num: int, a1, a2, a3, a4, a5, a6: int): int
{
	proc := get_current_proc();
	if(proc == nil)
		return -1;	# ESRCH

	case syscall_num {
	SYS_READ =>
		return sys_read(proc, a1, a2, a3);
	SYS_WRITE =>
		return sys_write(proc, a1, a2, a3);
	SYS_OPEN =>
		return sys_open(proc, a1, a2, a3);
	SYS_CLOSE =>
		return sys_close(proc, a1);
	SYS_STAT =>
		return sys_stat(proc, a1, a2);
	SYS_FSTAT =>
		return sys_fstat(proc, a1, a2);
	SYS_LSEEK =>
		return sys_lseek(proc, a1, a2, a3);
	SYS_MMAP =>
		return sys_mmap(proc, a1, a2, a3, a4, a5, a6);
	SYS_MUNMAP =>
		return sys_munmap(proc, a1, a2);
	SYS_MPROTECT =>
		return 0;	# No-op on Inferno (no W^X enforcement needed)
	SYS_BRK =>
		return sys_brk(proc, a1);
	SYS_PIPE =>
		return sys_pipe(proc, a1);
	SYS_DUP =>
		return sys_dup(proc, a1);
	SYS_DUP2 =>
		return sys_dup2(proc, a1, a2);
	SYS_NANOSLEEP =>
		sys->sleep(a1 / 1000000);	# Convert ns to ms
		return 0;
	SYS_GETPID =>
		return proc.pid;
	SYS_GETPPID =>
		return proc.ppid;
	SYS_FORK =>
		return sys_fork(proc);
	SYS_VFORK =>
		return sys_fork(proc);
	SYS_CLONE =>
		return sys_clone(proc, a1, a2, a3, a4);
	SYS_EXECVE =>
		return sys_execve(proc, a1, a2, a3);
	SYS_EXIT =>
		sys_exit(proc, a1);
		return 0;	# Never reached
	SYS_WAIT4 =>
		return sys_wait4(proc, a1, a2, a3);
	SYS_KILL =>
		return sys_kill(proc, a1, a2);
	SYS_UNAME =>
		return sys_uname(proc, a1);
	SYS_FCNTL =>
		return 0;	# Simplified
	SYS_GETCWD =>
		return sys_getcwd(proc, a1, a2);
	SYS_CHDIR =>
		return sys_chdir(proc, a1);
	SYS_MKDIR =>
		return sys_mkdir(proc, a1, a2);
	SYS_RMDIR =>
		return sys_rmdir(proc, a1);
	SYS_UNLINK =>
		return sys_unlink(proc, a1);
	SYS_CHMOD =>
		return 0;	# No-op: Inferno uses own permissions
	SYS_CHOWN =>
		return 0;
	SYS_UMASK =>
		return 16r0022;	# Standard umask
	SYS_GETTIMEOFDAY =>
		return sys_gettimeofday(proc, a1, a2);
	SYS_GETUID =>
		return proc.uid;
	SYS_GETGID =>
		return proc.gid;
	SYS_GETEUID =>
		return proc.uid;
	SYS_GETEGID =>
		return proc.gid;
	SYS_SETUID =>
		proc.uid = a1;
		return 0;
	SYS_SETGID =>
		proc.gid = a1;
		return 0;
	SYS_SETSID =>
		return proc.pid;
	SYS_SIGALTSTACK =>
		return 0;	# Stub
	SYS_RT_SIGACTION =>
		return 0;	# Stub
	SYS_RT_SIGPROCMASK =>
		return 0;	# Stub
	SYS_GETRUSAGE =>
		return sys_getrusage(proc, a1, a2);
	SYS_SYSINFO =>
		return sys_sysinfo(proc, a1);
	SYS_GETTID =>
		return proc.pid;
	SYS_TGKILL =>
		return 0;	# Stub
	SYS_ACCESS =>
		return sys_access(proc, a1, a2);
	SYS_GETRANDOM =>
		return sys_getrandom(proc, a1, a2);
	SYS_MMAP|SYS_MMAP+0x10000 =>	# Handle both compat
		return sys_mmap(proc, a1, a2, a3, a4, a5, a6);
	SYS_OPENAT =>
		return sys_openat(proc, a1, a2, a3, a4);
	* =>
		# Unknown syscall - log and return -ENOSYS
		sys->print("Unhandled Linux syscall: %d\n", syscall_num);
		return -38;	# -ENOSYS
	}
}

# === File I/O syscalls ===

sys_read(proc: ref LinuxProcess, fd: int, buf: int, count: int): int
{
	if(fd < 0 || fd >= len proc.fd_map || proc.fd_map[fd] == nil)
		return -9;	# -EBADF

	f := proc.fd_map[fd];
	d := array of byte buf;
	n := sys->pread(d, count, f.offset, f.inferno_path);
	if(n > 0)
		f.offset += n;
	return n;
}

sys_write(proc: ref LinuxProcess, fd: int, buf: int, count: int): int
{
	if(fd < 0 || fd >= len proc.fd_map || proc.fd_map[fd] == nil)
		return -9;

	f := proc.fd_map[fd];

	# Handle stdout/stderr specially - print to serial
	if(fd == 1 || fd == 2) {
		d := array of byte buf;
		text := string d[:count];
		sys->print("%s", text);
		return count;
	}

	d := array of byte buf;
	n := sys->pwrite(d, count, f.offset, f.inferno_path);
	if(n > 0)
		f.offset += n;
	return n;
}

sys_open(proc: ref LinuxProcess, path_ptr: int, flags: int, mode: int): int
{
	# Read path from process memory (simplified - in real impl, read from address space)
	path := read_user_string(path_ptr);
	if(path == nil)
		return -14;	# -EFAULT

	# Translate Linux path to Inferno path
	inferno_path := translate_path(path);

	# Map Linux open flags to Inferno open flags
	o_flags := 0;
	if(flags & 16r100 != 0)	# O_TRUNC
		o_flags |= Sys->OTRUNC;
	if(flags & 16r200 != 0)	# O_CREAT
		o_flags |= Sys->OCEXEC;	# Approximate

	fd_num := find_free_fd(proc);
	if(fd_num < 0)
		return -24;	# -EMFILE

	inf_fd := sys->open(inferno_path, o_flags);
	if(inf_fd < 0)
		return -2;	# -ENOENT

	proc.fd_map[fd_num] = ref FD(inferno_path, inf_fd, flags, 0);
	return fd_num;
}

sys_close(proc: ref LinuxProcess, fd: int): int
{
	if(fd < 0 || fd >= len proc.fd_map || proc.fd_map[fd] == nil)
		return -9;

	f := proc.fd_map[fd];
	if(f.fd >= 0)
		sys->close(f.fd);
	proc.fd_map[fd] = nil;
	return 0;
}

sys_fstat(proc: ref LinuxProcess, fd: int, stat_buf: int): int
{
	# Simplified stat - fill in minimal linux_stat structure
	# In production, read actual file info from Inferno
	if(fd < 0 || fd >= len proc.fd_map || proc.fd_map[fd] == nil)
		return -9;

	# Write a minimal stat structure (Linux x86_64 struct stat)
	# This is a stub - real implementation needs proper struct packing
	return 0;
}

sys_lseek(proc: ref LinuxProcess, fd: int, offset: int, whence: int): int
{
	if(fd < 0 || fd >= len proc.fd_map || proc.fd_map[fd] == nil)
		return -9;

	f := proc.fd_map[fd];
	case whence {
	0 => f.offset = offset;	# SEEK_SET
	1 => f.offset += offset;	# SEEK_CUR
	2 => f.offset = 0;		# SEEK_END (simplified)
	}
	return f.offset;
}

# === Memory management ===

sys_mmap(proc: ref LinuxProcess, addr: int, length: int, prot: int, flags: int, fd: int, offset: int): int
{
	# For now, implement mmap as a simple bump allocator
	# Real implementation would need proper virtual memory

	# Align to page boundary
	length = (length + 4095) & ~4095;

	if(addr == 0) {
		# Let kernel choose address
		addr = proc.heap_top;
		proc.heap_top += length;
	}

	# If fd is provided, map the file
	if(fd >= 0 && fd < len proc.fd_map && proc.fd_map[fd] != nil) {
		f := proc.fd_map[fd];
		d := array[length];
		n := sys->pread(d, length, offset, f.inferno_path);
		if(n > 0 && n < length)
			d = d[:n];
		# In real implementation, copy d to process address space
	}

	return addr;
}

sys_munmap(proc: ref LinuxProcess, addr: int, length: int): int
{
	# No-op for now (Inferno doesn't track individual mappings)
	return 0;
}

sys_brk(proc: ref LinuxProcess, new_brk: int): int
{
	if(new_brk == 0)
		return proc.heap_top;

	if(new_brk > proc.heap_top)
		proc.heap_top = new_brk;

	return proc.heap_top;
}

# === Process management ===

sys_fork(proc: ref LinuxProcess): int
{
	new_pid := alloc_pid();

	new_proc := ref LinuxProcess;
	new_proc.pid = new_pid;
	new_proc.ppid = proc.pid;
	new_proc.uid = proc.uid;
	new_proc.gid = proc.gid;
	new_proc.cwd = proc.cwd;
	new_proc.root = proc.root;
	new_proc.fd_map = array[len proc.fd_map] of ref FD;

	# Clone file descriptors
	for(i := 0; i < len proc.fd_map; i++) {
		if(proc.fd_map[i] != nil) {
			new_proc.fd_map[i] = ref FD(
				proc.fd_map[i].inferno_path,
				proc.fd_map[i].fd,
				proc.fd_map[i].flags,
				proc.fd_map[i].offset,
			);
		}
	}

	new_proc.heap_top = proc.heap_top;
	new_proc.stack_top = proc.stack_top;

	proc_table.add(new_pid, new_proc);

	# In real implementation, this would use Inferno rfork
	# For now, just register the process
	sys->print("fork: pid %d -> %d\n", proc.pid, new_pid);

	return new_pid;	# Returns 0 to child, new_pid to parent
}

sys_clone(proc: ref LinuxProcess, flags: int, child_stack: int, ptid: int, ctid: int): int
{
	# Simplified clone - treat as fork for now
	return sys_fork(proc);
}

sys_execve(proc: ref LinuxProcess, path_ptr: int, argv_ptr: int, envp_ptr: int): int
{
	path := read_user_string(path_ptr);
	if(path == nil)
		return -14;

	inferno_path := translate_path(path);
	sys->print("execve: %s -> %s\n", path, inferno_path);

	# In real implementation:
	# 1. Parse ELF binary
	# 2. Map segments into process address space
	# 3. Set up stack with argv and envp
	# 4. Jump to entry point

	# For now, try to run as Inferno binary
	fd := sys->open(inferno_path, Sys->OREAD);
	if(fd < 0)
		return -2;	# -ENOENT

	sys->close(fd);
	return 0;
}

sys_exit(proc: ref LinuxProcess, status: int): int
{
	# Close all file descriptors
	for(i := 0; i < len proc.fd_map; i++) {
		if(proc.fd_map[i] != nil) {
			if(proc.fd_map[i].fd >= 0)
				sys->close(proc.fd_map[i].fd);
			proc.fd_map[i] = nil;
		}
	}

	proc_table.del(proc.pid);
	sys->print("exit: pid %d status %d\n", proc.pid, status);

	# In real implementation, this would terminate the process
	# For init (pid 1), this would reboot
	if(proc.pid == 1) {
		sys->print("INIT EXIT - Rebooting...\n");
		# Would trigger reboot
	}

	return 0;
}

sys_wait4(proc: ref LinuxProcess, pid: int, status_ptr: int, options: int): int
{
	# Simplified wait - just return immediately
	# Real implementation needs process state tracking
	return -10;	# -ECHILD (no children)
}

sys_kill(proc: ref LinuxProcess, pid: int, sig: int): int
{
	target := proc_table.find(pid);
	if(target == nil)
		return -3;	# -ESRCH

	# Simplified - just handle SIGKILL
	if(sig == 9) {
		sys_exit(target, -9);
	}
	return 0;
}

# === System information ===

sys_uname(proc: ref LinuxProcess, buf: int): int
{
	# Fill in Linux utsname structure
	# sysname, nodename, release, version, machine
	# Return "Linux" compatible strings
	name := "Linux";
	nodename := "plazavm-inferno";
	release := "6.6.110";
	version := "#1-SMP";
	machine := "x86_64";

	# In real implementation, write to process memory
	return 0;
}

sys_getcwd(proc: ref LinuxProcess, buf: int, size: int): int
{
	cwd := proc.cwd;
	if(len cwd >= size)
		return -34;	# -ERANGE

	# Copy cwd to process buffer
	return 0;
}

sys_chdir(proc: ref LinuxProcess, path_ptr: int): int
{
	path := read_user_string(path_ptr);
	if(path == nil)
		return -14;

	proc.cwd = translate_path(path);
	return 0;
}

sys_mkdir(proc: ref LinuxProcess, path_ptr: int, mode: int): int
{
	path := read_user_string(path_ptr);
	if(path == nil)
		return -14;

	inferno_path := translate_path(path);
	# Use Inferno create to make directory
	fd := sys->create(inferno_path, Sys->OWRITE, 8r755);
	if(fd >= 0) {
		sys->close(fd);
		return 0;
	}
	return -2;
}

sys_rmdir(proc: ref LinuxProcess, path_ptr: int): int
{
	path := read_user_string(path_ptr);
	if(path == nil)
		return -14;

	inferno_path := translate_path(path);
	sys->remove(inferno_path);
	return 0;
}

sys_unlink(proc: ref LinuxProcess, path_ptr: int): int
{
	path := read_user_string(path_ptr);
	if(path == nil)
		return -14;

	inferno_path := translate_path(path);
	sys->remove(inferno_path);
	return 0;
}

sys_gettimeofday(proc: ref LinuxProcess, tv_ptr: int, tz_ptr: int): int
{
	# Return current time
	# In real implementation, write timeval to process memory
	return 0;
}

sys_getrusage(proc: ref LinuxProcess, who: int, rusage_ptr: int): int
{
	# Return minimal resource usage
	return 0;
}

sys_sysinfo(proc: ref LinuxProcess, buf_ptr: int): int
{
	# Return system info - importantly, total/free memory
	# This is where we report the 1-15MB budget
	return 0;
}

sys_getrandom(proc: ref LinuxProcess, buf_ptr: int, count: int): int
{
	# Generate random bytes using Inferno's random device
	fd := sys->open("/dev/random", Sys->OREAD);
	if(fd < 0)
		return -2;

	d := array[count];
	n := sys->read(fd, d, count);
	sys->close(fd);
	return n;
}

sys_access(proc: ref LinuxProcess, path_ptr: int, mode: int): int
{
	path := read_user_string(path_ptr);
	if(path == nil)
		return -14;

	inferno_path := translate_path(path);
	fd := sys->open(inferno_path, Sys->OREAD);
	if(fd < 0)
		return -2;	# -ENOENT

	sys->close(fd);
	return 0;
}

# === Utility functions ===

# Translate Linux path to Inferno namespace path
translate_path(linux_path: string): string
{
	if(linux_path == nil)
		return nil;

	# Map common Linux paths to Inferno namespace
	if(linux_path[0] == '/') {
		if(linux_path[:10] == "/workspace/") {
			return linux_path;
		} else if(linux_path[:6] == "/home/") {
			# /home/user/... -> /workspace/...
			parts := sys->tokenize(linux_path, "/");
			if(len parts > 2)
				return "/workspace/" + hd tl parts;
			return "/workspace";
		} else if(linux_path[:5] == "/tmp/") {
			return "/tmp/" + linux_path[5:];
		} else if(linux_path[:5] == "/dev/") {
			return linux_path;	# /dev is same
		} else if(linux_path[:5] == "/bin/" || linux_path[:5] == "/usr/") {
			return linux_path;	# Pass through
		} else if(linux_path[:4] == "/lib") {
			return linux_path;
		}
	}

	# Relative path - resolve against cwd
	return linux_path;
}

read_user_string(addr: int): string
{
	# In real implementation, read bytes from process address space
	# For now, return a stub
	return "";
}

find_free_fd(proc: ref LinuxProcess): int
{
	for(i := 3; i < len proc.fd_map; i++) {
		if(proc.fd_map[i] == nil)
			return i;
	}
	return -1;	# No free FDs
}

alloc_pid(): int
{
	pid := next_pid;
	next_pid++;
	return pid;
}

get_current_proc(): ref LinuxProcess
{
	# In real implementation, use current CPU's process
	return proc_table.find(1);
}
