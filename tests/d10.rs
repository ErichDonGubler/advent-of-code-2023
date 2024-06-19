use insta::assert_debug_snapshot;

#[derive(Debug)]
enum Tile {
    Ground,
    VerticalPipe,
    HorizontalPipe,
    RightUpBend,
    LeftUpBend,
    LeftDownBend,
    RightDownBend,
    StartPosition,
}

impl Tile {
    pub fn from_ascii(c: char) -> Result<Self, ()> {
        Ok(match c {
            '|' => Self::VerticalPipe,
            '-' => Self::HorizontalPipe,
            'L' => Self::RightUpBend,
            'J' => Self::LeftUpBend,
            '7' => Self::LeftDownBend,
            'F' => Self::RightDownBend,
            '.' => Self::Ground,
            'S' => Self::StartPosition,
            _ => return Err(()),
        })
    }
}

#[derive(Debug)]
struct Map {
    parsed: Vec<Tile>,
    dimensions: (u32, u32),
}

impl Map {
    pub fn parse(s: &str) -> Self {
        let mut lines = s.lines().map(str::trim).enumerate();
        let (_zero, first_line) = lines.next().unwrap();
        let expected_row_char_count = first_line.chars().count().try_into().unwrap();
        let width_validated_lines =
            Some(first_line)
                .into_iter()
                .chain(lines.map(|(line_idx, line)| {
                    let this_line_char_count = line.chars().count();
                    assert_eq!(
                        this_line_char_count, expected_row_char_count as usize,
                        "line {} contains {} characters, expected {} based on first row",
                        line_idx, this_line_char_count, expected_row_char_count,
                    );
                    line
                }));
        let parsed = width_validated_lines
            .flat_map(|line| line.chars().map(|c| Tile::from_ascii(c).unwrap()))
            .collect::<Vec<_>>();
        let height = parsed.len() / usize::try_from(expected_row_char_count).unwrap();
        Self {
            parsed,
            dimensions: (expected_row_char_count, height.try_into().unwrap()),
        }
    }

    pub fn analyze_farthest_minmax_point_in_loop(&self) {}
}

const P1_NO_START_EXAMPLE: &str = "\
.....
.F-7.
.|.|.
.L-J.
.....
";

const P1_FULL_EXAMPLE: &str = "\
.....
.S-7.
.|.|.
.L-J.
.....
";

#[test]
fn p1_example() {
    let map = Map::parse(P1_NO_START_EXAMPLE);
    assert_debug_snapshot!(map);

    let map = Map::parse(P1_FULL_EXAMPLE);
    assert_debug_snapshot!(map);
}
