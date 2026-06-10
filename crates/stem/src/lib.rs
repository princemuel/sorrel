//! Snowball Stemmer Library for Rust
//!
//! This library provides Rust implementations of the Snowball stemming
//! algorithms for multiple languages. It's generated from the official Snowball
//! compiler.
//!
//! # Example
//!
//! ```rust
//! use stem::{Algorithm, stem};
//!
//! // Stem a single word
//! let stemmed = stem(Algorithm::English, "running");
//! assert_eq!(stemmed, "run");
//!
//! // Create a reusable stemmer
//! let stemmer = Algorithm::English.stemmer();
//! let result = stemmer.stem("jumping");
//! assert_eq!(result, "jump");
//! ```

use std::borrow::Cow;

pub mod snowball;
pub use snowball::{Among, SnowballEnv};

// Include all generated algorithm modules
pub mod algorithms {
    include!("algorithms/mod.rs");
}

/// Supported stemming algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Algorithm {
    Arabic,
    Armenian,
    Basque,
    Catalan,
    Danish,
    Dutch,
    DutchPorter,
    English,
    Esperanto,
    Estonian,
    Finnish,
    French,
    German,
    Greek,
    Hindi,
    Hungarian,
    Indonesian,
    Irish,
    Italian,
    Lithuanian,
    Lovins,
    Nepali,
    Norwegian,
    Porter,
    Portuguese,
    Romanian,
    Russian,
    Serbian,
    Spanish,
    Swedish,
    Tamil,
    Turkish,
    Yiddish,
}

impl Algorithm {
    /// Get the string name of the algorithm
    pub fn as_str(&self) -> &str {
        match self {
            Algorithm::Arabic => "arabic",
            Algorithm::Armenian => "armenian",
            Algorithm::Basque => "basque",
            Algorithm::Catalan => "catalan",
            Algorithm::Danish => "danish",
            Algorithm::Dutch => "dutch",
            Algorithm::DutchPorter => "dutch_porter",
            Algorithm::English => "english",
            Algorithm::Esperanto => "esperanto",
            Algorithm::Estonian => "estonian",
            Algorithm::Finnish => "finnish",
            Algorithm::French => "french",
            Algorithm::German => "german",
            Algorithm::Greek => "greek",
            Algorithm::Hindi => "hindi",
            Algorithm::Hungarian => "hungarian",
            Algorithm::Indonesian => "indonesian",
            Algorithm::Irish => "irish",
            Algorithm::Italian => "italian",
            Algorithm::Lithuanian => "lithuanian",
            Algorithm::Lovins => "lovins",
            Algorithm::Nepali => "nepali",
            Algorithm::Norwegian => "norwegian",
            Algorithm::Porter => "porter",
            Algorithm::Portuguese => "portuguese",
            Algorithm::Romanian => "romanian",
            Algorithm::Russian => "russian",
            Algorithm::Serbian => "serbian",
            Algorithm::Spanish => "spanish",
            Algorithm::Swedish => "swedish",
            Algorithm::Tamil => "tamil",
            Algorithm::Turkish => "turkish",
            Algorithm::Yiddish => "yiddish",
        }
    }

    /// Create a stemmer for this algorithm
    pub fn stemmer(&self) -> Stemmer { Stemmer::new(*self) }

    /// Parse algorithm from string name
    pub fn from_str(s: &str) -> Option<Algorithm> {
        match s.to_lowercase().as_str() {
            "arabic" => Some(Algorithm::Arabic),
            "armenian" => Some(Algorithm::Armenian),
            "basque" => Some(Algorithm::Basque),
            "catalan" => Some(Algorithm::Catalan),
            "danish" => Some(Algorithm::Danish),
            "dutch" => Some(Algorithm::Dutch),
            "dutch_porter" => Some(Algorithm::DutchPorter),
            "english" => Some(Algorithm::English),
            "esperanto" => Some(Algorithm::Esperanto),
            "estonian" => Some(Algorithm::Estonian),
            "finnish" => Some(Algorithm::Finnish),
            "french" => Some(Algorithm::French),
            "german" => Some(Algorithm::German),
            "greek" => Some(Algorithm::Greek),
            "hindi" => Some(Algorithm::Hindi),
            "hungarian" => Some(Algorithm::Hungarian),
            "indonesian" => Some(Algorithm::Indonesian),
            "irish" => Some(Algorithm::Irish),
            "italian" => Some(Algorithm::Italian),
            "lithuanian" => Some(Algorithm::Lithuanian),
            "lovins" => Some(Algorithm::Lovins),
            "nepali" => Some(Algorithm::Nepali),
            "norwegian" => Some(Algorithm::Norwegian),
            "porter" => Some(Algorithm::Porter),
            "portuguese" => Some(Algorithm::Portuguese),
            "romanian" => Some(Algorithm::Romanian),
            "russian" => Some(Algorithm::Russian),
            "serbian" => Some(Algorithm::Serbian),
            "spanish" => Some(Algorithm::Spanish),
            "swedish" => Some(Algorithm::Swedish),
            "tamil" => Some(Algorithm::Tamil),
            "turkish" => Some(Algorithm::Turkish),
            "yiddish" => Some(Algorithm::Yiddish),
            _ => None,
        }
    }
}

/// A stemmer for a specific algorithm
pub struct Stemmer {
    algorithm: Algorithm,
}

impl Stemmer {
    /// Create a new stemmer for the given algorithm
    pub fn new(algorithm: Algorithm) -> Self { Stemmer { algorithm } }

