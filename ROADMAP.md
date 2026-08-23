# GVIBU Roadmap

GVIBU owns userland command behavior and the shared VIBIX-facing runtime. This
roadmap is the operational contract for work in this repository and for
handoffs to VIBIX, VIBIT, VISH, and the image integration owner.

Update this file when a runtime/API contract, artifact path, support claim, or
verification gate changes. Every completed milestone must also add an entry to
`CHANGELOG.md` with the files and checks that support the claim.

## Stack ownership

```text
VIBIX kernel/platform
        ↓ SYSCALL.md ABI
vibix-lib shared user runtime
   ├── VIBIT: PID 1, launch, reaping, supervision, shutdown
   ├── VISH: parsing, expansion, readline, shell policy, dispatch
   └── GVIBU: commands, flags, stdout/stderr, exit statuses
        ↓
image integration owner: manifest, initramfs, QEMU/release smoke
```

| Layer | Repository | Owns | Does not own |
|---|---|---|---|
| Kernel/platform | `../vibix-ai-lab` | ABI, memory, processes, VFS, TTY, loaders, devices | commands, shell policy, image source of truth |
| Shared runtime | `vibix-lib/` in GVIBU initially | raw bindings, typed errors, descriptors, checked I/O, allocator-facing primitives | shell parsing, init policy, command flags |
| Init/supervision | `../vibit` | PID 1, service launch, reaping, restart, shutdown | shell implementation, command semantics |
| Shell | `../vish` | parsing, expansion, interactive behavior, builtins, job policy | kernel ABI, init policy, coreutils implementation |
| Commands | this repository | coreutils behavior, flags, output, status, VIBIX command artifacts | kernel mechanisms, shell policy, service supervision |
| Image integration | separate integration owner | versioned manifest, artifact assembly, initramfs, QEMU/release checks | layer implementation details |

Sibling repositories remain read-only from this worktree. Cross-repository
changes require an explicit handoff naming the owner, contract, file, and
verification impact.

## Current status

### Completed locally

- Linux Rust multicall implementation and Python reference for the documented
  command set.
- Existing writer-based Linux/WASM command boundary.
- `vibix-lib` typed runtime slice:
  - ABI raw-result decoding for errno and `u64::MAX` sentinels;
  - non-negative descriptor type with fd 0/1/2 constants;
  - allocation-free checked writes with short-write and zero-progress handling;
  - checked `brk` boundary without a global allocator;
  - inline assembly that declares the VIBIX caller-clobber contract.
- VIBIX `echo` migrated to checked runtime output while preserving flags,
  successful bytes, and exit semantics.
- Host-safe runtime contract tests and focused Linux echo tests.
- OpenSpec change `establish-gvibu-vibix-runtime` with proposal, design,
  capability specs, and tasks.

### In progress

- Handoff of `vibix-lib` to VIBIT and VISH as the shared runtime contract.
- Image artifact manifest and finite VIBIT → VISH → GVIBU verification.
- Broader VIBIX command migration after the first runtime consumer is proven.

### Not claimed

- GVIBU is not yet fully VIBIX-portable.
- Building `vibix.bin` does not prove VFS, process, TTY, descriptor inheritance,
  or full-stack behavior.
- Canonical VIBIX VFS syscall 14/15 support is not available from GVIBU until
  the VIBIX kernel registers and verifies those slots.
- A global allocator is intentionally not installed yet.

## GVIBU image artifacts

The image owner must consume explicit artifacts, not infer them from a Linux
build. The following is the current manifest contract.

