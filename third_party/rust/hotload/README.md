# Hotload

A small rust library to wrap a hotloadable dylib.

It comes with "Hotloader", a struct that auto hotloads a dylib, and a basic method proxy that can load, unload, and run a dylib.

Users can define their own method proxies, provided that they still support loading and unloading internal state.
