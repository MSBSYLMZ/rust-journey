fn main() {
    let lines = &["  +-+", "  | |", "+-+-+", "| | |", "+-+-+"];
    count(lines);
}

pub fn count(lines: &[&str]) -> u32 {
    let mut finder = Finder {
        lines,
        count: 0,
        counter_count: 0,
    };
    finder.count_squares((0, 0), (0, 0));
    println!("{}", finder.count);
    finder.count
}

struct Finder<'a> {
    lines: &'a [&'a str],
    count: u32,
    counter_count: u32,
}

impl<'a> Finder<'a> {
    fn count_squares(&mut self, base: (usize, usize), starting_point: (usize, usize)) -> u32 {
        self.follow_right(base, starting_point);
        return self.count;
    }

    fn follow_right(&mut self, base: (usize, usize), starting_point: (usize, usize)) {
        println!("right  starting point: {} {} {}      | count: {}      |  base: {} {}", self.lines.get(starting_point.0).unwrap().as_bytes().get(starting_point.1).unwrap().to_owned() as char, starting_point.0, starting_point.1, self.count, base.0, base.1);
        let y_axis = self.lines.get(starting_point.0);
        let is_next_x_exists = if let Some(V) = y_axis {
            V.as_bytes().get(starting_point.1 + 1).is_some()
        } else {
            false
        };

        let current_char = self.lines[starting_point.0].as_bytes()[starting_point.1] as char;

        if !is_next_x_exists {
            if self.lines.get(base.0 + 1).is_none() {
                return;
            } else if base != starting_point {
                return;
            }
            let base = (base.0 + 1, 0);
            self.follow_right(base, base);
            return;
        }

        if current_char != '+' && base == starting_point {
            let base = (base.0, base.1 + 1);
            self.follow_right(base, base);
            return;
        }
        let next_char = self.lines[starting_point.0].as_bytes()[starting_point.1 + 1] as char;

        if next_char != '+' && next_char != '-' {
            if self.counter_count < 2 {
                let base = (base.0, base.1+1);
                self.follow_right(base, base)
            }
            self.counter_count -= 1;
            return;
        } else {
            let starting_point = (starting_point.0, starting_point.1 + 1);
            if next_char == '+' {
                println!();
                self.follow_bottom(base, starting_point);
            }
            self.follow_right(base, starting_point);
        }
    }

    fn follow_bottom(&mut self, base: (usize, usize), starting_point: (usize, usize)) {
        println!("bottom starting point: {} {} {}      | count: {}      |  base: {} {}", self.lines.get(starting_point.0).unwrap().as_bytes().get(starting_point.1).unwrap().to_owned() as char, starting_point.0, starting_point.1, self.count, base.0, base.1);
        let y_axis = self.lines.get(starting_point.0 + 1);
        let is_next_x_exists: bool = if let Some(V) = y_axis {
            V.as_bytes().get(starting_point.1).is_some()
        } else {
            false
        };

        if !is_next_x_exists {
            let base = (base.0, base.1 + 1);
            self.follow_right(base, base);
        } else {
            let next_char =
                self.lines[starting_point.0 + 1].as_bytes()[starting_point.1] as char;

            if next_char != '|' && next_char != '+' {
                if self.counter_count < 2 {
                    let base = (base.0, base.1+1);
                    self.follow_right(base, base)
                }
                self.counter_count -= 1;
                return;
            } else {
                let starting_point = (starting_point.0 + 1, starting_point.1);
                if next_char == '+' {
                    self.follow_left(base, starting_point);
                }
                self.follow_bottom(base, starting_point);
            }
        }
    }

    fn follow_left(&mut self, base: (usize, usize), starting_point: (usize, usize)) {
        println!("left   starting point: {} {} {}      | count: {}      |  base: {} {}", self.lines.get(starting_point.0).unwrap().as_bytes().get(starting_point.1).unwrap().to_owned() as char, starting_point.0, starting_point.1, self.count, base.0, base.1);

        let y_axis = self.lines.get(starting_point.0);
        let is_next_x_exists: bool = if let Some(V) = y_axis {
            if starting_point.1 as i32 - 1 < 0  {
                false
            } else {
                V.as_bytes().get(starting_point.1 - 1).is_some()
            }
        } else {
            false
        };

        if !is_next_x_exists {
            self.follow_right((base.0, base.1 + 1), (base.0, base.1 + 1));
        } else {
            let next_char = self.lines[starting_point.0].as_bytes()[starting_point.1 - 1] as char;
            if next_char != '+' && next_char != '-' {
                if self.counter_count < 2 {
                    let base = (base.0, base.1+1);
                    self.follow_right(base, base)
                }
                self.counter_count -= 1;
                return;
            } else {
                if next_char == '+' {
                    self.follow_top(base, (starting_point.0, starting_point.1 - 1));
                }
                self.follow_left(base, (starting_point.0, starting_point.1 - 1));
            }
        }
    }

    fn follow_top(&mut self, base: (usize, usize), starting_point: (usize, usize)) {
        self.counter_count += 1;
        println!("top    starting point: {} {} {}      | count: {}      |  base: {} {}", self.lines.get(starting_point.0).unwrap().as_bytes().get(starting_point.1).unwrap().to_owned() as char, starting_point.0, starting_point.1, self.count, base.0, base.1);
        if starting_point.0 as i32 - 1 < 0 {
            self.follow_right((base.0, base.1 + 1), (base.0, base.1 + 1));
            return
        }
        let y_axis = self.lines.get(starting_point.0 - 1);
        let is_next_x_exists: bool = if let Some(V) = y_axis {
            V.as_bytes().get(starting_point.1).is_some()
        } else {
            false
        };

        if  !is_next_x_exists {
            self.follow_right((base.0, base.1 + 1), (base.0, base.1 + 1));
        } else {
            let next_char = self.lines[starting_point.0 - 1].as_bytes()[starting_point.1] as char;
            if next_char != '+' && next_char != '|' {
                if self.counter_count < 2 {
                    let base = (base.0, base.1+1);
                    self.follow_right(base, base)
                }
                self.counter_count -= 1;
                return;
            } else {
                if next_char == '+' {
                    if (starting_point.0 - 1, starting_point.1) == base {
                        self.count += 1;
                        return;
                    }
                }
                self.follow_top(base, (starting_point.0 - 1, starting_point.1));
            }
        }
        self.counter_count += 1;
    }
}
