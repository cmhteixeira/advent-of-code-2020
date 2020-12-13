use std::iter::Iterator;


fn find_values(input: &[i32]) -> Option<(i32, i32)> {
    let mut res: Option<(i32, i32)> = None;
    for (pos, i) in input.iter().enumerate() {
        for j in input.split_at(pos).1 {
            if i + j == 2020 {
                res = Some((*i, *j))
            }
        }
    }
    res
}

pub fn part1(input: &[i32]) -> i32 {
    let res = find_values(input).unwrap();
    res.0 * res.1
}


#[cfg(test)]
mod tests {
    use crate::part1::part1;

    #[test]
    fn entries_at_beginning(){
        let input = vec![1, 2019, 5, 4, 2];
        assert_eq!(part1(input.as_slice()), 2019)
    }

    #[test]
    fn entries_at_end(){
        let input = vec![3, 4, 5, 7, 3, 90, 10, 2010];
        assert_eq!(part1(input.as_slice()), 20100)
    }

    #[test]
    fn entries_in_middle(){
        let input = vec![3, 4, 5, 7, 3, 90, 1021, 999, 34, 234, 909, 341];
        assert_eq!(part1(input.as_slice()), 1019979)
    }

    #[test]
    fn entries_not_in_sequence(){
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(part1(input.as_slice()), 514579)
    }
}