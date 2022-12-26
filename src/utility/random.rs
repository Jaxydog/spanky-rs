use std::ops::{Deref, DerefMut};

use rand::{thread_rng, Rng};

use crate::prelude::*;

#[repr(transparent)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WeightVec<T>(Vec<(T, f64)>);

impl<T> WeightVec<T> {
    pub fn get(&self) -> Option<&T> {
        let weights = self.iter().map(|(_, w)| *w).enumerate().fold(
            Vec::with_capacity(self.0.len()),
            |mut v, (i, w)| {
                v.push(if i > 0 { w + v[i - 1] } else { w });
                v
            },
        );

        let weight = thread_rng().gen_range(0.0..=*weights.last()?);

        self.iter()
            .zip(weights.into_iter())
            .find(|(_, w)| *w >= weight)
            .map(|((v, _), _)| v)
    }
}

impl<T> Deref for WeightVec<T> {
    type Target = [(T, f64)];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for WeightVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> IntoIterator for WeightVec<T> {
    type Item = (T, f64);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
