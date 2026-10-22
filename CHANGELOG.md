# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [4.0.0](https://github.com/kuest/rs-clob-client/compare/v3.0.0...v4.0.0) - 2026-10-22

### Fixed

- *(clob)* [**breaking**] align builder trades with CLOB response

## [3.0.0](https://github.com/kuest/rs-clob-client/compare/v2.0.18...v3.0.0) - 2026-10-22

### Changed

- Read Kuest trade account owners as strings while keeping WebSocket subscription owners as API-key UUIDs. Open-order owners are optional ULIDs, and trade outcomes and transaction hashes may be absent. These response type changes require consumers to update their Rust code.
- Accept the CLOB's RFC3339 trade times and `type` perspective field, while retaining epoch times and `trader_side` as input aliases.
- Report discarded WebSocket messages without logging payloads or credentials.

## [2.0.18](https://github.com/kuest/rs-clob-client/compare/v2.0.17...v2.0.18) - 2026-09-30

### Fixed

- update repository links for the GitHub organization migration

## [2.0.8](https://github.com/kuest/rs-clob-client/compare/v2.0.7...v2.0.8) - 2026-06-14

### Fixed

- scope market discovery to fork site ([#52](https://github.com/kuest/rs-clob-client/pull/52))

### Other

- *(deps)* bump chrono from 0.4.44 to 0.4.45 ([#50](https://github.com/kuest/rs-clob-client/pull/50))
- *(deps)* bump aws-sdk-kms from 1.107.0 to 1.109.0 ([#51](https://github.com/kuest/rs-clob-client/pull/51))
- *(deps)* bump uuid from 1.23.1 to 1.23.2 ([#49](https://github.com/kuest/rs-clob-client/pull/49))
- *(deps)* bump actions/checkout from 6.0.2 to 6.0.3 ([#48](https://github.com/kuest/rs-clob-client/pull/48))
- *(deps)* bump reqwest from 0.13.3 to 0.13.4 ([#47](https://github.com/kuest/rs-clob-client/pull/47))
- *(deps)* bump zizmorcore/zizmor-action from 0.5.3 to 0.5.6 ([#46](https://github.com/kuest/rs-clob-client/pull/46))
- *(deps)* bump strum_macros from 0.27.2 to 0.28.0 ([#43](https://github.com/kuest/rs-clob-client/pull/43))
