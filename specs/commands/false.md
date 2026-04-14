# false

## Synopsis
false

## Description

The false command returns an unsuccessful exit status of one. It takes no options and ignores any arguments.

## Options

None. This command accepts no options.

## Operands

Any arguments are ignored.

## Exit Codes

- `1`: Always returns failure

## Examples

    $ false
    $ echo $?
    1

    $ false foo bar
    $ echo $?
    1

## Notes

This is a minimal command used primarily in scripts and shell conditionals.
