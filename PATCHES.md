# Viper's rust-url fork: patch ledger

This repository is a fork of [rust-url](https://github.com/servo/rust-url)
kept by the Viper project. Its `viper/*` branches carry a small set of
patches on top of exact rust-url release commits; `main` and every other
branch are not Viper's and follow their own history. Viper builds `url`
from one exact commit of a `viper/*` branch, through a Cargo
`[patch.crates-io]` entry; `url`'s dependencies from this workspace
(`form_urlencoded`, `idna`, `percent-encoding`) come to Viper from
crates.io (the patch `registry-deps`).

This file is the canonical ledger of those patches. It holds the verified
base, one row and one section per patch, the fork-side rebase procedure and
the rebase log. Viper's own record of the fork (how the pin moves, the gate
step that checks it, the cargo-deny source and the licence obligations) is
its ADR 0015, "Network stack, explicit routes and the fetch contract",
section "The url crate's fork".

Base: d6ea13c5f8e7e6e627f6390161b3e185bda5e5ce
Branch: viper/url-2.5.8

## Rules

- **Branches.** One long-lived branch per url release, `viper/url-<version>`,
  created at the commit the published crate names. A `viper/*` branch is
  never force-pushed, and never deleted while any Viper revision pins a
  commit on it (a repository ruleset blocks both).
- **Markers.** Every changed hunk carries `VIPER-PATCH(<id>)` in a comment
  (`//` in Rust, `#` in TOML and in `expected_failures.txt`, a string or a
  `comment` field in the JSON test data), with `<id>` matching `[a-z0-9-]+`
  and naming a row of the ledger below. The comment states the URL Standard
  step followed or the problem fixed, and why upstream differs. Every file
  that differs from the base, apart from this one, carries a marker. Viper's
  gate step `url-fork` fails when the marker ids in the pinned commit differ
  from the ledger ids, or when a changed file carries no marker.
- **One fix per step.** Each patch follows one step of the
  [URL Standard](https://url.spec.whatwg.org/), is small (target at most 100
  changed code lines) and is one commit, which also removes from
  `url/tests/expected_failures.txt` the web-platform-tests cases it fixes.
- **Tests.** Every patch has tests in the fork (listed in its row): its
  cases in `url/tests/urltestdata.json` and `url/tests/setters_tests.json`,
  run by `url/tests/wpt.rs`, and named tests in `url/tests/viper_patches.rs`.
  Its proof in Viper's gate is a named Viper test that must pass in the
  gate's run.
- **Upstream.** A patch may be offered upstream later; nothing goes to any
  servo repository (issue, pull request or comment) without the Viper
  owner's go-ahead for that specific draft. A patch is dropped when
  upstream releases an equivalent.
