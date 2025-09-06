use crate::types::domino::Domino;
use rand::prelude::*;

pub fn choose_random(k: usize) -> Vec<Domino> {
    let mut rng = rand::rng();
    let mut dominoes = generate_all();

    dominoes.shuffle(&mut rng);
    dominoes.truncate(k);
    // println!("{:?}", dominoes);

    return dominoes;
}

/// Generate a vector containing all possible dominoes.
/// TODO: this is broken because duplicates
fn generate_all() -> Vec<Domino> {
    let mut dominoes = Vec::new();

    for i in 1..7 {
        for ii in 1..7 {
            dominoes.push(Domino::new(i, ii));
        }
    }

    return dominoes;
}
