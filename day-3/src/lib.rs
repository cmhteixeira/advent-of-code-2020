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

pub fn number_trees(map: &str) -> u16 {
    let lines: Vec<TreeLine> = map.lines().map(|line| TreeLine::new(line)).collect();
    fn internal(
        remaining_lines: &[TreeLine],
        num_trees: u16,
        current_pos: u16,
    ) -> u16 {
        if remaining_lines.is_empty() {
            num_trees
        } else {
            let (current_line, remaining) = remaining_lines.split_at(1);
            match current_line.get(0).unwrap().tree_at(current_pos) {
                true => internal(remaining, num_trees + 1, current_pos + 3),
                false => internal(remaining, num_trees, current_pos + 3)
            }
        }
    }
    internal(lines.as_slice(), 0, 0)
}

#[cfg(test)]
mod tests {
    use crate::number_trees;
    use indoc::indoc;

    #[test]
    fn these_2_lines_work() {
        let input: &str = indoc! {
        "..##.........##.........##.........##.........##.........##.......
         #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#.."
         };

        assert_eq!(number_trees(input), 0)
    }

    #[test]
    fn only_diagonal_positions_count() {
        let input: &str = indoc! {
        "..##......
         #.###...#."
         };

        assert_eq!(number_trees(input), 1)
    }

    #[test]
    fn lines_can_shift() {
        let input: &str = indoc! {
        "..#..#.
         #...#..
         ##..#.#
         ....#.."
         };

        assert_eq!(number_trees(input), 1)
    }

    #[test]
    fn we_can_hit_all_trees() {
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

        assert_eq!(number_trees(input), 8)
    }

    #[test]
    fn it_works() {
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

        assert_eq!(number_trees(input), 7)
    }
}
