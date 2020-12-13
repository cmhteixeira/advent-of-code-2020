

fn find_values(input: &[i32]) -> Option<(i32, i32, i32)> {
    let mut res: Option<(i32, i32, i32)> = None;
    for (pos_i, i) in input.iter().enumerate() {
        for (pos_j, j) in input.split_at(pos_i).1.iter().enumerate() {
            for k in input.split_at(pos_j).1 {
                if i + j + k == 2020 {
                    res = Some((*i, *j, *k))
                }
            }

        }
    }
    res
}

pub fn part2(input: &[i32]) -> i32 {
    let res = find_values(input).unwrap();
    res.0 * res.1 * res.2
}

#[cfg(test)]
mod tests {
    use crate::part2::part2;

    #[test]
    fn entries_at_beginning(){
        let input = vec![1, 1, 2018, 4, 2];
        assert_eq!(part2(input.as_slice()), 2018)
    }

    #[test]
    fn entries_at_end(){
        let input = vec![1, 1, 30, 4, 2, 55, 45, 1920];
        assert_eq!(part2(input.as_slice()), 4752000)
    }

    #[test]
    fn entries_at_middle(){
        let input = vec![1, 1, 30, 4, 600, 700, 720, 55, 45, 1919];
        assert_eq!(part2(input.as_slice()), 302400000)
    }

    #[test]
    fn entries_not_in_sequence(){
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(part2(input.as_slice()), 241861950)
    }
}