# Developing Zygisk Modules

This repository hosts a template zygisk module for developers to start developing Zygisk modules. Before developing Zygisk modules, you should first check out the official documentation for [Magisk Modules](https://topjohnwu.github.io/Magisk/guides.html). Do not fork this repository for your new module; either manually clone this repository.

## API
- The canonical URL of the latest public Zygisk API is [module/jni/zygisk.hpp](https://github.com/topjohnwu/zygisk-module-sample/blob/master/module/jni/zygisk.hpp).
- The header file is self documented; directly refer to the header source code for all Zygisk API details.
- Magisk is committed to maintain backwards compatibility forever. That is, whenever there is an API update for Zygisk in a newer Magisk version, Magisk can always load Zygisk modules built for an older Zygisk API.
- If you do not need the new features introduced in newer API versions, it's perfectly fine to stay on the older API version to maintain maximum compatibility.

|                                        Zygisk API                                         | Minimal Magisk |                                      Diff                                      |
| :---------------------------------------------------------------------------------------: | :------------: | :----------------------------------------------------------------------------: |
| [v4](https://github.com/topjohnwu/zygisk-module-sample/blob/master/module/jni/zygisk.hpp) |     26000      | [v3..v4](https://github.com/topjohnwu/zygisk-module-sample/compare/v3..master) |
|   [v3](https://github.com/topjohnwu/zygisk-module-sample/blob/v3/module/jni/zygisk.hpp)   |     24300      |   [v2..v3](https://github.com/topjohnwu/zygisk-module-sample/compare/v2..v3)   |
|   [v2](https://github.com/topjohnwu/zygisk-module-sample/blob/v2/module/jni/zygisk.hpp)   |     24000      |                                      N/A                                       |


## Build Dependence
### 1. [Cargo NDK](https://github.com/bbqsrc/cargo-ndk)
### 2. [Python3](https://www.python.org/) 
Test on Python 3.11.4



## Building

### 1. Install Cargo NDK

```cargo install cargo-ndk```

For more Details, view https://github.com/bbqsrc/cargo-ndk

### 2. Install Android target for Rust

```shell
rustup target add armv7-linux-androideabi   # for arm
rustup target add i686-linux-android        # for x86
rustup target add aarch64-linux-android     # for arm64
rustup target add x86_64-linux-android      # for x86_64
```

### 3. Build Project with Python

```python build.py```

And then, artifact will put in `out`

## Problem
Only Support API of Version 2

## Thanks
### [@Kazurin-775](https://github.com/Kazurin-775)
Some codes from [Zygisk-Rust-bindings](https://github.com/Kazurin-775/Zygisk-Rust-bindings)
### [@Gstalker](https://github.com/Gstalker)
Some codes from [Zygisk-Rust-ModuleTemplate](https://github.com/Gstalker/Zygisk-Rust-ModuleTemplate)
