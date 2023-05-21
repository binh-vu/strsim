use pyo3::prelude::*;

use crate::error::into_pyerr;

use super::{
    CharacterTokenizer, HybridJaccard, Jaro, JaroWinkler, Levenshtein, MongeElkan, StrSim,
    SymmetricMongeElkan, Tokenizer, WhitespaceCharSeqTokenizer,
};

#[pyclass(module = "strsim", name = "WhitespaceCharSeqTokenizer")]
pub struct PyWhitespaceCharSeqTokenizer(WhitespaceCharSeqTokenizer);

#[pyclass(module = "strsim", name = "CharacterTokenizer")]
pub struct PyCharacterTokenizer(CharacterTokenizer);

#[pyclass(module = "strsim")]
pub struct VecVecChar(Vec<Vec<char>>);

#[pyclass(module = "strsim")]
pub struct VecChar(Vec<char>);

#[pymethods]
impl PyWhitespaceCharSeqTokenizer {
    #[new]
    fn new() -> Self {
        PyWhitespaceCharSeqTokenizer(WhitespaceCharSeqTokenizer {})
    }

    fn tokenize(&mut self, s: &str) -> VecVecChar {
        VecVecChar(self.0.tokenize(s))
    }

    fn unique_tokenize(&mut self, s: &str) -> VecVecChar {
        VecVecChar(self.0.unique_tokenize(s))
    }
}

#[pymethods]
impl PyCharacterTokenizer {
    #[new]
    fn new() -> Self {
        PyCharacterTokenizer(CharacterTokenizer {})
    }

    fn tokenize(&mut self, s: &str) -> VecChar {
        VecChar(self.0.tokenize(s))
    }

    fn unique_tokenize(&mut self, s: &str) -> VecChar {
        VecChar(self.0.unique_tokenize(s))
    }
}

#[pyfunction]
pub fn hybrid_jaccard_similarity(key: &VecVecChar, query: &VecVecChar) -> PyResult<f64> {
    HybridJaccard::default()
        .similarity_pre_tok2(&key.0, &query.0)
        .map_err(into_pyerr)
}

#[pyfunction]
pub fn levenshtein_similarity(key: &VecChar, query: &VecChar) -> PyResult<f64> {
    Levenshtein::default()
        .similarity_pre_tok2(&key.0, &query.0)
        .map_err(into_pyerr)
}

#[pyfunction]
#[pyo3(name = "jaro_similarity")]
pub fn jaro_similarity(key: &VecChar, query: &VecChar) -> PyResult<f64> {
    (Jaro {})
        .similarity_pre_tok2(&key.0, &query.0)
        .map_err(into_pyerr)
}

#[pyfunction(name = "jaro_winkler_similarity")]
#[pyo3(signature = (key, query, threshold = 0.7, scaling_factor = 0.1, prefix_len = 4))]
pub fn jaro_winkler_similarity(
    key: &VecChar,
    query: &VecChar,
    threshold: f64,
    scaling_factor: f64,
    prefix_len: usize,
) -> PyResult<f64> {
    (JaroWinkler {
        threshold,
        scaling_factor,
        prefix_len,
    })
    .similarity_pre_tok2(&key.0, &query.0)
    .map_err(into_pyerr)
}

#[pyfunction]
pub fn monge_elkan_similarity(key: &VecVecChar, query: &VecVecChar) -> PyResult<f64> {
    MongeElkan::default()
        .similarity_pre_tok2(&key.0, &query.0)
        .map_err(into_pyerr)
}

#[pyfunction]
pub fn symmetric_monge_elkan_similarity(key: &VecVecChar, query: &VecVecChar) -> PyResult<f64> {
    SymmetricMongeElkan(MongeElkan::default())
        .similarity_pre_tok2(&key.0, &query.0)
        .map_err(into_pyerr)
}
