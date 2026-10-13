# csaf-workshop

Workshop around Rust tooling for CSAF

This repository contains the `esittely` Rust crate, which is a game you can run in your terminal.
See its README on how to run the game.

Unfortunately, a few vulnerabilities have slipped into the crate.
This can be observed using [trustify](https://github.com/guacsec/trustify) and the SBOM and CSAF documents that are part of this repository.

## Setting up Trustify

Download the latest `trustd-pm` [release](https://github.com/guacsec/trustify/releases), or clone the repository and build it from source following the [official instructions](https://github.com/guacsec/trustify#option-2-build-from-source).

Using the released binary, start `trustify` locally by executing:

```sh
AUTH_DISABLED=true ./trustd-pm
```

This will start a webserver that is accessible at http://localhost:8080.

## Creating a SBOM

To create a SBOM for Rust crates, install [`cargo-cyclonedx](https://crates.io/crates/cargo-cyclonedx) via:

```sh
cargo install cargo-cyclonedx
```

Once installed, navigate into the `esittely` directory and run:

```sh
cargo cyclonedx --format=json
```

By default, this will generate the file `esittely.cdx.json` at the crate directory level.

**Note:** `cargo-cyclonedx` is only one of many tools that is able to create SBOMs for Rust crates.
Another tool that is able to handle multiple programming languages for example is [syft](https://github.com/anchore/syft).

## Observing Vulnerabe State

Using the `trustify` web interface, the CycloneDX SBOM file for `esittely` can be uploaded to `trustify`.
Once the SBOM has been uploaded, the CSAF documents from the `csaf` folder can be uploaded to `trustify` under the `Advisories` tab.

Initially, upload the documents `time-cve-2026-25727.json` and `esittely-cve-2026-99999-interim.json`.
Uploading those two advisories will show that the `esittely` crate is affected by two vulnerabilities.


