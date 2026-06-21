//! End-to-end behavioral spec for the public `textstat` API.

// The empty-text guards return a literal 0.0, so exact comparison is intended.
#![allow(clippy::float_cmp)]

use textstat::{
    automated_readability_index, char_count, coleman_liau_index, flesch_kincaid_grade,
    flesch_reading_ease, gunning_fog, letter_count, lexicon_count, polysyllabic_count,
    reading_time, sentence_count, smog_index, syllable_count, syllables,
};

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.05
}

// All-monosyllabic text: 10 words, 2 sentences, 10 syllables, 32 non-space
// chars, 30 letters — so every metric is hand-computable and exact.
const T: &str = "The cat sat on the mat. The dog ran fast.";

// ---------------------------------------------------------------------------
// Counts
// ---------------------------------------------------------------------------

#[test]
fn counts_words_sentences() {
    assert_eq!(lexicon_count(T), 10);
    assert_eq!(sentence_count(T), 2);
    assert_eq!(sentence_count("One. Two! Three?"), 3);
    assert_eq!(sentence_count("no terminator here"), 1);
}

#[test]
fn counts_chars_and_letters() {
    assert_eq!(char_count(T), 32); // non-whitespace, includes the two periods
    assert_eq!(letter_count(T), 30); // alphabetic only
}

#[test]
fn counts_syllables() {
    assert_eq!(syllables("cat"), 1);
    assert_eq!(syllables("hello"), 2);
    assert_eq!(syllables("table"), 2);
    assert_eq!(syllables("make"), 1);
    assert_eq!(syllables("readability"), 5);
    assert_eq!(syllables("the"), 1);
    assert_eq!(syllable_count(T), 10);
}

#[test]
fn counts_polysyllabic() {
    assert_eq!(polysyllabic_count("cat dog the"), 0);
    assert_eq!(polysyllabic_count("beautiful"), 1);
    assert_eq!(polysyllabic_count(T), 0);
}

// ---------------------------------------------------------------------------
// Readability metrics (exact, hand-computed from the counts above)
// ---------------------------------------------------------------------------

#[test]
fn flesch_reading_ease_value() {
    // 206.835 - 1.015*(10/2) - 84.6*(10/10) = 117.16
    assert!(
        close(flesch_reading_ease(T), 117.16),
        "{}",
        flesch_reading_ease(T)
    );
}

#[test]
fn flesch_kincaid_grade_value() {
    // 0.39*5 + 11.8*1 - 15.59 = -1.84
    assert!(
        close(flesch_kincaid_grade(T), -1.84),
        "{}",
        flesch_kincaid_grade(T)
    );
}

#[test]
fn gunning_fog_value() {
    // 0.4*((10/2) + 100*(0/10)) = 2.0
    assert!(close(gunning_fog(T), 2.0), "{}", gunning_fog(T));
}

#[test]
fn smog_index_value() {
    // 1.0430*sqrt(0 * 30/2) + 3.1291 = 3.1291
    assert!(close(smog_index(T), 3.1291), "{}", smog_index(T));
}

#[test]
fn automated_readability_index_value() {
    // 4.71*(32/10) + 0.5*(10/2) - 21.43 = -3.858
    assert!(
        close(automated_readability_index(T), -3.858),
        "{}",
        automated_readability_index(T)
    );
}

#[test]
fn coleman_liau_index_value() {
    // 0.0588*300 - 0.296*20 - 15.8 = -4.08
    assert!(
        close(coleman_liau_index(T), -4.08),
        "{}",
        coleman_liau_index(T)
    );
}

#[test]
fn reading_time_seconds() {
    // 10 words / 200 wpm * 60 = 3.0 seconds
    assert!(
        close(reading_time(T, 200.0), 3.0),
        "{}",
        reading_time(T, 200.0)
    );
}

// ---------------------------------------------------------------------------
// Harder text scores higher (sanity / monotonicity)
// ---------------------------------------------------------------------------

#[test]
fn complex_text_is_graded_harder() {
    let hard = "Extraordinary circumstances necessitate immediate, comprehensive reconsideration.";
    assert!(polysyllabic_count(hard) >= 4);
    // Harder text => lower reading ease, higher grade level than the simple text.
    assert!(flesch_reading_ease(hard) < flesch_reading_ease(T));
    assert!(flesch_kincaid_grade(hard) > flesch_kincaid_grade(T));
    assert!(gunning_fog(hard) > 15.0);
}

// ---------------------------------------------------------------------------
// Edge cases
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Regression tests from the adversarial pre-publish review
// ---------------------------------------------------------------------------

#[test]
fn sentence_count_ignores_decimals() {
    assert_eq!(sentence_count("The value is 3.14 today."), 1);
    assert_eq!(sentence_count("Pi is 3.14159 and e is 2.718."), 1);
    assert_eq!(sentence_count("Values: 1.5 2.5 3.5 4.5."), 1);
}

#[test]
fn sentence_count_reduces_initialism_overcount() {
    // U.S.A. no longer counts each internal dot (was 4); ellipsis stays one.
    assert_eq!(sentence_count("I live in the U.S.A. now."), 2);
    assert_eq!(sentence_count("Wait... what happened?"), 2);
}

#[test]
fn syllables_count_accented_and_nonascii() {
    assert_eq!(syllables("café"), 2); // accented vowel is counted
    assert!(syllables("Москва") >= 1); // non-Latin word gets the >=1 floor
    assert_eq!(syllable_count("100 200 300"), 3); // numeric tokens are >=1 each
}

#[test]
fn non_ascii_text_yields_bounded_scores() {
    // Previously produced ~204 (far out of range) due to zero syllables.
    let cyr = "Это короткий текст. Он простой.";
    let fre = flesch_reading_ease(cyr);
    assert!(fre > 0.0 && fre <= 122.0, "out of range: {fre}");
}

#[test]
fn empty_text_is_safe() {
    assert_eq!(lexicon_count(""), 0);
    assert_eq!(sentence_count(""), 0);
    assert_eq!(syllable_count(""), 0);
    assert_eq!(flesch_reading_ease(""), 0.0);
    assert_eq!(flesch_kincaid_grade(""), 0.0);
    assert_eq!(gunning_fog(""), 0.0);
    assert_eq!(smog_index(""), 0.0);
    assert_eq!(automated_readability_index(""), 0.0);
    assert_eq!(coleman_liau_index(""), 0.0);
    assert_eq!(reading_time("", 200.0), 0.0);
}
