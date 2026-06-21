# textstat

[![Crates.io](https://img.shields.io/crates/v/textstat.svg)](https://crates.io/crates/textstat)
[![Documentation](https://docs.rs/textstat/badge.svg)](https://docs.rs/textstat)
[![CI](https://github.com/trananhtung/textstat/actions/workflows/ci.yml/badge.svg)](https://github.com/trananhtung/textstat/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/textstat.svg)](#license)
[![no_std](https://img.shields.io/badge/no__std-yes-brightgreen.svg)](#no_std)

**Readability metrics for English text.** Flesch Reading Ease, Flesch-Kincaid
Grade, Gunning Fog, SMOG, Automated Readability Index, Coleman-Liau — plus the
word / sentence / syllable counts they're built on. Inspired by Python's
[`textstat`](https://pypi.org/project/textstat/). Zero dependencies, `#![no_std]`.

```rust
let text = "The cat sat on the mat. The dog ran fast.";

assert_eq!(textstat::lexicon_count(text), 10);   // words
assert_eq!(textstat::sentence_count(text), 2);
assert_eq!(textstat::syllable_count(text), 10);

let ease = textstat::flesch_reading_ease(text);  // ~117 (very easy)
let grade = textstat::flesch_kincaid_grade(text); // ~ -1.8 (well below grade 1)
```

## Why textstat?

Rust's `readability` crates are *article extractors* (arc90/Mozilla Readability) —
they pull the main content out of a web page. **None of them compute readability
*scores*.** `textstat` fills that gap: drop-in functions for the standard formulas,
useful for content tooling, SEO, writing assistants, education, and accessibility
(WCAG) checks.

## Install

```toml
[dependencies]
textstat = "0.1"
```

## Metrics

| Function | Returns |
| --- | --- |
| `flesch_reading_ease` | 0–100+ score (higher = easier) |
| `flesch_kincaid_grade` | U.S. school grade level |
| `gunning_fog` | U.S. school grade level |
| `smog_index` | U.S. school grade level |
| `automated_readability_index` | U.S. school grade level |
| `coleman_liau_index` | U.S. school grade level |
| `reading_time(text, wpm)` | estimated seconds to read |

### Counts

`lexicon_count` (words), `sentence_count`, `syllable_count`, `syllables(word)`,
`polysyllabic_count`, `char_count` (non-whitespace), `letter_count`.

## Accuracy

Syllables are estimated with a fast English heuristic (vowel groups + a silent-`e`
rule, including accented Latin vowels), not a pronunciation dictionary, so scores
are *close to* — not bit-identical with — dictionary-based tools. The metric
**formulas** are the standard published ones.

- `sentence_count` skips decimal points (`3.14`) and initialism dots (`U.S.A.`);
  trailing abbreviation dots (`Dr.`) may still add one.
- Best results are on English text. Non-Latin scripts (no Latin vowels) fall back
  to one syllable per word, so scores stay bounded rather than correct.
- Empty input yields `0` everywhere — no panics, no division by zero.

## no_std

`textstat` is `#![no_std]` (needs only `alloc`) with a dependency-free Newton's
-method `sqrt`, so it builds for bare-metal targets such as `thumbv7em-none-eabi`.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at
your option.
