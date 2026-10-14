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

The `esittely-cve-2026-99999-interim.json` CSAF document contains an initial description of the vulnerability,
but has status `interim` to indicate that the CSAF document is still being worked on.
The final version of the vulnerability is `esittely-cve-2026-99999-final.json`, which adds the information that `esittely` version `v0.2.0`
fixes the vulnerability. A possible fix and version increase for `esittely` is available in the `fixed` branch of this repository.

**Note:** Since `trustify` as of v0.6.3 only reports vulnerabilities for CVEs, `CVE-2026-99999` was used for the vulnerability in `esittely` as a placeholder.
See related issue [#2723](https://github.com/guacsec/trustify/issues/2723) on GitHub.

**Vulnerabilities:**

- `CVE-2026-25727` for the `time` crate

  This is an actual vulnerability in the `time` crate and may cause a denial of service due to stack exhaustion.
  The official advisory is available on [RUSTSEC](https://rustsec.org) as [RUSTSEC-2026-0009](https://rustsec.org/advisories/RUSTSEC-2026-0009.html).
  The issue was in the parsing implementation of RFC 2822 formatted user input, which could cause unbounded recursion on malicious input.
  More details are available in the official `time` changelog for [v0.3.47](https://github.com/time-rs/time/blob/main/CHANGELOG.md#0347-2026-02-05),
  which is the first version to mitigate the vulnerability.
  The demo uses the affected function `time::Date::parse()` in function `esittely::guess_date()`,
  when converting the user input into a valid date, but it uses a custom date format, making `esittely` unaffected.
  Nevertheless, the `time` crate should be updated to a later version.

  **Solution for this Workshop:** Update the `time` version in `Cargo.toml` to a version `>=0.3.47` (recommended `0.3.55`).
  
  **Note:** The `=` in front of the version for the `time` crate in `Cargo.toml` is used to ensure we initially use a vulnerable version.
  It can be removed if a newer version is set.

- `cve-2026-99999` for `esittely`

  This is an intentionally constructed vulnerability in `esittely` that can be seen as a potential memory exhaustion due to unbounded user input.
  The initial implementation does not restrict the user input for a new guess and only looks for a newline character.
  Therefore, malicious input could theoretically lead to memory exhaustion.

  The fix for this vulnerability is to change the used `guess_date_v1()` function in `main.rs` to `guess_date_v2()`.
  This will internally use the `esittely::retrieval::retrieve_guess_bounded()` function to restrict user input to the length for a valid date input.
  To match this fix with the final CSAF document `esittely-cve-2026-99999-final.json`, update the version of `esittely` to `0.1.1` in `Cargo.toml`.

## Observing Fixed State

Once both vulnerabilities are resolved, update the SBOM by running `cargo cyclondedx --format json`,
and upload the new SBOM to `trustify`.
The new `esittely` version `0.1.1` now contains no more known vulnerabilities.
