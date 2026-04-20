# bastion — moved

> **This repository has been archived.** Bastion now lives as a
> sub-crate in [`devopsdefender/dd`](https://github.com/devopsdefender/dd)
> at [`crates/bastion/`](https://github.com/devopsdefender/dd/tree/main/crates/bastion).

## Why

Bastion was split out in April 2026 to evolve independently of DD.
Within days the coupling proved too tight — every bastion API change
forced a matching DD PR — so the sources were merged back into DD's
Cargo workspace. Two release tracks survive the merge: a `bastion-v*`
tag on `devopsdefender/dd` still rebuilds just the bastion binary and
deploys it onto live DD agents without CP/agent fleet churn.

See `devopsdefender/dd`'s plan notes for the full timeline.

## Install

`bastion-term v0.1.1` on crates.io is preserved as-is; future releases
go via GitHub releases on `devopsdefender/dd` only.

To pull the latest binary:

```sh
# From a DD release artifact
curl -LO https://github.com/devopsdefender/dd/releases/latest/download/bastion.x86_64
chmod +x bastion.x86_64 && ./bastion.x86_64 serve --port 7681
```

Or build from source:

```sh
git clone https://github.com/devopsdefender/dd
cd dd
cargo build --release -p bastion-term --bin bastion
./target/release/bastion serve --port 7681
```

## License

MIT.