    /// Stem a word using this stemmer
    pub fn stem<'a>(&self, word: &'a str) -> Cow<'a, str> { stem(self.algorithm, word) }
}

/// Stem a word using the specified algorithm
///
/// # Arguments
///
/// * `algorithm` - The stemming algorithm to use
/// * `word` - The word to stem (should be lowercase for best results)
///
/// # Returns
///
/// The stemmed word. If the word is unchanged, returns a borrowed reference.
/// If the word is modified, returns an owned String.
pub fn stem(algorithm: Algorithm, word: &str) -> Cow<'_, str> {
    use algorithms::*;

    let mut env = SnowballEnv::create(word);

    match algorithm {
        Algorithm::Arabic => {
            arabic_stemmer::stem(&mut env);
        }
        Algorithm::Armenian => {
            armenian_stemmer::stem(&mut env);
        }
        Algorithm::Basque => {
            basque_stemmer::stem(&mut env);
        }
        Algorithm::Catalan => {
            catalan_stemmer::stem(&mut env);
        }
        Algorithm::Danish => {
            danish_stemmer::stem(&mut env);
        }
        Algorithm::Dutch => {
            dutch_stemmer::stem(&mut env);
        }
        Algorithm::DutchPorter => {
            dutch_porter_stemmer::stem(&mut env);
        }
        Algorithm::English => {
            english_stemmer::stem(&mut env);
        }
        Algorithm::Esperanto => {
            esperanto_stemmer::stem(&mut env);
        }
        Algorithm::Estonian => {
            estonian_stemmer::stem(&mut env);
        }
        Algorithm::Finnish => {
            finnish_stemmer::stem(&mut env);
        }
        Algorithm::French => {
            french_stemmer::stem(&mut env);
        }
        Algorithm::German => {
            german_stemmer::stem(&mut env);
        }
        Algorithm::Greek => {
            greek_stemmer::stem(&mut env);
        }
        Algorithm::Hindi => {
            hindi_stemmer::stem(&mut env);
        }
        Algorithm::Hungarian => {
            hungarian_stemmer::stem(&mut env);
        }
        Algorithm::Indonesian => {
            indonesian_stemmer::stem(&mut env);
        }
        Algorithm::Irish => {
            irish_stemmer::stem(&mut env);
        }
        Algorithm::Italian => {
            italian_stemmer::stem(&mut env);
        }
        Algorithm::Lithuanian => {
            lithuanian_stemmer::stem(&mut env);
        }
        Algorithm::Lovins => {
            lovins_stemmer::stem(&mut env);
        }
        Algorithm::Nepali => {
            nepali_stemmer::stem(&mut env);
        }
        Algorithm::Norwegian => {
            norwegian_stemmer::stem(&mut env);
        }
        Algorithm::Porter => {
            porter_stemmer::stem(&mut env);
        }
        Algorithm::Portuguese => {
            portuguese_stemmer::stem(&mut env);
        }
        Algorithm::Romanian => {
            romanian_stemmer::stem(&mut env);
        }
        Algorithm::Russian => {
            russian_stemmer::stem(&mut env);
        }
        Algorithm::Serbian => {
            serbian_stemmer::stem(&mut env);
        }
        Algorithm::Spanish => {
            spanish_stemmer::stem(&mut env);
        }
        Algorithm::Swedish => {
            swedish_stemmer::stem(&mut env);
        }
        Algorithm::Tamil => {
            tamil_stemmer::stem(&mut env);
        }
        Algorithm::Turkish => {
            turkish_stemmer::stem(&mut env);
        }
        Algorithm::Yiddish => {
            yiddish_stemmer::stem(&mut env);
        }
    }

    env.get_current()
}

/// Get a list of all supported algorithms
pub fn algorithms() -> &'static [Algorithm] {
    &[
        Algorithm::Arabic,
        Algorithm::Armenian,
        Algorithm::Basque,
        Algorithm::Catalan,
        Algorithm::Danish,
        Algorithm::Dutch,
        Algorithm::DutchPorter,
        Algorithm::English,
        Algorithm::Esperanto,
        Algorithm::Estonian,
        Algorithm::Finnish,
        Algorithm::French,
        Algorithm::German,
        Algorithm::Greek,
        Algorithm::Hindi,
        Algorithm::Hungarian,
        Algorithm::Indonesian,
        Algorithm::Irish,
        Algorithm::Italian,
        Algorithm::Lithuanian,
        Algorithm::Lovins,
        Algorithm::Nepali,
        Algorithm::Norwegian,
        Algorithm::Porter,
        Algorithm::Portuguese,
        Algorithm::Romanian,
        Algorithm::Russian,
        Algorithm::Serbian,
        Algorithm::Spanish,
        Algorithm::Swedish,
        Algorithm::Tamil,
        Algorithm::Turkish,
        Algorithm::Yiddish,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_english_stemming() {
        assert_eq!(stem(Algorithm::English, "running"), "run");
        assert_eq!(stem(Algorithm::English, "flies"), "fli");
        assert_eq!(stem(Algorithm::English, "dogs"), "dog");
    }

    #[test]
    fn test_algorithm_from_str() {
        assert_eq!(Algorithm::from_str("english"), Some(Algorithm::English));
        assert_eq!(Algorithm::from_str("FRENCH"), Some(Algorithm::French));
        assert_eq!(Algorithm::from_str("invalid"), None);
    }

    #[test]
    fn test_stemmer() {
        let stemmer = Algorithm::English.stemmer();
        assert_eq!(stemmer.stem("running"), "run");
    }
}
