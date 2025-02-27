# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added 

_(none)_

### Changed

- Migrated `yt-dlp` from `--referer "<URL>"` to new style `--add-header "Referer:<URL>"`

### Fixed

_(none)_

### Removed

_(none)_

## [0.5.2] - 2023-05-03

### Changed

- Updated transitive dependencies.


## [0.5.1] - 2023-02-10

### Fixed

- Progress detail extraction failed in rare cases.

## [0.5.0] - 2023-02-10

### Added

- Tracing logs are now written to `vimeo-showcase.log` and can be `tail`ed for live viewing.
- Custom patched versions of `yt-dlp` or `youtube-dl` can be used via the new `--bin` option.
- Add `CHANGELOG.md`, following [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

### Fixed

- Downloader errors are now reported with error log level.

## [0.4.0] - 2023-01-21

### Fixed

- Download progress is now correctly parsed again, after `yt-dlp` changed to a tabular format.

## [0.3.2] - 2023-01-21

### Fixed

- Command line help is now wrapped by `clap`.

### Changed

- Change command line arguments definitions to use `clap` 4 attribute macros.
- Follow `clippy` auto-deref lints.

## [0.3.1] - 2023-01-21

### Added

- Extract `mp3` and `opus` audio with `ffmeg`.

### Fixed

- Regex failed to match valid embeds.

### Changed

- Update dependencies.
- Upgrade `clap` from `3.x` to `4.1.1`.

## [0.3.0] - 2022-09-12

### Added

- Implement terminal user interface.
- Add `README.md`

### Changed

- Spawn tasks to make use of multi-threaded runtime.
- Implement graceful shutdown.

## [0.2.0] - 2022-09-07

### Added

- Implement progress tracking in shared state, preparing for terminal UI.
- Use lazy_static! for compile-once regular expressions.

## [0.1.0] - 2022-09-05

### Added

- Initial implementation.

[unreleased]: https://github.com/LeoniePhiline/showcase-dl/compare/0.5.2...HEAD
[0.5.2]: https://github.com/LeoniePhiline/showcase-dl/compare/0.5.1...0.5.2
[0.5.1]: https://github.com/LeoniePhiline/showcase-dl/compare/0.5.0...0.5.1
[0.5.0]: https://github.com/LeoniePhiline/showcase-dl/compare/0.4.0...0.5.0
[0.4.0]: https://github.com/LeoniePhiline/showcase-dl/compare/0.3.2...0.4.0
[0.3.2]: https://github.com/LeoniePhiline/showcase-dl/compare/0.3.1...0.3.2
[0.3.1]: https://github.com/LeoniePhiline/showcase-dl/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/LeoniePhiline/showcase-dl/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/LeoniePhiline/showcase-dl/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/LeoniePhiline/showcase-dl/releases/tag/0.1.0

