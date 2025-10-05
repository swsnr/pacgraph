# pacgraph

Analyze pacman (ALPM) dependency graphs.

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

But `pacgraph` finds this cycle of unrequired dependencies:

```console
$ pacgraph orphans
libcamera 0.5.2-1
libcamera-ipa 0.5.2-1
```

## License

Licensed under EUPL-1.2 OR GPL-3.0.
