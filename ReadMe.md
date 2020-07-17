![logo.png](assets/gdext-ferris.png)

# Rust bindings for GDExtension

_[Discord] | [Mastodon] | [Twitter]_

**gdext** is an early-stage library to bind the **Rust** language to **Godot 4**.

[Godot] is an open-source game engine, whose upcoming version 4.0 brings several improvements.
Its _GDExtension_ API allows integrating third-party languages and libraries.

> **Note**: if you are looking for a Rust binding for GDNative (Godot 3), checkout [`gdnative`].

> **Warning**: this library is experimental and rapidly evolving. In particular, this means:
> * Lots of bugs. A lot of the scaffolding is still being ironed out. 
>   There are known safety issues, possible undefined behavior as well as other potential problems.
> * Lots of missing features. The priority is to get basic interactions working;
>   as such, edge case APIs are deliberately neglected at this stage.
> * No stability guarantees. APIs will break frequently (for releases, we try to take SemVer seriously though).
>   Resolving the above two points has currently more weight than a stable API.

We do not recommend building a larger project in gdext yet.
However, the library can serve as a playground for experimenting.

To get an overview of currently supported features, consult [#24](https://github.com/godot-rust/gdext/issues/24).  
At this point, there is **no** support for Android, iOS or WASM. Contributions are very welcome!


## Getting started

### Toolchain

You need to have LLVM installed to use `bindgen`, see [the book](https://godot-rust.github.io/book/getting-started/setup.html#llvm) for instructions.

To find a version of Godot 4, the library expects either an executable of name `go