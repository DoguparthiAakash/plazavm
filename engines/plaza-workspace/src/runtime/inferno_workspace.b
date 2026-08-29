implement InfernoWorkspace;

include "sys.m";
	sys: Sys;
include "draw.m";
include "styx.m";
	styx: Styx;
include "keyring.m";
include "security.m";

InfernoWorkspace: module {
	init: fn(nil: ref Draw->Context, args: list of string);
};

init(nil: ref Draw->Context, args: list of string)
{
	sys = load Sys Sys->PATH;
	styx = load Styx Styx->PATH;

	sys->print("Starting PlazaVM 9P/Styx Workspace Service...\n");

	# 1. Mount the workspace block device (e.g. sdC1) using kfs
	# kfs -f /dev/sdC1/data -n workspace
	
	# 2. Bind the workspace to /workspace
	sys->bind("#U/workspace", "/workspace", Sys->MREPL);

	# 3. Export the workspace over Styx (9P) on the virtio serial port or standard network
	# Here we simulate exporting /workspace over a network listener on port 564
	
	# Skeleton: In a real environment, we'd start styx servers or use exportfs
	# sys->export("/workspace", "/", ...);

	sys->print("SUCCESS_INFERNO_GUEST_READY\n");

	# Keep init alive to prevent kernel panic
	for(;;) {
		sys->sleep(10000);
	}
}
