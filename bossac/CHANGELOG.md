# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v2.1.0 (2023-10-28)

### New Features

 - <csr-id-1e44ef128812cce37e6ade4c1a435cd4438313ed/> FlasherInfoRs
   - Convenient Rust struct to access FlasherInfo fields natively
     * Includes unique_id() to get the string version of the unique id as
       shown by bossa
   - Rename mod bossa to lib (rust compiler gets confused with too many
     bossa module/crates names)
   - Add example on how to use FlasherInfoRs

### Bug Fixes

 - <csr-id-062479b4b21403c5c3c04ea0e87298360672222f/> BossaObserver now sends output through callback
   - Allows for fully capturing output of functions (instead of being
     printed to stdout)
 - <csr-id-5a45d19940f8fc54fdbb2017c54b52caba32e6d2/> README.md GitHub Action badges

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 16 calendar days.
 - 16 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Increment versions of bossa and bossac ([`7874fcf`](https://github.com/kiibohd/BOSSA/commit/7874fcf7e8bb5618fe039d3b7a9eb676d0409a5c))
    - BossaObserver now sends output through callback ([`062479b`](https://github.com/kiibohd/BOSSA/commit/062479b4b21403c5c3c04ea0e87298360672222f))
    - FlasherInfoRs ([`1e44ef1`](https://github.com/kiibohd/BOSSA/commit/1e44ef128812cce37e6ade4c1a435cd4438313ed))
    - README.md GitHub Action badges ([`5a45d19`](https://github.com/kiibohd/BOSSA/commit/5a45d19940f8fc54fdbb2017c54b52caba32e6d2))
</details>

## v2.0.0 (2023-10-12)

### Bug Fixes

 - <csr-id-d7ea2775198dfa0e5d75f2d7abc08b34f5e88a8a/> Missing CHANGELOG.md

### New Features

 - <csr-id-8b29f5ac3b18d59edecc904fcfae612ebab78bcc/> Initial bossa library and bossac utility
   - CXX Rust wrapper around bossa so it can be statically linked library
   into rust utilities (this is useful for Windows applications rather
   than relying on external processes).
* To get the rust wrapper to work some C++ changes were necessary to
   cleanup the api. This probably would have been easier if the wider
   C++ apis were const'd correctly but I err'd on the side of minimal
   changes.
* Keep Linux at 3.0 for Ubuntu GitHub Action

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release bossac v2.0.0 ([`7d12611`](https://github.com/kiibohd/BOSSA/commit/7d126115bba81c2e0111472cc3dedc0c0786a573))
    - Missing CHANGELOG.md ([`d7ea277`](https://github.com/kiibohd/BOSSA/commit/d7ea2775198dfa0e5d75f2d7abc08b34f5e88a8a))
    - Initial bossa library and bossac utility ([`8b29f5a`](https://github.com/kiibohd/BOSSA/commit/8b29f5ac3b18d59edecc904fcfae612ebab78bcc))
</details>

