# GVIBU id Primary Group Design

## Goal

Make `id -G` report the process's primary group even when the operating system returns no supplementary groups.

## Design

For `-G`, start with the supplementary group IDs and add the selected primary ID only when absent: effective GID by default, real GID with `-r`. Preserve existing supplementary group order and avoid duplicates. With `-n`, resolve the resulting IDs using the existing group-name lookup. Keep default `id`, `-g`, and `-u` behavior unchanged.

Implement the same rule in the Rust and Python implementations. Keep group-list construction independently testable so coverage can explicitly provide an empty supplementary list and a non-empty list that already contains the primary ID; integration coverage must also assert that `id -G` includes the selected primary ID in the current environment.

## Validation

Run the focused Rust CLI and Python tests first, then the complete Rust test suite and Python test suite. Run the existing Python/Rust parity cases for `id`. Tests must cover numeric and name output, real-group selection, empty supplementary groups, and duplicate suppression.

## Constraints

- Do not add dependencies or change lockfiles.
- Do not alter behavior for unrelated identity flags.
- Do not depend on the test runner having supplementary groups configured.
