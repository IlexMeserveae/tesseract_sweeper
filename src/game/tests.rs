use super::*;

fn debug_minefield() -> Minefield { Minefield::new(coordinate(5, 5, 5, 5), 4).unwrap() }

#[test]
fn iter_ordinate_test() {
    let field = debug_minefield();
    let result = field.iter_ordinate(coordinate(1, 1, 3, 1), 2, Ordinate::Z);

    assert_eq!(result, 1..6);
}

#[test]
fn get_neighbours_test_2() {
    let field = debug_minefield();
    let result = field.get_neighbours(coordinate(1, 2, 5, 4), 1);
    // println!("{:?}", result);

    assert_eq!(result.len(), 36 - 1);
    assert!(!result.contains(&coordinate(1, 2, 5, 4)));
    assert!(result.contains(&coordinate(2, 3, 5, 5)));
    assert!(result.contains(&coordinate(1, 1, 4, 3)));
}