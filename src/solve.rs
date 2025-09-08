use crate::types::domino::{Domino};

pub fn solve(dominoes: &[Domino]) -> Vec<Domino> {
    _solve(&Vec::new(), dominoes)
}

/// Recursively build a chain of length `target` using a backtracking approach.
/// Note: this algorithm is built very functional. It mutates state only when absolutely necessary.
///
/// Returns: the longest chain built
fn _solve(chain: &[Domino], remaining: &[Domino]) -> Vec<Domino> {
    let mut longest: Vec<Domino> = Vec::from(chain);
    for (i, domino) in remaining.iter().enumerate() {
        // Always remove the current domino from the remaining ones
        let mut new_remaining = Vec::from(remaining);
        new_remaining.remove(i);

        // We need to try the first domino both ways
        if chain.is_empty() {
            let res = start_new_chain(domino, &new_remaining);
            if res.len() > longest.len() {
                longest = res;
            }
            continue;
        }

        let orientation = chain.last().unwrap().matches(domino);
        if orientation.is_none() { // = does not match
            continue;
        }
        let orientation = orientation.unwrap();

        // append domino to chain in correct orientation
        let mut new_chain = Vec::from(chain);
        new_chain.push(if orientation == domino.orientation {
            domino.clone()
        } else {
            domino.clone_rotated()
        });

        let res = _solve(&new_chain, &new_remaining);
        if res.len() > longest.len() {
            longest = res;
        }
    }
    return longest;
}

fn start_new_chain(domino: &Domino, remaining: &[Domino]) -> Vec<Domino> {
    let res_1 = _solve(&[domino.clone()], &remaining);
    let res_2 = _solve(&[domino.clone_rotated()], &remaining);

    return if res_1.len() > res_2.len() {
        res_1
    } else {
        res_2
    };
}
