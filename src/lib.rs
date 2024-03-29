mod error;
mod helper;
mod hybrid_jaccard;
mod jaro;
mod jaro_winkler;
mod levenshtein;
mod monge_elkan;
mod python;
mod tokenizers;
mod wrapped_strsim;

use anyhow::Result;

use pyo3::{prelude::*, types::PyList};

pub use self::hybrid_jaccard::HybridJaccard;
pub use self::jaro::Jaro;
pub use self::jaro_winkler::JaroWinkler;
pub use self::levenshtein::Levenshtein;
pub use self::monge_elkan::{MongeElkan, SymmetricMongeElkan};

pub use self::tokenizers::{
    CachedWhitespaceTokenizer, CharacterTokenizer, WhitespaceCharSeqTokenizer, WhitespaceTokenizer,
};
pub use self::wrapped_strsim::SeqStrSim;
pub use crate::error::StrSimError;
pub use crate::helper::ReturnKind;

pub trait StrSim<T> {
    /** Calculate the similarity with both key and query has already``` been pre-tokenized */
    fn similarity_pre_tok2(
        &self,
        tokenized_key: &T,
        tokenized_query: &T,
    ) -> Result<f64, StrSimError>;
}

pub trait ExpectTokenizerType {
    fn get_expected_tokenizer_type(&self) -> TokenizerType;
}

pub trait StrSimWithTokenizer<T>: StrSim<T> {
    ///
    /// Calculate the similarity between two strings. Usually, the similarity function is symmetric so
    /// key and query can be swapped. However, some functions such as monge-elkan are not symmetric, so
    /// key and query takes specific meaning: key is the value in the database and query is the search
    /// query from the user.
    ///
    /// The return value is a likelihood between 0 and 1.
    ///
    /// # Arguments
    ///
    /// * `key` the value in the database (e.g., entity label)
    /// * `query` the search query from the user (e.g., cell in the table)
    ///
    fn similarity(&self, key: &str, query: &str) -> Result<f64, StrSimError>;

    ///
    /// Calculate the similarity with the query's already been pre-tokenized
    ///
    fn similarity_pre_tok1(&self, key: &str, tokenized_query: &T) -> Result<f64, StrSimError>;

    ///
    /// Tokenize a string into a tokens used for this method.
    ///
    fn tokenize(&self, str: &str) -> T;

    ///
    /// Tokenize a list of strings into a list of tokens used for this method.
    ///
    fn tokenize_list(&self, strs: &[&str]) -> Vec<T>;
}

/// A mirror trait of StrSimWithTokenizer, but requires mutable self to invoke functions that call
/// tokenizers' functions.
pub trait StrSimWithMutTokenizer<T>: StrSim<T> {
    /// See StrSimWithTokenizer::similarity
    fn similarity(&mut self, key: &str, query: &str) -> Result<f64, StrSimError>;

    /// See StrSimWithTokenizer::similarity_pre_tok1
    fn similarity_pre_tok1(&mut self, key: &str, tokenized_query: &T) -> Result<f64, StrSimError>;

    /// See StrSimWithTokenizer::tokenize
    fn tokenize(&mut self, str: &str) -> T;

    /// See StrSimWithTokenizer::tokenize_list
    fn tokenize_list(&mut self, strs: &[&str]) -> Vec<T>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenizerType {
    Seq(Box<Option<TokenizerType>>),
    Set(Box<Option<TokenizerType>>),
}

impl TokenizerType {
    fn is_outer_seq(&self) -> bool {
        match self {
            TokenizerType::Seq(_) => true,
            TokenizerType::Set(_) => false,
        }
    }

    #[allow(dead_code)]
    #[inline]
    fn is_outer_set(&self) -> bool {
        !self.is_outer_seq()
    }

    fn has_nested(&self) -> bool {
        match self {
            TokenizerType::Seq(inner) => inner.is_some(),
            TokenizerType::Set(inner) => inner.is_some(),
        }
    }

    fn get_nested(&self) -> &Option<TokenizerType> {
        match self {
            TokenizerType::Seq(inner) => inner.as_ref(),
            TokenizerType::Set(inner) => inner.as_ref(),
        }
    }
}

pub trait BaseTokenizer<T> {
    type Return: for<'t> ReturnKind<'t, T>;

    fn is_compatible(&self, tok_type: &TokenizerType) -> bool;
}

pub trait Tokenizer<T>: BaseTokenizer<T> {
    fn tokenize<'t>(&'t self, s: &str) -> <Self::Return as ReturnKind<'t, T>>::Type;
    fn tokenize_pair<'t>(
        &'t self,
        key: &str,
        query: &str,
    ) -> (
        <Self::Return as ReturnKind<'t, T>>::Type,
        <Self::Return as ReturnKind<'t, T>>::Type,
    );
    fn unique_tokenize<'t>(&'t self, s: &str) -> <Self::Return as ReturnKind<'t, T>>::Type;
    fn unique_tokenize_pair<'t>(
        &'t self,
        key: &str,
        query: &str,
    ) -> (
        <Self::Return as ReturnKind<'t, T>>::Type,
        <Self::Return as ReturnKind<'t, T>>::Type,
    );
}

pub trait MutTokenizer<T>: BaseTokenizer<T> {
    fn tokenize<'t>(&'t mut self, s: &str) -> <Self::Return as ReturnKind<'t, T>>::Type;
    fn tokenize_pair<'t>(
        &'t mut self,
        key: &str,
        query: &str,
    ) -> (
        <Self::Return as ReturnKind<'t, T>>::Type,
        <Self::Return as ReturnKind<'t, T>>::Type,
    );
    fn unique_tokenize<'t>(&'t mut self, s: &str) -> <Self::Return as ReturnKind<'t, T>>::Type;
    fn unique_tokenize_pair<'t>(
        &'t mut self,
        key: &str,
        query: &str,
    ) -> (
        <Self::Return as ReturnKind<'t, T>>::Type,
        <Self::Return as ReturnKind<'t, T>>::Type,
    );
}

impl<'t, T: Sized + 't> ReturnKind<'t, T> for Vec<char> {
    type Type = Vec<char>;
}

impl<'t, T: Sized + 't> ReturnKind<'t, T> for Vec<Vec<char>> {
    type Type = Vec<Vec<char>>;
}

impl<'t, T: Sized + 't> ReturnKind<'t, T> for Vec<String> {
    type Type = Vec<String>;
}

impl<'t, T: Sized + 't> ReturnKind<'t, T> for &'t Vec<String> {
    type Type = &'t Vec<String>;
}

/// String similarity functions that are implemented in Rust.
#[cfg(feature = "extension-module")]
#[pymodule]
#[pyo3(name = "strsim")]
pub fn yass(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.setattr("__path__", PyList::empty(py))?;

    m.add_class::<self::python::PyWhitespaceCharSeqTokenizer>()?;
    m.add_class::<self::python::PyCharacterTokenizer>()?;
    m.add_class::<self::python::VecVecChar>()?;
    m.add_function(wrap_pyfunction!(self::python::levenshtein_similarity, m)?)?;
    m.add_function(wrap_pyfunction!(self::python::jaro_similarity, m)?)?;
    m.add_function(wrap_pyfunction!(self::python::jaro_winkler_similarity, m)?)?;
    m.add_function(wrap_pyfunction!(self::python::monge_elkan_similarity, m)?)?;
    m.add_function(wrap_pyfunction!(
        self::python::symmetric_monge_elkan_similarity,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        self::python::hybrid_jaccard_similarity,
        m
    )?)?;

    Ok(())
}
