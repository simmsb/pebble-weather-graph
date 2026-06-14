# pebble-weather-graph

A Pebble watchface written in Rust using [Gravel](gravel/) bindings for the Pebble SDK.

## Building & running

Requires nightly Rust (see `rust-toolchain.toml`) for `build-std`.

```sh
pebble build                          # build for all targetPlatforms
pebble install --emulator emery       # install on the emery emulator
pebble install --phone <ip>           # install to a paired phone
```

## Target platforms

`targetPlatforms` in `package.json` controls which watches you build for. The
modern Pebble hardware is **emery** (Pebble Time 2), **gabbro** (Pebble Round
2), and **flint** (Pebble 2 Duo); the original Pebble platforms (aplite,
basalt, chalk, diorite) are included by default for backwards compatibility.

## Project layout

```
src/               Rust watchface source (staticlib entry in lib.rs)
src/ts/            PebbleKit JS (phone-side) source
src/generated/     Message key and resource ID bindings for the Pebble link step
gravel/            Gravel Pebble SDK bindings (gravel + gravel-sys)
resources/         Images, fonts, and other bundled resources
package.json       Project metadata (UUID, platforms, resources, message keys)
wscript            Build rules — cargo staticlib linked via pbl_program
Cargo.toml         Rust crate manifest
```

By default this project is configured as a watchapp. To make it a watchface,
set `pebble.watchapp.watchface` to `true` in `package.json`.

## Documentation

Full SDK docs, tutorials, and API reference: <https://developer.repebble.com>
