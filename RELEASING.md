# Publishing Rusty GPUI

The repository contains five publishable libraries and an unpublished gallery. All five libraries were published as 0.4.0 on 2026-10-30. The steps below document the release process for future versions.

## Before the first GitHub push

The project is named Rusty GPUI, with `rusty-gpui` as both the public crate and GitHub repository name. The selected repository URL is https://github.com/Ahmdrza/rusty-gpui; it is set in all five package manifests. Verify that repository is accessible and contains the release source before publication. All five proposed names had no existing crates.io entries when checked during preparation; this does not reserve them or guarantee future availability. A crate name belongs to its crates.io owners, independently of the GitHub repository name.

Set the repository URL once chosen:

```sh
python3 scripts/set-repository.py https://github.com/Ahmdrza/rusty-gpui
```

This writes repository metadata into the workspace and all five publishable package manifests. The selected URL is already set in this candidate. Review the generated diff, package names, descriptions, README files, licenses and upstream attribution before committing.

Then create an empty GitHub repository and push this source folder using your normal Git workflow. For a new local checkout, the usual sequence is:

```sh
git init -b main
git add .
git commit -m "Prepare Rusty GPUI design system libraries"
git remote add origin https://github.com/Ahmdrza/rusty-gpui.git
git push -u origin main
```

Use your actual URL and review the staged files before committing. If the folder already has Git history or a remote, use that existing setup. Source packaging excludes build caches, local runtime files and credentials. Keep Cargo.lock committed; do not commit target, .env files, registry credentials or generated .crate files.

## Local development versus registry dependencies

The workspace declares a matching version and path for each first-party dependency. Cargo uses the path during development and strips it when generating the published manifest. The facade depends on the distinct rusty-gpui-components fork instead of patching upstream gpui-component. Applications need only rusty-gpui; no consumer patch tables are required.

There are no patch tables in the workspace. The clean consumer check exposed xattr 0.2.3 through gpui_http_client's tar dependency. Rusty GPUI's HTTP compatibility package disables zed-async-tar's unused default xattr feature; its extraction helper never enables archive xattr restoration. The GPUI runtime distribution routes to that HTTP package and explicitly types grid sizing literals for current Rust compatibility. This avoids pinning libc backward, changing the app's controls, or requiring a downstream patch. Provenance and the narrow change are documented in both compatibility crates.

The gallery has publish = false and is not a dependency of any library. The native fork retains Apache-2.0 and UPSTREAM.md; facade/theme code is MIT. Fonts and icons carry their own notices inside rusty-gpui.

## Preflight

Use current stable Rust, Python 3.11+ and native GPUI prerequisites. Run the focused checks from CONTRIBUTING.md and review the gallery on macOS for relevant UI changes. First cache the locked dependency archives, then run:

```sh
cargo fetch --locked
python3 scripts/check-release.py --build-consumer
```

The script:

1. Creates a temporary local registry under target/release-check using already cached Cargo metadata/archives.
2. Packages rusty-gpui-http, rusty-gpui-runtime, rusty-gpui-theme, rusty-gpui-components and rusty-gpui in publication order.
3. Examines their normalized manifests and package contents for path/git dependencies, README files, licenses and unintended gallery/build files.
4. Registers those actual .crate files in the temporary registry.
5. Resolves and checks an independent consumer whose only dependency is a versioned rusty-gpui. Its manifest has no patch table and no path dependencies.
6. Compiles the public README quickstart as a doctest, the full starter and every copyable example extracted from the packaged crate; `--build-consumer` also builds a runnable starter at `target/debug/rusty-gpui-release-consumer` and example executables under `target/debug/examples/`.

Run `target/debug/rusty-gpui-release-consumer` from a graphical native session to inspect the packaged starter.

A successful check writes archive sizes, file counts, SHA-256 checksums and compile results to `target/release-check/report.json`. The old report is removed when a new check starts, so a failed run cannot leave stale success evidence. Copy the final report into `docs/release-validation-0.4.0.json` after recording any separately performed native review; regenerate it whenever packaged files change.

