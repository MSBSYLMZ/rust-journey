fn main() {
    let lines = &["  +-+", "  | |", "+-+-+", "| | |", "+-+-+"];
    count(lines);
}

enum Direction {
    Left,
    Right,
    Bottom,
    Top,
}

pub fn count(lines: &[&str]) -> u32 {
    let result = follow_line(Direction::Right, (0, 0), lines, (0, 0));
    println!("{result}");
    result
}

fn follow_line(
    direction: Direction,
    base: (usize, usize),
    lines: &[&str],
    starting_point: (usize, usize),
) -> u32 {
    let mut rectangle_count: u32 = 0;
    println!(
        "starting point : {}, {} , {}",
        lines
            .get(starting_point.0)
            .unwrap()
            .as_bytes()
            .get(starting_point.1)
            .unwrap()
            .to_owned() as char,
        starting_point.0,
        starting_point.1
    );
    if starting_point.0 > 3 {
        return 0;
    }
    match direction {
        Direction::Right => {
            let y_axis = lines.get(starting_point.0);
            let is_x_axis_exists = if let Some(V) = y_axis {
                V.as_bytes().get(starting_point.1 + 1).is_some()
            } else {
                false
            };

            if is_x_axis_exists {
                if lines.get(base.0 + 1).is_none() {
                    return rectangle_count;
                }
                let base = (base.0 + 1, 0);
                rectangle_count += follow_line(Direction::Right, base, lines, base);
            } else {
                let current_char = lines[starting_point.0].as_bytes()[starting_point.1 + 1] as char;
                if current_char == '+' {
                    if starting_point == base {
                        let base = (base.0, base.1 + 1);
                        rectangle_count += follow_line(Direction::Right, base, lines, base);
                    } else {
                        rectangle_count += follow_line(
                            Direction::Bottom,
                            base,
                            lines,
                            (starting_point.0, starting_point.1 + 1),
                        );
                        rectangle_count += follow_line(
                            Direction::Right,
                            base,
                            lines,
                            (starting_point.0, starting_point.1 + 1),
                        );
                    }
                } else if current_char == '-' {
                    rectangle_count += follow_line(
                        Direction::Right,
                        base,
                        lines,
                        (starting_point.0, starting_point.1 + 1),
                    );
                } else {
                    let base = (starting_point.0, starting_point.1 + 1);
                    rectangle_count += follow_line(Direction::Right, base, lines, base);
                }
            }
        }

        Direction::Bottom => {
            let y_axis = lines.get(starting_point.0 + 1);
            let is_x_axis_exists: bool = if let Some(v) = y_axis {
                v.as_bytes().get(starting_point.1).is_some()
            } else {
                false
            };

            if is_x_axis_exists {
                let base = (base.0, base.1 + 1);
                rectangle_count += follow_line(Direction::Right, base, lines, base);
            } else {
                let current_char = lines[starting_point.0 + 1].as_bytes()[starting_point.1] as char;

                if current_char != '|' || current_char != '+' {
                    let base = (base.0 + 1, base.1);
                    rectangle_count += follow_line(Direction::Right, base, lines, base);
                } else {
                    let starting_point = (starting_point.0 + 1, starting_point.1);
                    if current_char == '+' {
                        rectangle_count +=
                            follow_line(Direction::Left, base, lines, starting_point);
                    }
                    rectangle_count += follow_line(Direction::Bottom, base, lines, starting_point);
                }
            }
        }

        Direction::Left => {
            if starting_point.1 as i32 - 1 < 0
                || lines.get(starting_point.0).is_none()
                || lines
                    .get(starting_point.0)
                    .unwrap()
                    .as_bytes()
                    .get(starting_point.1 - 1)
                    .is_none()
            {
                rectangle_count += follow_line(
                    Direction::Right,
                    (base.0, base.1 + 1),
                    lines,
                    (base.0, base.1 + 1),
                );
            } else {
                if lines[starting_point.0].as_bytes()[starting_point.1 - 1] as char == '+' {
                    rectangle_count += follow_line(
                        Direction::Top,
                        base,
                        lines,
                        (starting_point.0, starting_point.1 - 1),
                    );

                    rectangle_count += follow_line(
                        Direction::Left,
                        base,
                        lines,
                        (starting_point.0, starting_point.1 - 1),
                    );
                } else if lines[starting_point.0].as_bytes()[starting_point.1 - 1] as char == '-' {
                    rectangle_count += follow_line(
                        Direction::Left,
                        base,
                        lines,
                        (starting_point.0, starting_point.1 - 1),
                    );
                } else {
                    let base = (base.0 + 1, base.1);
                    rectangle_count += follow_line(Direction::Right, base, lines, base);
                }
            }
        }
        Direction::Top => {
            if starting_point.0 as i32 - 1 < 0
                || lines.get(starting_point.0 - 1).is_none()
                || lines
                    .get(starting_point.0 - 1)
                    .unwrap()
                    .as_bytes()
                    .get(starting_point.1)
                    .is_none()
            {
                rectangle_count += follow_line(
                    Direction::Right,
                    (base.0, base.1 + 1),
                    lines,
                    (base.0, base.1 + 1),
                );
            } else {
                if lines[starting_point.0 - 1].as_bytes()[starting_point.1] as char == '+' {
                    if (starting_point.0 - 1, starting_point.1) == base {
                        rectangle_count += 1;
                    }
                    {
                        let base = (base.0, base.1 + 1);
                        rectangle_count += follow_line(Direction::Right, base, lines, base);
                    }
                    rectangle_count += follow_line(
                        Direction::Top,
                        base,
                        lines,
                        (starting_point.0 - 1, starting_point.1),
                    );
                } else if lines[starting_point.0 - 1].as_bytes()[starting_point.1] as char == '|' {
                    println!("We are venom");
                    rectangle_count += follow_line(
                        Direction::Top,
                        base,
                        lines,
                        (starting_point.0 - 1, starting_point.1),
                    );
                } else {
                    let base = (base.0 + 1, base.1);
                    rectangle_count += follow_line(Direction::Right, base, lines, base);
                }
            }
        }
    }
    println!("{rectangle_count}");
    rectangle_count
}
