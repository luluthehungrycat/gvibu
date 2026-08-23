# Shared VIBIX Runtime

## Ownership boundary

GVIBU owns `vibix-lib/` as the shared userland runtime home. VIBIX owns the
kernel and ABI; VIBIT owns PID 1 and supervision; VISH owns shell policy;
GVIBU commands own flags, output bytes, diagnostics, and exit statuses. The
runtime must stay below those policy layers.

```text
VIBIX kernel → SYSCALL.md ABI → vibix-lib
                                  ├→ VIBIT
                                  ├→ VISH
                                  └→ GVIBU commands
```

## API in the first slice

- `vibix_lib::sys`: raw syscall wrappers. Results are ABI `u64` values and
  wrappers explicitly model the VIBIX full caller-clobber register set.
- `vibix_lib::runtime::decode_raw`: converts negative errno encodings and
  `u64::MAX` sentinels before a value is used.
- `vibix_lib::runtime::Fd`: non-negative descriptor newtype with fd 0/1/2
  constants.
- `vibix_lib::runtime::write_all`: allocation-free checked writes over
  borrowed buffers. It retries short writes and rejects zero progress.
- `vibix_lib::runtime::brk`: checked program-break boundary. It is not a
  global allocator and does not expose failure sentinels as heap pointers.

The current flat-binary kernel path registers compatibility syscalls 1/2, so
migrated output uses syscall 1. The ABI's canonical VFS numbers 14/15 are
named in `sys.rs` but remain pending until the VIBIX kernel registers them and
QEMU verifies their behavior.

## Command semantics

The VIBIX `echo` consumer is the first checked path. It preserves its command
specification and successful output. Normal bytes use stdout (fd 1),
diagnostics use stderr (fd 2), runtime output errors return status 1, usage
errors remain status 2 where a command defines them, and success remains 0.
No command flags or shell policy are implemented in this runtime.

## Platform status

| Platform | Runtime/command status | Required evidence |
|---|---|---|
| Linux | Existing writer-based multicall path | Rust tests and parity suite |
| WASM | Existing captured-writer path | WASM build and wrapper checks |
| VIBIX | Checked `echo` slice; flat-binary compatibility path | no-std build, runtime unit tests, VIBIX/QEMU smoke |

Host tests inject raw results and fake sinks; they never issue VIBIX syscalls.
They cannot prove paging, VFS, process, TTY, image packaging, or VIBIT/VISH
integration.

## Required handoffs

- **VIBIX:** register and verify canonical syscall 14/15 behavior, including
  negative errno returns and pointer/descriptor validation; confirm ABI
  versioning and clobber behavior in kernel/QEMU tests.
- **VIBIT:** consume the shared runtime for launch, wait, exit, and standard
  descriptor setup without copying wrappers into init policy.
- **VISH:** consume typed descriptors, checked I/O, and later process/terminal
  APIs while keeping parsing, expansion, and job-control policy in VISH.
- **Image/integration owner:** package the versioned `vibix-lib`, VIBIT, VISH,
  and GVIBU artifacts and run the finite VIBIT → VISH → GVIBU smoke path with
  stdout/stderr, status, redirection or pipes, child reaping, and clean exit.
