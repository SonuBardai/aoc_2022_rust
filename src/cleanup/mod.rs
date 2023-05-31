pub mod pair;
use pair::Pair;

pub fn get_pairs(input_values: Vec<&str>) -> Vec<Vec<&str>> {
    let mut divided_pairs: Vec<Vec<&str>> = Vec::new();
    for pair in input_values {
        let divided = pair.split(',').collect();
        divided_pairs.push(divided);
    }
    divided_pairs
}

pub fn parse_pairs(pairs: Vec<Vec<&str>>) -> Vec<Pair> {
    let mut parsed_pairs = Vec::new();
    for pair in pairs {
        parsed_pairs.push(Pair::build_pair(pair));
    }
    parsed_pairs
}

pub fn get_contained_pairs(pairs: Vec<Vec<&str>>) -> Vec<Pair> {
    let mut contained_pairs: Vec<Pair> = Vec::new();
    let parsed_pairs = parse_pairs(pairs);
    for pair in parsed_pairs {
        if (pair.0.min >= pair.1.min && pair.0.max <= pair.1.max)
            || (pair.1.min >= pair.0.min && pair.1.max <= pair.0.max)
        {
            contained_pairs.push(pair);
        }
    }
    contained_pairs
}

/*
.234.....  2-4
.....678.  6-8

.23......  2-3
...45....  4-5

....567..  5-7
......789  7-9

.2345678.  2-8
..34567..  3-7

.....6...  6-6
...456...  4-6

.23456...  2-6
...45678.  4-8
*/

pub fn get_overlapping_pairs(pairs: Vec<Vec<&str>>) -> Vec<Pair> {
    let mut overlapping_pairs: Vec<Pair> = Vec::new();
    let parsed_pairs = parse_pairs(pairs);
    for pair in parsed_pairs {
        if (pair.0.max >= pair.1.min && pair.0.max <= pair.1.max)
            || (pair.1.max >= pair.0.min && pair.1.max <= pair.0.max)
        {
            overlapping_pairs.push(pair);
        }
    }
    overlapping_pairs
}

#[cfg(test)]
mod test {
    use super::get_pairs;

    #[test]
    fn test_get_pairs() {
        let input_values = vec!["2-4,6-8", "4,3-8", "1-9,14-352"];
        let expected_pairs = vec![vec!["2-4", "6-8"], vec!["4", "3-8"], vec!["1-9", "14-352"]];
        let pairs = get_pairs(input_values);
        assert_eq!(pairs, expected_pairs);
    }
}
