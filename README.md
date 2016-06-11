# rustendo64 [![Build Status](https://travis-ci.org/yupferris/rustendo64.svg?branch=master)](https://travis-ci.org/yupferris/rustendo64) [![Build Status](https://ci.appveyor.com/api/projects/status/owjloq84v91147nd/branch/master?svg=true)](https://ci.appveyor.com/project/yupferris/rustendo64/branch/master)

![rustendo64](Rustendo-64.png)

Livecoding a Nintendo 64 emulator in Rust :D

## Follow along
The entire process is currently being streamed on [Twitch](http://www.twitch.tv/ferrisstreamsstuff), and each segment is being recorded and uploaded to [this YouTube playlist](https://www.youtube.com/playlist?list=PL-sXmdrqqYYcznDg4xwAJWQgNL2gRray2). For stream times and announcements, you can check out [my Twitter](https://twitter.com/ferristweetsnow).

At the end of each episode, I mark the latest commit with a tag so you can see where we finished. Check the [releases](https://github.com/yupferris/rustendo64/releases) for this repo to see those.

## Helpful tools
- [Hex Fiend](http://ridiculousfish.com/hexfiend/)
- [Online disassembler](https://www.onlinedisassembler.com/odaweb/)
- [Dash](https://kapeli.com/dash) (OS X / iOS) for documentation. Compatible alternatives for other platforms can be found linked from [Dash Docset Links](https://kapeli.com/docset_links).

## Literature
- [n64dev repo doc's](https://github.com/mikeryan/n64dev/tree/master/docs)
- [VR4300 datasheet](http://datasheets.chipdb.org/NEC/Vr-Series/Vr43xx/U10504EJ7V0UMJ1.pdf)
- [Forum post where we found some boot info](http://www.emutalk.net/threads/53938-N64-tech-documentation)
- [Detailed N64 memory map](http://infrid.com/rcp64/docfiles/n64maps.txt)
- [Alternate MIPS register names](http://www.cs.umd.edu/class/sum2003/cmsc311/Notes/Mips/altReg.html)

## Test ROM's
* [Turtle's enormous public domain ROM repository](https://github.com/vgturtle127/N64-PD-ROMS)
* [Pouet list](http://www.pouet.net/prodlist.php?platform[0]=Nintendo+64&page=1)
* [Zophar](http://www.zophar.net/pdroms/n64.html)
* [PDROMs](http://pdroms.de/news/nintendo64/)
* [Micro-64](http://micro-64.com/features/aafeatures.shtml)
* [PeterLemon's ROMs](https://github.com/PeterLemon/N64)

## Building and Running
Currently, the only dependency for building is Rust itself, which can be downloaded [here](https://www.rust-lang.org/downloads.html).

An N64 BIOS (PIF ROM) is required to boot the emulator. The ROM I've been testing with thus far has a SHA-1 of `9174eadc0f0ea2654c95fd941406ab46b9dc9bdd`.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Please read [Contribution notes](CONTRIBUTING.md) before submitting a PR!
