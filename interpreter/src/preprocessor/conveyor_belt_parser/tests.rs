/*use belt_follower::follow_belt;

use super::*;

#[test]
fn test_get_neighbors() {
    let map = vec![
        vec![' ', ' ', ' ', ' '],
        vec![' ', '[', ']', ' '],
        vec![' ', ' ', ' ', ' '],
    ];
    let station = Station::new(
        "",
        SourceSpan::new(SourcePos::new(1, 1), 2),
        StationModifiers::default(),
        &STATION_TYPES,
    )
    .unwrap();
    assert_eq!(
        get_neighbors(&map, &station),
        vec![
            (SourcePos::new(0, 1), Direction::NORTH),
            (SourcePos::new(0, 2), Direction::NORTH),
            (SourcePos::new(1, 3), Direction::EAST),
            (SourcePos::new(2, 2), Direction::SOUTH),
            (SourcePos::new(2, 1), Direction::SOUTH),
            (SourcePos::new(1, 0), Direction::WEST),
        ]
    )
}

#[test]
fn test_get_neighbors_on_border() {
    let map = vec![vec![' ', '[', ']', ' ']];
    let station = Station::new(
        "",
        SourceSpan::new(SourcePos::new(0, 1), 2),
        StationModifiers::default(),
        &STATION_TYPES,
    )
    .unwrap();
    assert_eq!(
        get_neighbors(&map, &station),
        vec![
            (SourcePos::new(0, 3), Direction::EAST),
            (SourcePos::new(0, 0), Direction::WEST),
        ]
    );
    let map = vec![vec![' ', ' '], vec!['[', ']'], vec![' ', ' ']];
    let station = Station::new(
        "",
        SourceSpan::new(SourcePos::new(1, 0), 2),
        StationModifiers::default(),
        &STATION_TYPES,
    )
    .unwrap();
    assert_eq!(
        get_neighbors(&map, &station),
        vec![
            (SourcePos::new(0, 0), Direction::NORTH),
            (SourcePos::new(0, 1), Direction::NORTH),
            (SourcePos::new(2, 1), Direction::SOUTH),
            (SourcePos::new(2, 0), Direction::SOUTH),
        ]
    );
    let map = vec![vec!['[', ']']];
    let station = Station::new(
        "",
        SourceSpan::new(SourcePos::zero(), 2),
        StationModifiers::default(),
        &STATION_TYPES,
    )
    .unwrap();
    assert_eq!(get_neighbors(&map, &station), vec![])
}

#[test]
fn test_get_neighbors_with_modifiers() {
    let map = vec![
        vec![' ', ' ', ' ', ' '],
        vec![' ', '[', ']', ' '],
        vec![' ', ' ', ' ', ' '],
    ];
    let mut station = Station::new(
        "",
        SourceSpan::new(SourcePos::new(1, 1), 2),
        StationModifiers::default().with_priority(Direction::EAST),
        &STATION_TYPES,
    )
    .unwrap();
    assert_eq!(
        get_neighbors(&map, &station),
        vec![
            (SourcePos::new(1, 3), Direction::EAST),
            (SourcePos::new(2, 2), Direction::SOUTH),
            (SourcePos::new(2, 1), Direction::SOUTH),
            (SourcePos::new(1, 0), Direction::WEST),
            (SourcePos::new(0, 1), Direction::NORTH),
            (SourcePos::new(0, 2), Direction::NORTH),
        ]
    );
    station.modifiers = station.modifiers.with_priority(Direction::SOUTH);
    assert_eq!(
        get_neighbors(&map, &station),
        vec![
            (SourcePos::new(2, 2), Direction::SOUTH),
            (SourcePos::new(2, 1), Direction::SOUTH),
            (SourcePos::new(1, 0), Direction::WEST),
            (SourcePos::new(0, 1), Direction::NORTH),
            (SourcePos::new(0, 2), Direction::NORTH),
            (SourcePos::new(1, 3), Direction::EAST),
        ]
    );
    station.modifiers = station.modifiers.with_priority(Direction::WEST);
    assert_eq!(
        get_neighbors(&map, &station),
        vec![
            (SourcePos::new(1, 0), Direction::WEST),
            (SourcePos::new(0, 1), Direction::NORTH),
            (SourcePos::new(0, 2), Direction::NORTH),
            (SourcePos::new(1, 3), Direction::EAST),
            (SourcePos::new(2, 2), Direction::SOUTH),
            (SourcePos::new(2, 1), Direction::SOUTH),
        ]
    );
    station.modifiers = station.modifiers.reverse().with_priority(Direction::NORTH);
    assert_eq!(
        get_neighbors(&map, &station),
        vec![
            (SourcePos::new(0, 2), Direction::NORTH),
            (SourcePos::new(0, 1), Direction::NORTH),
            (SourcePos::new(1, 0), Direction::WEST),
            (SourcePos::new(2, 1), Direction::SOUTH),
            (SourcePos::new(2, 2), Direction::SOUTH),
            (SourcePos::new(1, 3), Direction::EAST),
        ]
    );
    station.modifiers = station.modifiers.with_priority(Direction::EAST);
    assert_eq!(
        get_neighbors(&map, &station),
        vec![
            (SourcePos::new(1, 3), Direction::EAST),
            (SourcePos::new(0, 2), Direction::NORTH),
            (SourcePos::new(0, 1), Direction::NORTH),
            (SourcePos::new(1, 0), Direction::WEST),
            (SourcePos::new(2, 1), Direction::SOUTH),
            (SourcePos::new(2, 2), Direction::SOUTH),
        ]
    );
    station.modifiers = station.modifiers.with_priority(Direction::SOUTH);
    assert_eq!(
        get_neighbors(&map, &station),
        vec![
            (SourcePos::new(2, 1), Direction::SOUTH),
            (SourcePos::new(2, 2), Direction::SOUTH),
            (SourcePos::new(1, 3), Direction::EAST),
            (SourcePos::new(0, 2), Direction::NORTH),
            (SourcePos::new(0, 1), Direction::NORTH),
            (SourcePos::new(1, 0), Direction::WEST),
        ]
    );
    station.modifiers = station.modifiers.with_priority(Direction::WEST);
    assert_eq!(
        get_neighbors(&map, &station),
        vec![
            (SourcePos::new(1, 0), Direction::WEST),
            (SourcePos::new(2, 1), Direction::SOUTH),
            (SourcePos::new(2, 2), Direction::SOUTH),
            (SourcePos::new(1, 3), Direction::EAST),
            (SourcePos::new(0, 2), Direction::NORTH),
            (SourcePos::new(0, 1), Direction::NORTH),
        ]
    );
}

/// helper function to create a 2d boolean map with the same dimensions as the
/// provided char map
fn make_visited_map(char_map: &Vec<Vec<char>>) -> Vec<Vec<bool>> {
    let mut visited_map: Vec<Vec<bool>> = Vec::new();
    for line in char_map {
        visited_map.push(line.iter().map(|_| false).collect());
    }
    return visited_map;
}

#[test]
fn test_follow_belt() {
    let map = vec![
        vec!['─', '─', '─', '┐'],
        vec![' ', '┌', '┐', '│'],
        vec!['╚', '┘', '└', '┘'],
    ];
    let mut visited_map = make_visited_map(&map);
    assert_eq!(
        follow_belt(&map, &mut visited_map, (SourcePos::zero(), Direction::EAST))
            .ok()
            .unwrap(),
        Some(SourcePos::new(1, 0))
    );
    assert_eq!(
        visited_map,
        vec![
            vec![true, true, true, true],
            vec![false, true, true, true],
            vec![true, true, true, true],
        ]
    );
    assert_eq!(
        follow_belt(
            &map,
            &mut visited_map,
            (SourcePos::new(0, 3), Direction::EAST)
        )
        .ok()
        .unwrap(),
        Some(SourcePos::new(1, 0))
    );
}

#[test]
fn test_follow_belt_dangling() {
    let map = vec![vec!['─', '┐'], vec![' ', '─']];
    let mut visited_map = make_visited_map(&map);
    assert!(follow_belt(&map, &mut visited_map, (SourcePos::zero(), Direction::EAST)).is_err());
    assert_eq!(visited_map, make_visited_map(&map));
    assert!(follow_belt(
        &map,
        &mut visited_map,
        (SourcePos::new(1, 1), Direction::WEST)
    )
    .is_err());
    assert_eq!(visited_map, make_visited_map(&map));
}

#[test]
fn test_follow_belt_out_of_bounds() {
    let map = vec![vec!['─', '┐']];
    let mut visited_map = vec![vec![false, false]];
    assert!(follow_belt(&map, &mut visited_map, (SourcePos::zero(), Direction::EAST)).is_err());
    assert_eq!(visited_map, vec![vec![false, false]]);
    assert!(follow_belt(
        &map,
        &mut visited_map,
        (SourcePos::new(0, 1), Direction::EAST)
    )
    .is_err());
    assert_eq!(visited_map, vec![vec![false, false]]);
}

#[test]
fn test_follow_belt_none() {
    let map = vec![vec![' ']];
    let mut visited_map = vec![vec![false]];
    assert!(follow_belt(
        &map,
        &mut visited_map,
        (SourcePos::zero(), Direction::NORTH)
    )
    .ok()
    .unwrap()
    .is_none());
    assert_eq!(visited_map, make_visited_map(&map));
}
*/
