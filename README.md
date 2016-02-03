# rustendo64 [![Build Status](https://travis-ci.org/yupferris/rustendo64.svg?branch=master)](https://travis-ci.org/yupferris/rustendo64) [![Build Status](https://ci.appveyor.com/api/projects/status/owjloq84v91147nd/branch/master?svg=true)](https://ci.appveyor.com/project/yupferris/rustendo64/branch/master)

![rustendo64](Rustendo-64.png)

Livecoding a Nintendo 64 emulator in Rust :D

## Follow along
The entire process is currently being streamed on [Twitch](http://www.twitch.tv/ferrisstreamsstuff), and each segment is being recorded and uploaded to [this YouTube playlist](https://www.youtube.com/playlist?list=PL-sXmdrqqYYcznDg4xwAJWQgNL2gRray2). For stream times and announcements, you can check out [my Twitter](https://twitter.com/ferristweetsnow).

## Helpful tools
- [Hex Fiend](http://ridiculousfish.com/hexfiend/)
- [Online disassembler](https://www.onlinedisassembler.com/odaweb/)
- [Dash](https://kapeli.com/dash) (OS X / iOS) for documentation. Compatible alternatives for other platforms can be found linked from [Dash Docset Links](https://kapeli.com/docset_links).

## Literature
- [VR4300 datasheet](http://datasheets.chipdb.org/NEC/Vr-Series/Vr43xx/U10504EJ7V0UMJ1.pdf)
- [Forum post where we found some boot info](http://www.emutalk.net/threads/53938-N64-tech-documentation)
- [Detailed N64 memory map](http://infrid.com/rcp64/docfiles/n64maps.txt)
- [Alternate MIPS register names](http://www.cs.umd.edu/class/sum2003/cmsc311/Notes/Mips/altReg.html)

## Test ROM's
* [Pouet list](http://www.pouet.net/prodlist.php?platform[0]=Nintendo+64&page=1)
* [Zophar](http://www.zophar.net/pdroms/n64.html)
* [PDROMs](http://pdroms.de/news/nintendo64/)
* [Micro-64](http://micro-64.com/features/aafeatures.shtml)
* [PeterLemon's ROMs](https://github.com/PeterLemon/N64)

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution _(please read before submitting a PR!)_

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

I do accept (and appreciate!) pull requests, but please, try to keep them to _small, meaningful, isolated changes only_ that I can go over completely on the stream. Significant outside contributions, as cool as they may be, somewhat defeat the most important part of this project - **documenting everything through livecoding**. I'm happy to accept small cosmetic changes/bugfixes, but please, consider what the larger audience as a whole might be missing out on when they don't get to see the thought process and resources that went into making the contribution (which is unfortunately what happens whenever I accept a PR).

If you'd like to see a particular library or coding style used somewhere, opening an issue is much preferred over a PR, so we can discuss it beforehand and implement it live. This also keeps people from stepping on each others' toes and implementing the same things (yes, this has happened already).

Issues, especially pertaining to accuracy/bugfixes, are always more than welcome!
