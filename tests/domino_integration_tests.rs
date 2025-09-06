use domino::types::domino::{Domino, Orientation};

#[test]
fn test_matches_left_orientation_with_left_match() {
    // Test case: domino (2|3) matches with (3|4) on the left
    let domino1 = Domino::new(2, 3); // orientation: LEFT, actual_right = 3
    let domino2 = Domino::new(3, 4); // left = 3

    let result = domino1.matches(domino2);
    assert_eq!(result, Some(Orientation::LEFT));
}

#[test]
fn test_matches_left_orientation_with_right_match() {
    // Test case: domino (2|3) matches with (4|3) on the right
    let domino1 = Domino::new(2, 3); // orientation: LEFT, actual_right = 3
    let domino2 = Domino::new(4, 3); // right = 3

    let result = domino1.matches(domino2);
    assert_eq!(result, Some(Orientation::RIGHT));
}

#[test]
fn test_matches_right_orientation_with_left_match() {
    // Test case: domino (2|3) rotated matches with (2|4) on the left
    let mut domino1 = Domino::new(2, 3);
    domino1.rotate(); // orientation: RIGHT, actual_right = 2
    let domino2 = Domino::new(2, 4); // left = 2

    let result = domino1.matches(domino2);
    assert_eq!(result, Some(Orientation::LEFT));
}

#[test]
fn test_matches_right_orientation_with_right_match() {
    // Test case: domino (2|3) rotated matches with (4|2) on the right
    let mut domino1 = Domino::new(2, 3);
    domino1.rotate(); // orientation: RIGHT, actual_right = 2
    let domino2 = Domino::new(4, 2); // right = 2

    let result = domino1.matches(domino2);
    assert_eq!(result, Some(Orientation::RIGHT));
}

#[test]
fn test_matches_no_match() {
    // Test case: domino (2|3) doesn't match with (4|5)
    let domino1 = Domino::new(2, 3); // orientation: LEFT, actual_right = 3
    let domino2 = Domino::new(4, 5); // left = 4, right = 5

    let result = domino1.matches(domino2);
    assert_eq!(result, None);
}

#[test]
fn test_matches_same_domino() {
    // Test case: domino (2|3) matches with itself
    let domino1 = Domino::new(2, 3); // orientation: LEFT, actual_right = 3
    let domino2 = Domino::new(3, 2); // left = 3

    let result = domino1.matches(domino2);
    assert_eq!(result, Some(Orientation::LEFT));
}

#[test]
fn test_matches_double_domino() {
    // Test case: double domino (3|3) matching scenarios
    let domino1 = Domino::new(2, 3); // orientation: LEFT, actual_right = 3
    let domino2 = Domino::new(3, 3); // left = 3, right = 3

    let result = domino1.matches(domino2);
    assert_eq!(result, Some(Orientation::LEFT)); // Should match on left since we check left first
}

#[test]
fn test_matches_zero_values() {
    // Test case: dominoes with zero values
    let domino1 = Domino::new(0, 1); // orientation: LEFT, actual_right = 1
    let domino2 = Domino::new(1, 0); // left = 1

    let result = domino1.matches(domino2);
    assert_eq!(result, Some(Orientation::LEFT));
}

#[test]
fn test_matches_negative_values() {
    // Test case: dominoes with negative values
    let domino1 = Domino::new(-1, 2); // orientation: LEFT, actual_right = 2
    let domino2 = Domino::new(2, -3); // left = 2

    let result = domino1.matches(domino2);
    assert_eq!(result, Some(Orientation::LEFT));
}

#[test]
fn test_rotate_functionality() {
    // Test the rotate method to ensure it works correctly
    let mut domino = Domino::new(1, 2);
    assert_eq!(domino.orientation, Orientation::LEFT);

    domino.rotate();
    assert_eq!(domino.orientation, Orientation::RIGHT);

    domino.rotate();
    assert_eq!(domino.orientation, Orientation::LEFT);
}

#[test]
fn test_new_domino_default_orientation() {
    // Test that new dominoes have LEFT orientation by default
    let domino = Domino::new(1, 2);
    assert_eq!(domino.orientation, Orientation::LEFT);
    assert_eq!(domino.left, 1);
    assert_eq!(domino.right, 2);
}

// -----------------------------------------------------------------------------------------------------

#[test]
fn test_domino_chain_matching() {
    // Integration test: Test a chain of domino matches
    let domino1 = Domino::new(1, 2);
    let domino2 = Domino::new(2, 3);
    let domino3 = Domino::new(3, 4);

    // Test that dominoes can be chained together
    assert_eq!(domino1.matches(domino2), Some(Orientation::LEFT));
    assert_eq!(domino2.matches(domino3), Some(Orientation::LEFT));
}

#[test]
fn test_complex_domino_matching_scenario() {
    // Integration test: More complex matching scenario
    let mut domino1 = Domino::new(6, 2);
    let domino2 = Domino::new(3, 6);

    // domino1 (6|2) doesn't match domino2 (3|6) initially
    assert_eq!(domino1.matches(domino2), None);

    // But if we rotate domino1 to (2|6), it matches domino2 (3|6) on the right
    domino1.rotate(); // Now domino1 is effectively (2|6), actual_right = 6
    assert_eq!(domino1.matches(domino2), Some(Orientation::RIGHT));

    // Test a different scenario: domino1 (rotated, actual_right=6) with a domino that has 6 on the left
    let domino4 = Domino::new(6, 9);
    assert_eq!(domino1.matches(domino4), Some(Orientation::LEFT));
}

#[test]
fn test_display_formatting() {
    // Integration test: Test display formatting works correctly
    let domino = Domino::new(3, 7);
    assert_eq!(format!("{}", domino), "(3 | 7)");

    let mut domino_rotated = Domino::new(3, 7);
    domino_rotated.rotate();
    assert_eq!(format!("{}", domino_rotated), "(7 | 3)");
}