| Artifact | Build command | Format / target | Current command set | Runtime dependencies | Status |
|---|---|---|---|---|---|
| `vibix-lib/vibix.bin` | `make -C vibix-lib` | Flat x86-64 binary, `x86_64-unknown-none` | `true`, `false`, `echo`, `yes`, `printenv`; `gvibu`/`vibix` subcommand dispatch | VIBIX flat-binary loader; startup registers; compatibility syscall 1/2; `vibix-lib` runtime | Built locally; full VIBIT/VISH/QEMU path unverified |
| `vibix-lib/target/x86_64-unknown-none/release/vibix` | `cargo build --release --manifest-path vibix-lib/Cargo.toml` | Intermediate ELF for objcopy | Same as above | Same as above | Build intermediate; do not substitute the Linux binary |
| `kernel/user_<name>.bin` | `make vibix-<name>` or `make vibix-all` | NASM flat compatibility fixture | Individual legacy user commands defined under `kernel/` | Direct VIBIX ABI assembly helpers; not the Rust shared runtime | Compatibility path; retain until Rust parity is demonstrated |
| `rust/target/release/gvibu` | `cargo build --release --manifest-path rust/Cargo.toml` | Linux host binary | Linux command set | Linux `std`/libc environment | Linux-only; not a VIBIX image artifact |

The integration manifest must add, for each copied artifact:

- source repository and revision;
- source path and destination path;
- file mode and ownership;
- binary format and target triple;
- VIBIX ABI version;
- runtime dependency version or source revision;
- build command and reproducibility inputs;
- checksum and verification result.

### Proposed image mapping

The exact image paths belong to the integration owner, but the first Rust
multicall handoff should map `vibix-lib/vibix.bin` to one explicit GVIBU
entrypoint such as `/bin/gvibu`, with command aliases only where the loader
and VISH agree on argv[0] dispatch. Do not copy `rust/target/release/gvibu`
into a VIBIX image: it is a Linux binary.

VIBIT and VISH must agree on how the flat binary receives `argc`, `argv`, and
`envp`, and on its initial stack. The current GVIBU entrypoint reads those
values from `rdi`, `rsi`, and `rdx` and establishes its own stack; this startup
contract must be reconciled with the VIBIX loader contract before release.

## VIBIT handoff contract

VIBIT requested two concrete deliverables: command artifacts/runtime
dependencies, and verification of command status, descriptor inheritance, and
execution through VISH.

### Artifact handoff

GVIBU can provide now:

1. `vibix-lib/vibix.bin` from `make -C vibix-lib`.
2. The `vibix-lib` source/runtime contract from:
   - `vibix-lib/src/sys.rs`;
   - `vibix-lib/src/runtime.rs`;
   - `vibix-lib/src/lib.rs`;
   - `vibix-lib/Cargo.toml` and `vibix-lib/Makefile`.
3. The supported first-slice command set: `true`, `false`, `echo`, `yes`,
   and `printenv`.
4. The legacy NASM compatibility artifacts from `make vibix-<name>` where the
   image still needs them.
5. The ABI references consumed by the runtime: VIBIX `SYSCALL.md` and the
   agreed startup/descriptor contract.

VIBIT must not depend on the Linux `rust/target/release/gvibu` artifact or
copy GVIBU source into the VIBIT repository.

### Behavior verification through VISH

The integration owner should run these finite scenarios in the supported
VIBIX image:

| Scenario | Expected result |
|---|---|
| `true` | no output; status `0` |
| `false` | no stdout; status `1` |
| `echo hello` | stdout `hello\n`; empty stderr; status `0` |
| `echo -n hello` | stdout `hello`; status `0` |
| `printenv` with inherited environment | expected environment output; status `0` |
| unknown command | diagnostic on stderr; status `1` |
| child command through VISH | VISH observes the command's exit status after `waitpid` |
| stdout redirection | child writes to the redirected descriptor; parent shell remains usable |
| pipe, when enabled by VISH | producer/consumer receive inherited pipe descriptors; both children are reaped |

These checks must execute the real path:

```text
VIBIT launches VISH
  → VISH resolves/executes the GVIBU artifact
  → fork/exec preserves standard descriptors
  → command writes stdout/stderr and exits
  → VISH waits and reports status
  → VIBIT reaps the child
```

### Descriptor inheritance assertions

The test harness must verify, not assume:

