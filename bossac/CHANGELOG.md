# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

 - 2 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Missing CHANGELOG.md ([`d7ea277`](https://github.com/kiibohd/BOSSA/commit/d7ea2775198dfa0e5d75f2d7abc08b34f5e88a8a))
    - Initial bossa library and bossac utility ([`8b29f5a`](https://github.com/kiibohd/BOSSA/commit/8b29f5ac3b18d59edecc904fcfae612ebab78bcc))
</details>

<csr-unknown>
bossac tries to be as close as possible to the C++ version of bossac(including cli args)Does not expose the complete bossa library api (only those necessaryfor bossac)An equivalent bossash should be possible, but I don’t really use it soleaving it open to future contributions.The bossa WxWidgets utility is more difficult, but might be possiblewith https://github.com/kenz-gelsoft/wxRust2.Personally, I would probably recommend https://github.com/KDAB/cxx-qt.But I don’t really use either so I’ll leave these as open to futurecontributions.The existing C++ binaries should continue to function/buildFix formbuilder warning (info)Update wxWidgets mostly to 3.2<csr-unknown/>

