# Demon

C function replacements for `halo_cache_symbols.exe` (2020 digsite build) and friends.

Symbol naming should match as much as possible thanks to all of the debugging information around on the internet (Mainly the HCEX 360 build).
Despite the name, `halo_cache_symbols.exe` was not released with matching symbols, so some names have to be guessed.

The code is not aiming to be byte-matching, but aims to do the same thing. Float math should be bit-perfect on the same CPU.

Technical bugs will be fixed, but gameplay should not change at all as a result of this.

## building

`demon.dll` should be built with 32-bit mingw-w64 using CMake, this is where all of the replacement code is.
`scythunk.dll` should be built with Rust for i686-pc-windows-gnu.

The b3sum of [halo_cache_symbols.exe](http://vaporeon.io/hosted/halo/original_files/misc/haloce_2020_debug.7z) used by this project is 98AE187C336235C5B1262C80A3EE42AA7FCB82D5D4158EAF899835B60CC40988

Patch `halo_cache_symbols.exe` with `halo_cache_demon.bps` and place `demon.dll`  and `scythunk.dll` next to the exe.
When using the patched exe functions will be replaced with the implemented versions on load.

I recommended testing with the original 2003 Halo PC maps. They will be supported when `demon.dll` is loaded.
