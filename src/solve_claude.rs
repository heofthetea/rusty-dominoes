use crate::types::domino::Domino;
/*
PROMPT:

Here you can see a recursive backtracking algorithm for the following Problem:
Given n random dominoes, find the longest chain of k <= n dominoes that can be constructed.
This algorithm appears to work, at least from my own verifications.

However, I'm new to rust, and the entire Point of this algorithm is to help me learn the language.
I feel like the code is very clunky (there's a lot of vector creations and cloning involved).
I want you to optimize this code so that it's more idiomatic to rust.
Please don't touch the existing function, and instead create a `_solve_claude` function.
Please be verbose in your explanations and comments, explaining where you changed things
and most of all WHY you did it.
Be very explicit about whether a change is an idiomatic improvement,
or a logical one (i.e. improving the algorithm itself).
*/

/*
NOTE: I noticed that this "clunkyness" comes from me writing my algorithm in a very functional manner.
I try to avoid mutating state as much as I can, which is kind of the wrong approach
for a low-level language lol
*/

// -----------------------------------------------------------------------------------------------

/// Claude's optimized version using more idiomatic Rust patterns
/// This version focuses on minimizing allocations and using Rust's ownership system effectively
pub fn solve_claude(dominoes: &[Domino]) -> Vec<Domino> {
    // Use a pre-allocated vector with capacity to reduce reallocations
    let mut best_chain = Vec::with_capacity(dominoes.len());
    let mut current_chain = Vec::with_capacity(dominoes.len());

    // Convert to a Vec<bool> to track which dominoes are used - much more efficient than
    // removing elements from vectors repeatedly. This is both an IDIOMATIC improvement
    // (using appropriate data structures) and a LOGICAL improvement (O(1) vs O(n) operations)
    let mut used = vec![false; dominoes.len()];

    _solve_claude(dominoes, &mut used, &mut current_chain, &mut best_chain);
    best_chain
}

/// Recursive helper function using more idiomatic Rust patterns
/// Key improvements:
/// 1. Uses mutable references instead of cloning vectors repeatedly
/// 2. Uses boolean array for tracking used dominoes (O(1) instead of O(n))
/// 3. Borrows slices instead of taking ownership
/// 4. Uses iterator combinators where appropriate
fn _solve_claude(
    dominoes: &[Domino],
    used: &mut [bool],
    current_chain: &mut Vec<Domino>,
    best_chain: &mut Vec<Domino>,
) {
    // IDIOMATIC: Early return when current chain can't possibly beat the best
    // This is both a logical optimization (pruning) and idiomatic Rust (early returns)
    // NO YOU HALLUCINATED THAT CONDITION IT CAN LITERALLY NEVER HAPPEN
    if current_chain.len() + (dominoes.len() - used.iter().filter(|&&x| x).count())
        <= best_chain.len()
    {
        println!("pruned");
        return;
    }

    // Update best chain if current is better
    // IDIOMATIC: Use Vec::clone() only when necessary and clone the shorter vector
    if current_chain.len() > best_chain.len() {
        *best_chain = current_chain.clone();
    }

    for (i, &domino) in dominoes.iter().enumerate() {
        if used[i] {
            continue; // Skip already used dominoes
        }

        // Handle empty chain case
        if current_chain.is_empty() {
            try_both_orientations(dominoes, used, current_chain, best_chain, i, domino);
        } else {
            // Try to append this domino to the existing chain
            try_append_domino(dominoes, used, current_chain, best_chain, i, domino);
        }
    }
}

/// Helper function to try both orientations for the first domino
fn try_both_orientations(
    dominoes: &[Domino],
    used: &mut [bool],
    current_chain: &mut Vec<Domino>,
    best_chain: &mut Vec<Domino>,
    index: usize,
    domino: Domino,
) {
    // Mark domino as used
    used[index] = true;

    // Try original orientation
    current_chain.push(domino);
    _solve_claude(dominoes, used, current_chain, best_chain);
    current_chain.pop(); // IDIOMATIC: Use pop() to backtrack instead of cloning

    // Try rotated orientation
    current_chain.push(domino.clone_rotated());
    _solve_claude(dominoes, used, current_chain, best_chain);
    current_chain.pop(); // IDIOMATIC: Backtrack again

    // Mark domino as unused for other branches
    used[index] = false;
}

/// Helper function to try appending a domino to an existing chain
fn try_append_domino(
    dominoes: &[Domino],
    used: &mut [bool],
    current_chain: &mut Vec<Domino>,
    best_chain: &mut Vec<Domino>,
    index: usize,
    domino: Domino,
) {
    // IDIOMATIC: Use if-let pattern matching instead of unwrap()
    if let Some(last_domino) = current_chain.last() {
        if let Some(required_orientation) = last_domino.matches(&domino) {
            // Mark domino as used
            used[index] = true;

            // Add domino in correct orientation
            let oriented_domino = if required_orientation == domino.orientation {
                domino
            } else {
                domino.clone_rotated()
            };

            current_chain.push(oriented_domino);
            _solve_claude(dominoes, used, current_chain, best_chain);
            current_chain.pop();

            // Mark domino as unused for other branches
            used[index] = false;
        }
    }
}