- **Licences.** rust-url is MIT OR Apache-2.0 (`LICENSE-MIT`,
  `LICENSE-APACHE`, copyright the rust-url developers); patched files keep
  their notices, the licence files are unchanged, and each row names the
  licence of its change. The web-platform-tests data files are BSD-3-Clause
  (upstream's own copies of them already were).
- **Commits** use the Viper owner's GitHub noreply identity.

## Base

`viper/url-2.5.8` starts at rust-url commit
`d6ea13c5f8e7e6e627f6390161b3e185bda5e5ce` (tag `v2.5.8`, "Bump to
2.5.8"). The published crate Viper builds was verified against this
repository on 2026-10-29:

| Crate | Published `.crate` SHA-256 (equals Viper's `Cargo.lock` checksum) | `.cargo_vcs_info.json` | Sources compared with the base |
| --- | --- | --- | --- |
| `url` 2.5.8 | `ff67a8a4397373c3ef660812acab3268222035010ab8680ec4215f38ba3d0eed` | `sha1` `d6ea13c5f8e7e6e627f6390161b3e185bda5e5ce`, `path_in_vcs` `url` | identical: every packaged file under `url/src` and `url/tests`; `Cargo.toml.orig` = `url/Cargo.toml`; the packaged `README.md`, `LICENSE-MIT` and `LICENSE-APACHE` = the root files (`url/`'s licence files link to them). Not packaged: `url/benches`, `url/fuzz` |

Generated at packaging time and not compared: the normalized `Cargo.toml`,
`Cargo.lock` and `.cargo_vcs_info.json`. `url/Cargo.toml` names its sibling
crates by path, so a git dependency on this workspace as published would
build `form_urlencoded`, `idna` and `percent-encoding` from this
repository, which Viper does not pin. The patch `registry-deps` makes `url`
take those three from crates.io at the versions `url` 2.5.8 was published
with, so that Viper takes only `url` from here and `Cargo.lock` changes
only url's source line.

## Ledger

The WPT column counts the web-platform-tests cases (at WPT commit
`a50a958a6e4c822d7330aa08447f345e85c67c99`) that each patch removes from
`url/tests/expected_failures.txt`; after the last patch it lists none.
Every fix is Viper's register entry `adr:url-crate-standard-lag`'s, and
its Viper proof is `viper-bindings-web::url_wpt
the_wpt_url_vectors_pass_but_for_the_listed_differences`, which runs the
same vectors on Viper's `URL` with no listed difference left.

| Id | Reason | Register entry | Tests | Licence and upstream attribution | Upstream status |
| --- | --- | --- | --- | --- | --- |
| `registry-deps` | `url` takes `form_urlencoded` 1.2.2, `idna` 1.1.0 and `percent-encoding` 2.3.2 from crates.io instead of its path siblings, so a git dependency on `url` builds no other package from this repository | none | fork: the whole `url` package's `cargo test` (it builds against the published crates); Viper: `viper-bindings-web::url_wpt` | MIT OR Apache-2.0 (Viper's own change) | Viper-only; never offered |
| `wpt-vectors` | `urltestdata.json` and `setters_tests.json` at WPT `a50a958a`; `wpt.rs` also checks `origin` where a case names one (WPT's `url-origin.any.js`) and runs the `href` setter cases, and reads `#` lines in `expected_failures.txt` as comments | none | fork: `url_wpt` (896 parser and 278 setter cases); Viper: `viper-bindings-web::url_wpt` | test data BSD-3-Clause (web-platform-tests); harness change MIT OR Apache-2.0 (Viper's own) | upstream updates its copies on its own schedule |
| `caret-path` | the path percent-encode set includes `^` (3 WPT cases) | `adr:url-crate-standard-lag` | fork: `unit.rs` `test_path_percent_encode` (upstream's), `url_wpt`; Viper: `viper-bindings-web::url_wpt` | MIT OR Apache-2.0; upstream commit `25137be1fc1d35cc8ea7d43fc0a02166b023a483` by Charles Samborski (#1141), unreleased after v2.5.8 | upstream on main; dropped when a release carries it |
| `file-drive-host` | the file host state keeps a file URL's host before a drive letter (4 WPT cases) | `adr:url-crate-standard-lag` | fork: `url_wpt`; Viper: `viper-bindings-web::url_wpt` | MIT OR Apache-2.0; upstream commit `00a6ce58d02f4e0d43c5ca0702c0bedb8b1ebf3a` by msrd0 (#1127), unreleased after v2.5.8 | upstream on main; dropped when a release carries it |
| `ascii-domain` | domain to ASCII with beStrict false lowercases an ASCII domain whatever UTS 46 ToASCII says, failing only on a forbidden domain code point (10 WPT cases) | `adr:url-crate-standard-lag` | fork: `viper_patches.rs` `ascii_domain_is_lowercased_whatever_to_ascii_says`, `url_wpt`; Viper: `viper-bindings-web::url_wpt` | MIT OR Apache-2.0 (Viper's own change, written from the URL Standard) | never offered without the owner |
| `opaque-path-space` | the opaque path state encodes a space before `?` or `#` as `%20`, and the search and hash setters no longer strip an opaque path's trailing spaces (13 WPT cases) | `adr:url-crate-standard-lag` | fork: `viper_patches.rs` `opaque_path_space_before_query_or_fragment_is_percent_encoded`, `unit.rs` `test_strip_trailing_spaces_from_opaque_path` (new results), `url_wpt`; Viper: `viper-bindings-web::url_wpt` | MIT OR Apache-2.0 (Viper's own change, written from the URL Standard) | never offered without the owner |
| `blob-origin` | a blob URL's origin is its path URL's only for http, https and file, else opaque (4 WPT cases) | `adr:url-crate-standard-lag` | fork: `viper_patches.rs` `blob_origin_is_its_path_origin_only_for_http_https_and_file`, `url_wpt`; Viper: `viper-bindings-web::url_wpt` | MIT OR Apache-2.0 (Viper's own change, written from the URL Standard) | never offered without the owner |
| `file-empty-segments` | the file host state goes to the path start state for every host, and the path state keeps a file URL's leading empty segments (25 WPT cases) | `adr:url-crate-standard-lag` | fork: `viper_patches.rs` `file_url_keeps_leading_empty_segments`, `url_wpt`; Viper: `viper-bindings-web::url_wpt` | MIT OR Apache-2.0 (Viper's own change, written from the URL Standard) | never offered without the owner |
| `file-base-host` | the file and file slash states take base's host before a drive letter, and the path state keeps the host (13 WPT cases) | `adr:url-crate-standard-lag` | fork: `viper_patches.rs` `file_url_keeps_base_host_before_a_drive_letter`, `unit.rs` `test_syntax_violation_callback_types`, `url_wpt`; Viper: `viper-bindings-web::url_wpt` | MIT OR Apache-2.0 (Viper's own change, written from the URL Standard) | never offered without the owner |
| `relative-slash` | in the relative state `\` is a slash only for a special URL, and a special URL skips every slash before its authority (10 WPT cases) | `adr:url-crate-standard-lag` | fork: `viper_patches.rs` `relative_slashes_follow_whether_url_is_special`, `url_wpt`; Viper: `viper-bindings-web::url_wpt` | MIT OR Apache-2.0 (Viper's own change, written from the URL Standard) | never offered without the owner |
| `file-only-drive-letter` | shortening a path keeps a drive letter only for a file URL (1 WPT case) | `adr:url-crate-standard-lag` | fork: `viper_patches.rs` `drive_letter_is_kept_for_file_urls_only`, `url_wpt`; Viper: `viper-bindings-web::url_wpt` | MIT OR Apache-2.0 (Viper's own change, written from the URL Standard) | never offered without the owner |
| `port-setter-tabs` | the port setter leaves the port for a value of only tabs and newlines (1 WPT case) | `adr:url-crate-standard-lag` | fork: `viper_patches.rs` `port_setter_with_only_tabs_and_newlines_keeps_the_port`, `url_wpt`; Viper: `viper-bindings-web::url_wpt` | MIT OR Apache-2.0 (Viper's own change, written from the URL Standard) | never offered without the owner |
| `path-dot-prefix` | the pathname and host setters keep the serializer's `/.` rule for a host-null path starting with an empty segment (6 WPT cases) | `adr:url-crate-standard-lag` | fork: `viper_patches.rs` `setters_keep_the_dot_prefix_rule`, `url_wpt`; Viper: `viper-bindings-web::url_wpt` | MIT OR Apache-2.0 (Viper's own change, written from the URL Standard) | never offered without the owner |
| `path-setter-null-host` | an empty pathname appends an empty segment only when the host is null, not when it is empty (1 WPT case) | `adr:url-crate-standard-lag` | fork: `viper_patches.rs` `empty_pathname_appends_a_segment_only_for_a_null_host`, `url_wpt`; Viper: `viper-bindings-web::url_wpt` | MIT OR Apache-2.0 (Viper's own change, written from the URL Standard) | never offered without the owner |

### `registry-deps`

`url/Cargo.toml`: `form_urlencoded = "=1.2.2"`, `idna = "=1.1.0"` and
`percent-encoding = "=2.3.2"` without a path, with their features
unchanged. These are the versions the published `url` 2.5.8 names and
Viper's `Cargo.lock` holds.

### `wpt-vectors`

`url/tests/urltestdata.json` and `url/tests/setters_tests.json` are WPT's
`url/resources/` files at `a50a958a6e4c822d7330aa08447f345e85c67c99`, with
one marker each: a leading string in the `urltestdata.json` array (the
harness skips strings, as it does WPT's own comments) and the marker in the
`comment` field of `setters_tests.json`. `url/tests/wpt.rs` checks a
parser case's `origin` with `url::quirks::origin` when the case names one,
runs the `href` setter cases with `url::quirks::set_href`, and skips `#`
lines in `expected_failures.txt`. With these vectors `url` 2.5.8 fails 91
cases, which this patch lists; each later patch removes those it fixes.

### `caret-path`

[Path percent-encode set](https://url.spec.whatwg.org/#path-percent-encode-set):
the query percent-encode set and U+003F (?), U+005E (^), U+0060 (`),
U+007B ({) and U+007D (}). Upstream's commit as is, with markers added.

### `file-drive-host`

[File host state](https://url.spec.whatwg.org/#file-host-state): the host
is kept; nothing removes it when the path starts with a Windows drive
letter. Upstream's commit as is, with a marker added.

### `ascii-domain`

[Domain to ASCII](https://url.spec.whatwg.org/#concept-domain-to-ascii),
beStrict false: if the domain is an ASCII string, the result is the domain
lowercased, whatever UTS 46 ToASCII returns (an `xn--` label that is not
valid Punycode, for one); otherwise ToASCII. The result then fails when it
is empty or contains a [forbidden domain code
point](https://url.spec.whatwg.org/#forbidden-domain-code-point).
`host.rs` (`Host::parse_cow`) checks the percent-decoded ASCII domain for
forbidden domain code points and lowercases it, and calls
`idna::domain_to_ascii_from_cow` only for a non-ASCII domain.

### `opaque-path-space`

[Opaque path state](https://url.spec.whatwg.org/#cannot-be-a-base-url-path-state):
a U+0020 SPACE followed by `?` or `#` appends `%20`. The
[search](https://url.spec.whatwg.org/#dom-url-search) and
[hash](https://url.spec.whatwg.org/#dom-url-hash) setters no longer
"potentially strip trailing spaces from an opaque path"; `lib.rs` drops
`strip_trailing_spaces_from_opaque_path` and its calls in `set_query` and
`set_fragment`.

### `blob-origin`

[Origin](https://url.spec.whatwg.org/#concept-url-origin), "blob": parse
the path; if that succeeds and the URL's scheme is http, https or file,
return its origin, else a new opaque origin. `origin.rs` matches the
scheme.

### `file-empty-segments`

[File host state](https://url.spec.whatwg.org/#file-host-state): for an
empty host as for any other, go to the [path start
state](https://url.spec.whatwg.org/#path-start-state), which consumes one
`/` (or `\`, a validation error) before the path state; and the [path
state](https://url.spec.whatwg.org/#path-state) no longer removes a file
URL's leading empty segments. `parser.rs` (`parse_file`) consumes the slash
when the input after `//` is not a drive letter, and `parse_path` no longer
strips leading empty segments.

### `file-base-host`

[File state](https://url.spec.whatwg.org/#file-state) and [file slash
state](https://url.spec.whatwg.org/#file-slash-state): url's host is set to
base's host whether or not the rest starts with a Windows drive letter; the
drive letter only decides whether base's path, or its drive letter, is
kept. The [path state](https://url.spec.whatwg.org/#path-state) no longer
clears the host before a drive letter (the `FileWithHostAndWindowsDrive`
violation is still reported).

### `relative-slash`

[Relative state](https://url.spec.whatwg.org/#relative-state): `\` is a
slash only when url is special. [Relative slash
state](https://url.spec.whatwg.org/#relative-slash-state): a special URL
goes to the [special authority ignore slashes
state](https://url.spec.whatwg.org/#special-authority-ignore-slashes-state),
which skips every further `/` and `\`; a non-special one goes to the
authority state after exactly `//`. `parser.rs` (`parse_relative`).

### `file-only-drive-letter`

[Shorten a URL's path](https://url.spec.whatwg.org/#shorten-a-urls-path):
the path is kept when url's scheme is "file", its path has one segment and
that segment is a normalized Windows drive letter. `parser.rs`
(`last_slash_can_be_removed`) takes the scheme type.

### `port-setter-tabs`

[Port setter](https://url.spec.whatwg.org/#dom-url-port): the empty string
sets the port to null; any other value is parsed with the port state as
state override. A value of only tabs and newlines reaches that state as
the end of input with an empty buffer, which returns with the port
unchanged. `quirks.rs` (`set_port`).

### `path-dot-prefix`

[URL serializer](https://url.spec.whatwg.org/#concept-url-serializer): if
url's host is null, url does not have an opaque path, its path has more
than one segment and the first is the empty string, `/.` comes before the
path. `lib.rs` (`set_path`) inserts or removes the `/.` after the path
changes, and `set_host_internal` drops it when a host is set.

### `path-setter-null-host`

[Path start state](https://url.spec.whatwg.org/#path-start-state): with a
state override and an empty input, append the empty string to url's path
only if url's host is null. `quirks.rs` (`set_pathname`) tests for no
authority (a null host) instead of no host, which an empty host also is.

## Rebase procedure (fork side)

Viper's ADR 0015 holds the whole procedure; these are its fork-side steps.

1. **Trigger.** A new url release that Viper adopts; a soundness fix may
   trigger one earlier.
2. **Verify the base**: the published `.crate` SHA-256 against the
   crates.io index, its `.cargo_vcs_info.json` naming its commit, and its
   unpacked sources compared with the new base, apart from the files
   generated at packaging time. Record the result in the "Base" section,
   and decide again which packages `registry-deps` takes from crates.io.
3. **Create the branch** `viper/url-<version>` at that commit and
   cherry-pick the patch commits in ledger order, keeping the markers. A
   patch that upstream made unnecessary is retired: its row moves to a
   "Retired" table with the upstream commit, and its Viper proof must
   still pass without it. A patch that no longer applies is reworked as a
   new commit with the same id.
4. **Test.** `cargo test -p url` in the fork and each patch's fork tests;
   `expected_failures.txt` lists no case. Update this file: `Base:`,
   `Branch:`, the base, the ledger and the rebase log, with conflicts and
   effort in hours.
5. **Review.** An independent review of
   `git range-diff <old base>..<old pin> <new base>..<new pin>`.
6. **Push the new branch.** Never force-push; never delete a branch that
   holds a commit any Viper revision pins.

## Rebase log

| Date | From → to | Patches carried / retired | Conflicts | Effort (hours) | Reviewer | Viper PR |
| --- | --- | --- | --- | --- | --- | --- |

## Pin log

Each Viper revision that moves its pin to this branch.

| Viper task | Pinned commit | Change |
| --- | --- | --- |
| S3-23c | the last commit of the range that adds this row | `registry-deps`, `wpt-vectors`, `caret-path`, `file-drive-host`, `ascii-domain`, `opaque-path-space`, `blob-origin`, `file-empty-segments`, `file-base-host`, `relative-slash`, `file-only-drive-letter`, `port-setter-tabs`, `path-dot-prefix`, `path-setter-null-host` |