- VIBIT establishes valid fd 0, 1, and 2 before launching VISH.
- `fork()` gives the child the inherited descriptor table.
- `exec()` preserves descriptors below the close-on-exec boundary and closes
  descriptors `>= 3` according to the VIBIX contract.
- `dup2()` and pipe descriptors are the exact descriptors used by redirected
  commands.
- invalid or closed descriptors produce an error/status failure, not a
  successful command result.
- parent VISH and VIBIT reap all command children and do not leave zombies.

The current GVIBU host tests cover typed conversion and checked-write logic,
not kernel descriptor inheritance. That evidence must come from VIBIX/QEMU.

## Milestones

### M1 — Runtime contract

- [x] Typed raw-result conversion and sentinel handling.
- [x] Descriptor and checked-write API.
- [x] `brk` boundary without a global allocator.
- [x] Full inline-assembly clobber declarations.
- [x] One checked Rust VIBIX command consumer.
- [x] Focused host-safe tests.

### M2 — Artifact handoff

- [ ] Publish a versioned artifact manifest for `vibix.bin` and any retained
      NASM command fixtures.
- [ ] Record ABI version, target, startup convention, runtime revision, and
      checksums in the integration checkout.
- [ ] Make VIBIT/VISH consume the shared runtime contract without duplicate
      syscall wrappers.

### M3 — VIBIT/VISH execution proof

- [ ] VIBIT launches the agreed VISH artifact with fd 0/1/2 established.
- [ ] VISH executes `true`, `false`, `echo`, and `printenv` through GVIBU.
- [ ] Status values and stderr are verified after `waitpid`.
- [ ] Descriptor inheritance is verified across fork/exec.
- [ ] Redirection or pipes, child reaping, and clean exit are verified.

### M4 — Canonical VIBIX runtime

- [ ] VIBIX registers and tests canonical VFS syscall 14/15.
- [ ] GVIBU switches consumers only after the kernel handoff passes QEMU.
- [ ] Runtime process, descriptor, and terminal APIs are extended only as
      required by VISH/VIBIT consumers.

### M5 — Broader command portability

- [ ] Migrate additional small VIBIX commands one at a time.
- [ ] Preserve each command specification and Linux/Python parity behavior.
- [ ] Add command-specific VIBIX smoke cases before marking a command portable.
- [ ] Do not migrate all commands as one change.

### M6 — Release image

- [ ] Separate integration owner assembles reproducible VIBIX/VIBIT/VISH/GVIBU
      image artifacts.
- [ ] Full finite QEMU smoke passes from boot through clean shutdown.
- [ ] Release manifest records provenance and verification for every artifact.

## Ownership decision: keep image integration separate

Yes. The image integration owner should be separate from the VIBIX kernel
repository.

VIBIX should retain a small convenience target such as `make INIT=vibit run`
for developer smoke tests, but the source of truth for image assembly should
live in a dedicated integration repository or checkout. This keeps kernel ABI
changes, userland artifacts, init policy, and release packaging independently
reviewable and prevents a kernel build from silently becoming the owner of
VIBIT/VISH/GVIBU source or artifact policy.

The separate owner should own:

- the artifact manifest and provenance checks;
- initramfs/image assembly;
- exact repository revisions and ABI compatibility;
- QEMU boot and finite behavior tests;
- release image publication and rollback.

VIBIX should own only the kernel/platform and ABI tests. GVIBU, VIBIT, and VISH
should each provide versioned artifacts and consumer contracts to the
integration owner.

## Blockers

- VIBIX kernel source currently does not register canonical syscall 14/15,
  although the ABI document specifies them.
- The initial stack/entry-register contract needs one authoritative agreement
  between the VIBIX loader, VIBIT, VISH, and the GVIBU Rust flat entrypoint.
- Full QEMU verification is not available from this repository alone.
- `vibix-lib` is not yet a separately versioned/published package.
- The full Rust VIBIX command set and process/terminal APIs remain incomplete.
- Existing repository-wide Linux parity, CLI, and fuzz suites contain unrelated
  failures; they must be isolated before release claims.
