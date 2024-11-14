/*use super::*;
use fs_core::stations::STATION_TYPES;

/// Helper function to easily make 2D vector character maps from string, used to
/// simplify test case code
pub fn make_char_map(src: &str) -> Vec<Vec<char>> {
    let mut char_map: Vec<Vec<char>> = Vec::new();
    for line in src.split('\n') {
        char_map.push(line.chars().collect::<Vec<char>>());
    }
    return char_map;
}

#[test]
fn test_get_next_char() {
    let char_map = vec![vec!['a', 'b'], vec![], vec!['c'], vec!['d', 'e']];
    let mut pos = SourcePos::zero();
    assert_eq!(get_next_char(&mut pos, &char_map).unwrap(), 'b');
    assert_eq!(get_next_char(&mut pos, &char_map).unwrap(), 'c');
    assert_eq!(get_next_char(&mut pos, &char_map).unwrap(), 'd');
    assert_eq!(get_next_char(&mut pos, &char_map).unwrap(), 'e');
    assert!(get_next_char(&mut pos, &char_map).is_none());
}

#[test]
fn test_parse_stations() {
    let char_map = make_char_map("[start] [print]\n  [exit]");
    let (stations, _) = parse_stations(&char_map, &STATION_TYPES).unwrap();
    assert_eq!(stations[0].logic.id, "start");
    assert_eq!(stations[0].loc, SourceSpan::new(SourcePos::zero(), 7));
    assert_eq!(stations[1].logic.id, "print");
    assert_eq!(stations[1].loc, SourceSpan::new(SourcePos::new(0, 8), 7));
    assert_eq!(stations[2].logic.id, "exit");
    assert_eq!(stations[2].loc, SourceSpan::new(SourcePos::new(1, 2), 6));
}

#[test]
fn test_parse_stations_incomplete() {
    let char_map = make_char_map("[start] [print]\n  [ex");
    assert!(parse_stations(&char_map, &STATION_TYPES).is_err());
}

#[test]
fn test_parse_stations_assign() {
    let char_map = make_char_map("[start] {} [exit]");
    let (stations, _) = parse_stations(&char_map, &STATION_TYPES).unwrap();
    assert_eq!(stations[0].logic.id, "start");
    assert_eq!(stations[0].loc, SourceSpan::new(SourcePos::zero(), 7));
    assert_eq!(stations[1].logic.id, "assign");
    assert_eq!(stations[1].loc, SourceSpan::new(SourcePos::new(0, 8), 2));
    assert_eq!(stations[2].logic.id, "exit");
    assert_eq!(stations[2].loc, SourceSpan::new(SourcePos::new(0, 11), 6));
}

#[test]
fn test_parse_stations_joint() {
    let char_map = make_char_map("[start] [] [exit]");
    let (stations, _) = parse_stations(&char_map, &STATION_TYPES).unwrap();
    assert_eq!(stations[0].logic.id, "start");
    assert_eq!(stations[0].loc, SourceSpan::new(SourcePos::zero(), 7));
    assert_eq!(stations[1].logic.id, "joint");
    assert_eq!(stations[1].loc, SourceSpan::new(SourcePos::new(0, 8), 2));
    assert_eq!(stations[2].logic.id, "exit");
    assert_eq!(stations[2].loc, SourceSpan::new(SourcePos::new(0, 11), 6));
}

#[test]
fn test_parse_assign_literal_empty() {
    assert_eq!(
        parse_assign_literal(&"".to_owned(), SourceSpan::zero())
            .ok()
            .unwrap(),
        Pallet::Empty
    );
}

#[test]
fn test_parse_assign_literal_string() {
    assert_eq!(
        parse_assign_literal(&"\"test\ttest \\t 123\"".to_owned(), SourceSpan::zero())
            .ok()
            .unwrap(),
        Pallet::String("test\ttest \\t 123".to_owned())
    );
    assert!(parse_assign_literal(&"\"test".to_owned(), SourceSpan::zero()).is_err());
}

#[test]
fn test_parse_assign_literal_char() {
    assert_eq!(
        parse_assign_literal(&"\'a\'".to_owned(), SourceSpan::zero())
            .ok()
            .unwrap(),
        Pallet::Char('a')
    );
    assert_eq!(
        parse_assign_literal(&"\'\n\'".to_owned(), SourceSpan::zero())
            .ok()
            .unwrap(),
        Pallet::Char('\n')
    );
    assert!(parse_assign_literal(&"\'ab\'".to_owned(), SourceSpan::zero()).is_err());
    assert!(parse_assign_literal(&"\'a".to_owned(), SourceSpan::zero()).is_err());
    assert!(parse_assign_literal(&"\'ab".to_owned(), SourceSpan::zero()).is_err());
}

#[test]
fn test_parse_assign_literal_bool() {
    assert_eq!(
        parse_assign_literal(&"true".to_owned(), SourceSpan::zero())
            .ok()
            .unwrap(),
        Pallet::Bool(true)
    );
    assert_eq!(
        parse_assign_literal(&"false".to_owned(), SourceSpan::zero())
            .ok()
            .unwrap(),
        Pallet::Bool(false)
    );
}

#[test]
fn test_parse_assign_literal_integer() {
    assert_eq!(
        parse_assign_literal(&"123".to_owned(), SourceSpan::zero())
            .ok()
            .unwrap(),
        Pallet::Int(123)
    );
    assert_eq!(
        parse_assign_literal(&"123_456_789".to_owned(), SourceSpan::zero())
            .ok()
            .unwrap(),
        Pallet::Int(123456789)
    );
    assert!(parse_assign_literal(&"123_4a56".to_owned(), SourceSpan::zero()).is_err());
}

#[test]
fn test_parse_assign_literal_float() {
    assert_eq!(
        parse_assign_literal(&"123.0".to_owned(), SourceSpan::zero())
            .ok()
            .unwrap(),
        Pallet::Float(123.0)
    );
    assert_eq!(
        parse_assign_literal(&"0.123456".to_owned(), SourceSpan::zero())
            .ok()
            .unwrap(),
        Pallet::Float(0.123456)
    );
    assert_eq!(
        parse_assign_literal(&"123_456_789f".to_owned(), SourceSpan::zero())
            .ok()
            .unwrap(),
        Pallet::Float(123456789.0)
    );
    assert!(parse_assign_literal(&"123f1".to_owned(), SourceSpan::zero()).is_err());
    assert!(parse_assign_literal(&"1.23.4".to_owned(), SourceSpan::zero()).is_err());
    assert!(parse_assign_literal(&"12a3.0".to_owned(), SourceSpan::zero()).is_err());
}
*/
