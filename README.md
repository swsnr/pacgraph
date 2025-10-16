# packit

Pacman Toolkit.

A random collection of mostly pacman related helper commands.

## Features

- Find orphans, across dependency cycles.

## Example

`pacman -Qtd` fails to recognize self-referenced dependency cycles:

```console
$ pacman -Qi libcamera libcamera-ipa | rg -i 'Name|Version|Required By|Optional For'
Name            : libcamera
Version         : 0.5.2-1
Required By     : libcamera-ipa
Optional For    : None
Name            : libcamera-ipa
Version         : 0.5.2-1
Required By     : libcamera
Optional For    : None
$ pacman -Qtd
```

But `packit` finds this cycle of unrequired dependencies:

```console
$ packit orphans
libcamera 0.5.2-1
libcamera-ipa 0.5.2-1
```

## License

Licensed under EUPL-1.2 OR GPL-3.0.
