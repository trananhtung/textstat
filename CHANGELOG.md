# Changelog

All notable changes to this project are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-06-21

### Added

- Initial release.
- Readability metrics: `flesch_reading_ease`, `flesch_kincaid_grade`,
  `gunning_fog`, `smog_index`, `automated_readability_index`, `coleman_liau_index`.
- `reading_time(text, wpm)` estimate.
- Counts: `lexicon_count`, `sentence_count`, `syllable_count`, `syllables`,
  `polysyllabic_count`, `char_count`, `letter_count`.
- Dependency-free heuristic syllable counting and a `no_std` Newton's-method `sqrt`.
- `#![no_std]` support (requires `alloc`); zero dependencies.

[0.1.0]: https://github.com/trananhtung/textstat/releases/tag/v0.1.0