Source replacement exists only in temporary tooling so unpublished names can be checked locally; it is not a configuration users install. The script also checks that all five Rusty GPUI libraries resolve as registry packages and upstream gpui-component, registry gpui/gpui_http_client and obsolete xattr are absent. It does not upload anything. --skip-consumer performs packaging and resolution without compilation when only package-content inspection is needed.

The script converts Cargo's current sparse-index cache into Cargo's local-registry format, exposing only versions whose archives are actually cached. Newer metadata fetched by unrelated Cargo operations cannot make this offline registry select an unavailable download. It needs cached metadata and archives. After Cargo upgrades, confirm the tooling still reads that cache correctly. A fresh temporary consumer and its lockfile are generated for each check. Only this tooling's prior staging copies of Rusty GPUI crates are removed from Cargo's local cache when refreshing an unpublished version.

The GitHub **Package release candidate** workflow runs on main pushes, pull requests and manual dispatch for macOS and Linux, and uploads candidate archives as CI artifacts. It uses the actual GitHub repository URL for metadata. It does not publish crates or create releases and needs no crates.io token.

## Publish order

Once names, repository metadata and 0.4.0 release contents are final, publish dependencies before the facade. Start with the HTTP and runtime compatibility crates, then theme, components and facade. Authenticate with cargo login using your own crates.io account; store credentials outside the repository. For automated publication later, configure crates.io trusted publishing for the actual repository/workflow rather than committing a token.

Run each pair separately, checking the output and waiting for the published version to become available in the registry before proceeding:

```sh
cargo publish -p rusty-gpui-http --dry-run
cargo publish -p rusty-gpui-http
```

```sh
cargo publish -p rusty-gpui-runtime --dry-run
cargo publish -p rusty-gpui-runtime
```

```sh
cargo publish -p rusty-gpui-theme --dry-run
cargo publish -p rusty-gpui-theme
```

```sh
cargo publish -p rusty-gpui-components --dry-run
cargo publish -p rusty-gpui-components
```

```sh
cargo publish -p rusty-gpui --dry-run
cargo publish -p rusty-gpui
```

Before the lower-level crates are published, a normal crates.io dry run of the facade may fail because its registry dependencies do not exist yet. The local-registry check solves pre-publication validation; it does not claim the remote registry has those packages.

Do not use --no-verify to bypass checks during real publication. The staging script uses it only while producing candidate archives, then separately compiles the patch-free consumer. crates.io publication is externally visible; package versions are immutable. The 0.4.0 release used normal verified publication for all five crates.

After all five are published, confirm a fresh app can add rusty-gpui = "0.4" from crates.io without source replacement or patches. Check documentation builds, change the changelog entry from Unreleased to its release date, and create the corresponding v0.4.0 Git tag/GitHub release.

## Version and compatibility policy

All five Rusty GPUI libraries currently share the workspace release version. Bump workspace.package.version and matching version requirements in workspace.dependencies together; regenerate Cargo.lock and rerun package checks. The maintained fork records upstream GPUI Component 0.5.1 separately; Rusty GPUI's version is not the upstream version.

Before 1.0, incompatible public API changes require a minor version increment; compatible fixes use patch versions. Do not claim an MSRV without checking it. Current stable Rust is the supported development baseline; the cloud validation used Rust 1.99.1. Native macOS/Linux are the release targets; Windows is not release-tested.

The compatibility runtime retains GPUI 0.2.2 source/API because its public types are part of Rusty GPUI's native API. An app importing GPUI directly must use a dependency named gpui with package = "rusty-gpui-runtime", version = "=0.4.0", default-features = false; do not mix registry gpui types with the Rusty GPUI runtime. Avoid mixing this component fork and upstream GPUI Component Root/theme globals. The facade reexports GPUI, theme and components so consumers can use one consistent set of types.

## Rename from Form

The pre-publication Form workspace is now Rusty GPUI. Crate names, Rust imports, source folders, gallery labels, launchers and CI commands use rusty-gpui/rusty_gpui and the matching support-crate prefix. RustyField and RustyTheme are the canonical public types; FormField and FormTheme remain compatibility aliases. The gallery reads its old Form settings location if the new file does not exist. RUSTY_GPUI_CONFIG_DIR is preferred, with FORM_CONFIG_DIR accepted as a legacy override.
