# no

## Description

The "no" utility serves as the opposite of the "yes" program, continuously outputting "n" or all arguments passed to it separated by spaces.

## Authors

- Petr Alexandrovich Sabanov

## License

"no" is distributed under the GNU General Public License v3.0. Additional information about the license can be found on the [official website](https://www.gnu.org/licenses/gpl-3.0.html).

## Usage

Usage: no [STRING]...  
or:  no OPTION  
Repeatedly output a line with all specified STRING(s), or 'n'.  

      --help     display this help and exit
      --version  output version information and exit

## Version

#### Rust version

no 1.0.0  
Copyright (C) 2024 Petr Alexandrovich Sabanov  
License GPLv3.0: GNU General Public License v3.0 <https://www.gnu.org/licenses/gpl-3.0.html>

## Examples

1. `no`

    ```
    n
    n
    n
    ... (forever)
    ```

2. `no 1`

    ```
    1
    1
    1
    ... (forever)
    ```

3. `no 1 2 3`

    ```
    1 2 3
    1 2 3
    1 2 3
    ... (forever)
    ```

4. `no | rm -ri /tmp`

    ```
    rm: descend into directory '/tmp'? %
    ```

## Build

1. `git clone <this repository url>`
2. `cd no`
3. `make`