//! Tokenization and counting: words, sentences, syllables, characters.

use alloc::vec::Vec;

/// Count words: whitespace-separated tokens containing at least one alphanumeric
/// character (so stray punctuation tokens are not counted).
#[must_use]
pub fn lexicon_count(text: &str) -> usize {
    text.split_whitespace()
        .filter(|w| w.chars().any(char::is_alphanumeric))
        .count()
}

/// Count sentences by runs of terminal punctuation (`.`, `!`, `?`). Non-empty
/// text with no terminator counts as one sentence.
///
/// A `.` is **not** treated as a sentence end when it is a decimal point (digit
/// on both sides) or part of an initialism (immediately followed by another
/// letter/digit), which avoids over-counting `3.14` and `U.S.A.`. Runs of
/// terminators (e.g. `...` or `?!`) collapse to one. Trailing abbreviation dots
/// (`Dr.`) can still be over-counted by one — full abbreviation handling is out
/// of scope.
#[must_use]
pub fn sentence_count(text: &str) -> usize {
    if text.trim().is_empty() {
        return 0;
    }
    let chars: Vec<char> = text.chars().collect();
    let mut count = 0;
    let mut in_run = false;
    for i in 0..chars.len() {
        if is_sentence_end(&chars, i) {
            if !in_run {
                count += 1;
                in_run = true;
            }
        } else {
            in_run = false;
        }
    }
    if count == 0 {
        1
    } else {
        count
    }
}

/// Whether the terminator at index `i` plausibly ends a sentence.
fn is_sentence_end(chars: &[char], i: usize) -> bool {
    match chars[i] {
        '!' | '?' => true,
        '.' => {
            let next = chars.get(i + 1).copied();
            let prev = if i > 0 {
                chars.get(i - 1).copied()
            } else {
                None
            };
            // Decimal point: digit on both sides (e.g. "3.14").
            if prev.is_some_and(|c| c.is_ascii_digit()) && next.is_some_and(|c| c.is_ascii_digit())
            {
                return false;
            }
            // Initialism: directly followed by a letter/digit (e.g. "U.S.A").
            if next.is_some_and(char::is_alphanumeric) {
                return false;
            }
            true
        }
        _ => false,
    }
}

/// Count non-whitespace characters (includes punctuation).
#[must_use]
pub fn char_count(text: &str) -> usize {
    text.chars().filter(|c| !c.is_whitespace()).count()
}

/// Count alphabetic characters only.
#[must_use]
pub fn letter_count(text: &str) -> usize {
    text.chars().filter(|c| c.is_alphabetic()).count()
}

/// Estimate the number of syllables in a single English word using a vowel-group
/// heuristic with a silent-`e` rule. Always returns at least 1 for a word with
/// letters, and 0 for a word with none.
#[must_use]
pub fn syllables(word: &str) -> usize {
    let chars: Vec<char> = word
        .chars()
        .filter(|c| c.is_alphabetic())
        .flat_map(char::to_lowercase)
        .collect();
    if chars.is_empty() {
        // No letters: a numeric token (e.g. "100") still reads as >= 1 syllable.
        return usize::from(word.chars().any(char::is_alphanumeric));
    }

    let mut count = 0;
    let mut prev_vowel = false;
    for &c in &chars {
        let vowel = is_vowel(c);
        if vowel && !prev_vowel {
            count += 1;
        }
        prev_vowel = vowel;
    }

    let n = chars.len();
    // Silent trailing 'e', except a "le" ending after a consonant ("ta-ble").
    if chars[n - 1] == 'e' && count > 1 {
        let le_after_consonant = n >= 3 && chars[n - 2] == 'l' && !is_vowel(chars[n - 3]);
        if !le_after_consonant {
            count -= 1;
        }
    }

    // Any word containing letters has at least one syllable (covers non-Latin
    // scripts, which have no Latin vowels but are still words).
    count.max(1)
}

/// Total syllables across all words in `text`.
#[must_use]
pub fn syllable_count(text: &str) -> usize {
    text.split_whitespace().map(syllables).sum()
}

/// Count polysyllabic words (three or more syllables) — used by Gunning Fog and SMOG.
#[must_use]
pub fn polysyllabic_count(text: &str) -> usize {
    text.split_whitespace()
        .filter(|w| syllables(w) >= 3)
        .count()
}

fn is_vowel(c: char) -> bool {
    matches!(
        c,
        'a' | 'e'
            | 'i'
            | 'o'
            | 'u'
            | 'y'
            | 'á'
            | 'à'
            | 'â'
            | 'ä'
            | 'ã'
            | 'å'
            | 'é'
            | 'è'
            | 'ê'
            | 'ë'
            | 'í'
            | 'ì'
            | 'î'
            | 'ï'
            | 'ó'
            | 'ò'
            | 'ô'
            | 'ö'
            | 'õ'
            | 'ú'
            | 'ù'
            | 'û'
            | 'ü'
            | 'ý'
            | 'ÿ'
    )
}
