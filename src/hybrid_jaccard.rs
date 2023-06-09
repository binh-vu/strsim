use crate::error::StrSimError;

use super::{ExpectTokenizerType, JaroWinkler, StrSim, TokenizerType};
use anyhow::Result;
use derive_more::Display;
use lsap::get_assigned_cost;

#[derive(Display)]
#[display(fmt = "HybridJaccard")]
pub struct HybridJaccard<S: StrSim<Vec<char>> + ExpectTokenizerType> {
    pub threshold: f64,
    pub lower_bound: f64,
    pub strsim: S,
}

impl HybridJaccard<JaroWinkler> {
    pub fn default() -> Self {
        HybridJaccard {
            threshold: 0.5,
            lower_bound: 0.0,
            strsim: JaroWinkler::default(),
        }
    }
}

impl<S: StrSim<Vec<char>> + ExpectTokenizerType> HybridJaccard<S> {
    pub fn new(strsim: S, threshold: Option<f64>, lower_bound: Option<f64>) -> Self {
        HybridJaccard {
            threshold: threshold.unwrap_or(0.5),
            lower_bound: lower_bound.unwrap_or(0.0),
            strsim,
        }
    }

    pub fn similarity<'t>(
        &self,
        mut set1: &'t Vec<Vec<char>>,
        mut set2: &'t Vec<Vec<char>>,
    ) -> Result<f64, StrSimError> {
        if set1.len() > set2.len() {
            (set1, set2) = (set2, set1);
        }
        let total_num_matches = set1.len();
        let mut matching_score = vec![1.0; set1.len() * set2.len()];
        // let mut matching_score = Array2::from_elem((set1.len(), set2.len()), 1.0);
        let mut row_max: Vec<f64> = vec![0.0; set1.len()];

        for (i, s1) in set1.iter().enumerate() {
            for (j, s2) in set2.iter().enumerate() {
                let mut score: f64 = self.strsim.similarity_pre_tok2(s1, s2)?;
                if score < self.threshold {
                    score = 0.0;
                }
                row_max[i] = row_max[i].max(score);
                // matching_score[[i, j]] = 1.0 - score // munkres finds out the smallest element
                // matching_score[[i, j]] = score
                matching_score[i * set2.len() + j] = score
            }

            if self.lower_bound > 0.0 {
                let max_possible_score_sum: f64 =
                    row_max[..i + 1].iter().sum::<f64>() + (total_num_matches - i - 1) as f64;
                let max_possible =
                    max_possible_score_sum / (set1.len() + set2.len() - total_num_matches) as f64;
                if max_possible < self.lower_bound {
                    return Ok(0.0);
                }
            }
        }

        let score_sum = get_assigned_cost(set1.len(), set2.len(), &matching_score, true)?;

        if set1.len() + set2.len() - total_num_matches == 0 {
            return Ok(1.0);
        }
        let sim = score_sum / (set1.len() + set2.len() - total_num_matches) as f64;
        if self.lower_bound > 0.0 && sim < self.lower_bound {
            Ok(0.0)
        } else {
            Ok(sim)
        }
    }

    // /**
    //  *
    //  */
    // fn similarity_impl_v1(&self, mut set1: &Vec<Vec<char>>, mut set2: &Vec<Vec<char>>) -> f64 {
    //     if set1.len() > set2.len() {
    //         let tmp = set1;
    //         set1 = set2;
    //         set2 = set1;
    //     }

    //     let mut match_score = 0.0;
    //     let mut match_count = 0.0;
    //     let mut matches = vec![];

    //     for (i, s1) in set1.iter().enumerate() {
    //         for (j, s2) in set2.iter().enumerate() {
    //             let mut score = self.strsim.similarity(s1, s2);
    //             if score > self.threshold {
    //                 matches.push((s1, s2, score));
    //             }
    //         }
    //     }

    //     // sort the score of all the pairs
    //     matches.sort_by(|a, b| b[2].partial_cmp(&a[2]).unwrap());

    //     // select score in increasing order of their weightage
    //     // do not reselect the same element from either set.
    //     let mut set1x = HashSet::new();
    //     let mut set2x = HashSet::new();
    //     for (s1, s2, score) in matches {
    //         if !set1x.contains(s1) && !set2x.contains(s2) {
    //             set1x.add(s1);
    //             set2x.add(s2);
    //             match_score += score;
    //             match_count += 1.0;
    //         }
    //     }

    //     match_score / (set1.len() + set2.len() - match_count)
    // }
}

impl<S: StrSim<Vec<char>> + ExpectTokenizerType> StrSim<Vec<Vec<char>>> for HybridJaccard<S> {
    fn similarity_pre_tok2(
        &self,
        set1: &Vec<Vec<char>>,
        set2: &Vec<Vec<char>>,
    ) -> Result<f64, StrSimError> {
        self.similarity(set1, set2)
    }
}

impl<S: StrSim<Vec<char>> + ExpectTokenizerType> ExpectTokenizerType for HybridJaccard<S> {
    fn get_expected_tokenizer_type(&self) -> TokenizerType {
        TokenizerType::Set(Box::new(Some(self.strsim.get_expected_tokenizer_type())))
    }
}
