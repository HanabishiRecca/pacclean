# pacclean

Arch `pacman -Sc` with `CleanMethod = KeepCurrent` analogue, but more verbose and faster.

    $ pacclean
    checking for outdated packages...

    foo-1.2.3-1-x86_64.pkg.tar.zst (1.2 MiB)
    bar-4.5.6-1-x86_64.pkg.tar.zst (50.51 MiB)

    Total packages to remove: 2 (51.71 MiB)

    :: Proceed with removing? [Y/n] y
    removing outdated packages...

## Usage

    $ pacclean [<option>...]

| Option              | Description                                                                                                    |
| ------------------- | -------------------------------------------------------------------------------------------------------------- |
| `--cachedir <path>` | Alternate package cache location. Default value is `/var/cache/pacman/pkg`.                                    |
| `--dbpath <path>`   | Alternate database location. Default value is `/var/lib/pacman`.                                               |
| `--repos <names>`   | Override working repositories. By default all repositories from `dbpath/sync` directory are used. <sup>1</sup> |
| `-h`, `--help`      | Display the help message.                                                                                      |

1. Multiple values could be specified using a comma-separated list.

## Download

You can download prebuilt binaries from [releases](https://github.com/HanabishiRecca/pacclean/releases) page.

## Building from the source

Install dependencies:

-   `libalpm`

Install Rust compiler and run:

    $ cargo build --release
