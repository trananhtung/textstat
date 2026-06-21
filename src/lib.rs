//! # textstat — readability metrics for English text
//!
//! Compute the classic readability scores — Flesch Reading Ease, Flesch-Kincaid
//! Grade, Gunning Fog, SMOG, Automated Readability Index, Coleman-Liau — plus the
//! word / sentence / syllable counts they are built on. Inspired by Python's
//! [`textstat`](https://pypi.org/project/textstat/).
//!
//! ```
//! let text = "The cat sat on the mat. The dog ran fast.";
//! assert_eq!(textstat::lexicon_count(text), 10);
//! assert_eq!(textstat::sentence_count(text), 2);
//!
//! // Simple text scores very high on reading ease (0–100+, higher = easier).
//! assert!(textstat::flesch_reading_ease(text) > 90.0);
//! ```
//!
//! Pure logic, zero dependencies, `#![no_std]`.
//!
//! ## Note on accuracy
//!
//! Syllables are counted with a fast English heuristic (vowel groups with a
//! silent-`e` rule), not a pronunciation dictionary, so scores are close to —
//! but not bit-identical with — dictionary-based tools. The metric *formulas*
//! themselves are the standard published ones.

#![no_std]
#![doc(html_root_url = "https://docs.rs/textstat/0.1.0")]
#![allow(clippy::cast_precision_loss)]

extern crate alloc;

mod count;

pub use count::{
    char_count, letter_count, lexicon_count, polysyllabic_count, sentence_count, syllable_count,
    syllables,
};

/// Flesch Reading Ease: higher is easier (typically 0–100; can exceed both ends).
///
/// `206.835 − 1.015 × (words/sentences) − 84.6 × (syllables/words)`
#[must_use]
pub fn flesch_reading_ease(text: &str) -> f64 {
    let (words, sentences, syllables) = base(text);
    if words == 0.0 || sentences == 0.0 {
        return 0.0;
    }
    206.835 - 1.015 * (words / sentences) - 84.6 * (syllables / words)
}

/// Flesch-Kincaid Grade Level (U.S. school grade).
///
/// `0.39 × (words/sentences) + 11.8 × (syllables/words) − 15.59`
#[must_use]
pub fn flesch_kincaid_grade(text: &str) -> f64 {
    let (words, sentences, syllables) = base(text);
    if words == 0.0 || sentences == 0.0 {
        return 0.0;
    }
    0.39 * (words / sentences) + 11.8 * (syllables / words) - 15.59
}

/// Gunning Fog index (U.S. school grade).
///
/// `0.4 × ((words/sentences) + 100 × (complex_words/words))`, where complex
/// words have three or more syllables.
#[must_use]
pub fn gunning_fog(text: &str) -> f64 {
    let (words, sentences, _) = base(text);
    if words == 0.0 || sentences == 0.0 {
        return 0.0;
    }
    let complex = polysyllabic_count(text) as f64;
    0.4 * ((words / sentences) + 100.0 * (complex / words))
}

/// SMOG index (U.S. school grade), based on polysyllabic word density.
///
/// `1.0430 × √(polysyllables × 30/sentences) + 3.1291`
#[must_use]
pub fn smog_index(text: &str) -> f64 {
    let (_, sentences, _) = base(text);
    if sentences == 0.0 {
        return 0.0;
    }
    let poly = polysyllabic_count(text) as f64;
    1.0430 * sqrt(poly * (30.0 / sentences)) + 3.1291
}

/// Automated Readability Index (U.S. school grade).
///
/// `4.71 × (chars/words) + 0.5 × (words/sentences) − 21.43`, where `chars` is the
/// non-whitespace character count.
#[must_use]
pub fn automated_readability_index(text: &str) -> f64 {
    let (words, sentences, _) = base(text);
    if words == 0.0 || sentences == 0.0 {
        return 0.0;
    }
    let chars = char_count(text) as f64;
    4.71 * (chars / words) + 0.5 * (words / sentences) - 21.43
}

/// Coleman-Liau index (U.S. school grade).
///
/// `0.0588 × L − 0.296 × S − 15.8`, where `L` is letters per 100 words and `S` is
/// sentences per 100 words.
#[must_use]
pub fn coleman_liau_index(text: &str) -> f64 {
    let (words, sentences, _) = base(text);
    if words == 0.0 {
        return 0.0;
    }
    let letters = letter_count(text) as f64;
    let letters_per_100 = letters / words * 100.0;
    let sentences_per_100 = sentences / words * 100.0;
    0.0588 * letters_per_100 - 0.296 * sentences_per_100 - 15.8
}

/// Estimated reading time in **seconds** at `words_per_minute`.
///
/// Note: unlike Python `textstat`'s `reading_time` (which is character-based with
/// a *milliseconds-per-character* argument), this is word-based and takes
/// *words per minute* — both return seconds.
#[must_use]
pub fn reading_time(text: &str, words_per_minute: f64) -> f64 {
    if words_per_minute <= 0.0 {
        return 0.0;
    }
    lexicon_count(text) as f64 / words_per_minute * 60.0
}

/// Shared `(words, sentences, syllables)` as floats.
fn base(text: &str) -> (f64, f64, f64) {
    (
        lexicon_count(text) as f64,
        sentence_count(text) as f64,
        syllable_count(text) as f64,
    )
}

/// `f64` square root via range-reduced Newton's method (so the crate stays
/// `no_std`). Correct across the full finite range and for non-finite inputs.
fn sqrt(x: f64) -> f64 {
    if x.is_nan() || x < 0.0 {
        return 0.0;
    }
    if x == 0.0 || x == f64::INFINITY {
        return x;
    }
    // Scale into [1, 4) by an even power of two: x = mantissa · 4^exp,
    // so √x = √mantissa · 2^exp.
    let mut mantissa = x;
    let mut exp: i32 = 0;
    while mantissa >= 4.0 {
        mantissa /= 4.0;
        exp += 1;
    }
    while mantissa < 1.0 {
        mantissa *= 4.0;
        exp -= 1;
    }
    // Newton on the small, well-conditioned [1, 4) range converges fast.
    let mut guess = 1.5;
    let mut iter = 0;
    while iter < 12 {
        guess = 0.5 * (guess + mantissa / guess);
        iter += 1;
    }
    // Rescale by 2^exp.
    if exp >= 0 {
        for _ in 0..exp {
            guess *= 2.0;
        }
    } else {
        for _ in 0..-exp {
            guess /= 2.0;
        }
    }
    guess
}
