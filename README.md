# Roblox Auto Patcher
Automatically and easily patch 2019 and below Roblox Player, Studio and RCC.

This program uses the safest way of patching, by simply replacing the signing certificates and replacing all roblox.com urls with one of your choice. This avoids disabling trust check, and signing outright.

## Warnings & Notices
> Currently this program is uncapable of patching >2017 clients (rbxsig2) at this time, future updates should hopefully include this!

> You will have to put the signing keys in certain coreguis/corescripts/join scripts, and files. If you didn't already know this, you probably shouldn't be using this program.

> This program should work perfectly fine for 2017 through 2015 clients. Other versions may require additional patches, some examples are >2018 clients, and >2014 clients.

## Building

Building for debug targets:
```bash
cargo run build
```

Building for release targets:
```bash
cargo run build -r
```

## Supported Versions
- [x] 2009 (requires additional patches)
- [x] 2010 (requires additional patches)
- [x] 2011 (requires additional patches)
- [x] 2012 (requires additional patches)
- [x] 2013 (may require additional patches)
- [x] 2014 (may require additional patches)
- [x] 2015
- [x] 2016
- [x] 2017
- [ ] 2018 (requires rbxsig2)
- [ ] 2019 (requires rbxsig2)

## Todo
- [x] Replace roblox.com urls with user specified
- [x] Replace --rbxsig certificates
- [ ] Use in-house certificate generator
- [ ] Support for --rbxsig2 (roblox uses different signatures for different versions)
- [ ] Be able to patch bootstrapper
- [ ] Support for longer than 10 character domains

## Contribution & Credits
Want to contribute? Simply create a pull request, or if you are experiencing problems please open an issue.

The rbxsig KeyGenerator was originally from [this archive](https://www.mediafire.com/file/msbfxp1ades6v9j/tools.zip/file), although this project uses a [version I decompiled](https://github.com/worships/Roblox-KeyGenerator-Decompiled). I am unaware of who created it, but if you know please open up an issue or pull request, so they can be credited properly.