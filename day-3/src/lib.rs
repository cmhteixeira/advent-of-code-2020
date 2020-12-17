pub struct Slope {
    right: u8,
    down: u8
}

impl Slope {
    pub fn new(right: u8, down: u8) -> Slope {
        Slope {
           right,
           down
        }
    }
}

struct TreeLine<'y> {
    line: &'y str,
    origin_size: u16,
}

impl TreeLine<'_> {
    fn new(line: &str) -> TreeLine {
        TreeLine {
            line,
            origin_size: line.chars().count() as u16,
        }
    }

    fn tree_at(&self, nth: u16) -> bool {
        if nth > self.origin_size - 1{
            self.tree_at(nth - self.origin_size)
        } else {
            self.line
                .chars()
                .nth(nth as usize)
                .map(|char| char == '#')
                .unwrap()
        }
    }
}

pub fn number_trees(map: &str, slope: Slope) -> u16 {
    let lines = map.lines().map(|line| TreeLine::new(line));
    fn internal(
        remaining_lines: &[TreeLine],
        num_trees: u16,
        current_pos: u16,
        right: u16
    ) -> u16 {
        if remaining_lines.is_empty() {
            num_trees
        } else {
            let (current_line, remaining) = remaining_lines.split_at(1);
            match current_line.get(0).unwrap().tree_at(current_pos) {
                true => internal(remaining, num_trees + 1, current_pos + right, right),
                false => internal(remaining, num_trees, current_pos + right, right)
            }
        }
    }
    let lines_that_matter = lines.enumerate().filter(|elem|{
        (elem.0 + 1) % (slope.down as usize) == 0 && elem.0 + 1 != 1
    }).map(|p| p.1);

    let lines_that_matter: Vec<TreeLine> = lines_that_matter.collect();
    internal(lines_that_matter.as_slice(), 0, slope.right as u16, slope.right as u16)
}

#[cfg(test)]
mod tests {
    use crate::{number_trees, Slope};
    use indoc::indoc;

    #[test]
    fn these_2_lines_work() {
        let slope: Slope = Slope::new(3, 1);

        let input: &str = indoc! {
        "..##.........##.........##.........##.........##.........##.......
         #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#.."
         };

        assert_eq!(number_trees(input, slope), 0)
    }

    #[test]
    fn only_diagonal_positions_count() {
        let slope: Slope = Slope::new(3, 1);
        let input: &str = indoc! {
        "..##......
         #.###...#."
         };

        assert_eq!(number_trees(input, slope), 1)
    }

    #[test]
    fn lines_can_shift() {
        let slope: Slope = Slope::new(3, 1);
        let input: &str = indoc! {
        "..#..#.
         #...#..
         ##..#.#
         ....#.."
         };

        assert_eq!(number_trees(input, slope), 1)
    }

    #[test]
    fn we_can_hit_all_trees() {
        let slope: Slope = Slope::new(3, 1);
        let input: &str = indoc! {
        ".####
         #####
         #####
         #####
         #####
         #####
         #####
         #####
         #####"
         };

        assert_eq!(number_trees(input, slope), 8)
    }

    #[test]
    fn it_works() {
        let slope: Slope = Slope::new(3, 1);
        let input: &str = indoc! {
        "..##.........##.........##.........##.........##.........##.......
         #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
         .#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
         ..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
         .#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
         ..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....
         .#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
         .#........#.#........#.#........#.#........#.#........#.#........#
         #.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
         #...##....##...##....##...##....##...##....##...##....##...##....#
         .#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#"
         };

        assert_eq!(number_trees(input, slope), 7)
    }

    #[test]
    fn two_jumps_down_works() {
        let slope: Slope = Slope::new(1, 2);
        let input: &str = indoc! {
        "..##.........##.........##.........##.........##.........##.......
         #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
         .#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
         ..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
         .#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
         ..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....
         .#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
         .#........#.#........#.#........#.#........#.#........#.#........#
         #.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
         #...##....##...##....##...##....##...##....##...##....##...##....#
         .#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#"
         };

        assert_eq!(number_trees(input, slope), 2)
    }
}
