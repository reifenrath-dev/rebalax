# Rebalance + Relax = Rebalax

## Setup

Make sure tauri-cli and trunk-cli are installed.

```shell
cargo install tauri trunk --locked
```

Or install all dependencies of the project using the following command while in the project directory:
```shell
cargo install --path .
```

Also make sure to meet the prerequisites: https://tauri.app/start/prerequisites/#linux

## Local Testing

`cargo tauri dev`

## Building Android APKs

```shell
cargo tauri android build --apk --split-per-abi
```

## App Verification Hash

```
D7:D8:D0:F5:ED:14:24:64:3F:43:4F:EE:4B:F0:0D:61:70:5F:D7:B7:4B:7B:C9:0B:C9:7B:A6:C7:96:07:B2:27
```