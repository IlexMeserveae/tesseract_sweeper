use crate::minesweeper::coordinate;
use super::*;

fn debug_minefield() -> Minefield { Minefield::new(coordinate::coordinate(5, 5, 5, 5), 4).unwrap() }

#[test]
fn iter_ordinate_test() {
    let field = debug_minefield();
    let result = field.iter_ordinate(coordinate::coordinate(1, 1, 3, 1), 2, Ordinate::Z);

    assert_eq!(result, 1..6);
}

#[test]
fn get_neighbours_test_2() {
    let field = debug_minefield();
    let result = field.get_neighbours(coordinate::coordinate(1, 2, 5, 4), 1);
    // println!("{:?}", result);

    assert_eq!(result.len(), 36 - 1);
    assert!(!result.contains(&coordinate::coordinate(1, 2, 5, 4)));
    assert!(result.contains(&coordinate::coordinate(2, 3, 5, 5)));
    assert!(result.contains(&coordinate::coordinate(1, 1, 4, 3)));
}

// #[test]
// fn cascade_test() {
//     let size = coordinate::coordinate(5, 5, 1, 1);
//     let tiles = vec![
//         0, 0, 0, 1, 0,
//         0, 0, 1, 0, 0,
//         0, 0, 0, 0, 0,
//         0, 0, 0, 0, 0,
//         0, 1, 0, 0, 0,
//     ];
//     let total_mines = tiles.iter().filter(|&&t| t == 1).count() as i16;
//     let tiles: Vec<_> = tiles.into_iter().enumerate().map(|(i, mine)|
//         Tile::new(mine == 1, Minefield::neighbour_no(size, i))).collect();
//     let flagged_mines = 0;
//     let blanks_remaining = tiles.len() as i16;
//     let delta = true;
//     let mut mf = Minefield { size, tiles, total_mines, flagged_mines, blanks_remaining, delta };
// }