use std::fmt::Display;

use super::super::{BaseTokenizer, ExpectTokenizerType, StrSim, StrSimWithTokenizer, Tokenizer};
use crate::{error::StrSimError, helper::ByValue, MutTokenizer, StrSimWithMutTokenizer};
use anyhow::Result;

pub struct SetStrSim<
    't,
    T,
    SS: StrSim<T> + Display + ExpectTokenizerType,
    TK: BaseTokenizer<T, Return = ByValue> + Display,
> {
    pub tokenizer: &'t mut TK,
    pub strsim: SS,
    phantom: std::marker::PhantomData<T>,
}

impl<
        't,
        T,
        SS: StrSim<T> + Display + ExpectTokenizerType,
        TK: BaseTokenizer<T, Return = ByValue> + Display,
    > SetStrSim<'t, T, SS, TK>
{
    #[allow(dead_code)]
    pub fn new(tokenizer: &'t mut TK, strsim: SS) -> Result<Self, StrSimError> {
        let expect_tok_type = strsim.get_expected_tokenizer_type();
        if !tokenizer.is_compatible(&expect_tok_type) || !expect_tok_type.is_outer_set() {
            Err(StrSimError::InvalidConfigData(format!(
                "StrSim {} expect {:?} tokenizer, but get {} which is not compatible",
                strsim, expect_tok_type, tokenizer,
            )))
        } else {
            Ok(Self {
                tokenizer,
                strsim,
                phantom: std::marker::PhantomData,
            })
        }
    }
}

impl<
        't,
        T,
        SS: StrSim<T> + Display + ExpectTokenizerType,
        TK: Tokenizer<T, Return = ByValue> + Display,
    > StrSimWithTokenizer<T> for SetStrSim<'t, T, SS, TK>
{
    fn similarity(&self, key: &str, query: &str) -> Result<f64, StrSimError> {
        let (s1, s2) = self.tokenizer.unique_tokenize_pair(key, query);
        self.strsim.similarity_pre_tok2(&s1, &s2)
    }

    fn similarity_pre_tok1(&self, key: &str, tokenized_query: &T) -> Result<f64, StrSimError> {
        let s1 = self.tokenizer.unique_tokenize(key);
        self.strsim.similarity_pre_tok2(&s1, tokenized_query)
    }

    fn tokenize(&self, str: &str) -> T {
        self.tokenizer.unique_tokenize(str)
    }

    fn tokenize_list(&self, strs: &[&str]) -> Vec<T> {
        strs.iter()
            .map(|s| self.tokenizer.unique_tokenize(s))
            .collect::<Vec<T>>()
    }
}

impl<
        't,
        T,
        SS: StrSim<T> + Display + ExpectTokenizerType,
        TK: MutTokenizer<T, Return = ByValue> + Display,
    > StrSimWithMutTokenizer<T> for SetStrSim<'t, T, SS, TK>
{
    fn similarity(&mut self, key: &str, query: &str) -> Result<f64, StrSimError> {
        let (s1, s2) = self.tokenizer.unique_tokenize_pair(key, query);
        self.strsim.similarity_pre_tok2(&s1, &s2)
    }

    fn similarity_pre_tok1(&mut self, key: &str, tokenized_query: &T) -> Result<f64, StrSimError> {
        let s1 = self.tokenizer.unique_tokenize(key);
        self.strsim.similarity_pre_tok2(&s1, tokenized_query)
    }

    fn tokenize(&mut self, str: &str) -> T {
        self.tokenizer.unique_tokenize(str)
    }

    fn tokenize_list(&mut self, strs: &[&str]) -> Vec<T> {
        strs.iter()
            .map(|s| self.tokenizer.unique_tokenize(s))
            .collect::<Vec<T>>()
    }
}

impl<
        't,
        T,
        SS: StrSim<T> + Display + ExpectTokenizerType,
        TK: BaseTokenizer<T, Return = ByValue> + Display,
    > StrSim<T> for SetStrSim<'t, T, SS, TK>
{
    fn similarity_pre_tok2(
        &self,
        tokenized_key: &T,
        tokenized_query: &T,
    ) -> Result<f64, StrSimError> {
        self.strsim
            .similarity_pre_tok2(tokenized_key, tokenized_query)
    }
}
