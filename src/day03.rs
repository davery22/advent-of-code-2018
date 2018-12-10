use std::collections::HashSet;
use regex::Regex;

pub fn run() {
    println!("On the third day of Christmas, AoC gave to me...");

    // Someone more familiar with R-Trees could probably do this more efficiently
    let fabric_with_claims = get_fabric_with_claims(include_str!("../input/day03.in"));
    println!("{}", count_conflicting_squares(&fabric_with_claims));
    println!("{}", get_non_overlapping_claim_id(&fabric_with_claims));

    println!("One conflicting-claim count and the only non-overlapping claim ID!");
    println!();
}

fn get_fabric_with_claims<>(claims: &str) -> Vec<Vec<HashSet<&str>>> {
    let mut fabric: Vec<Vec<HashSet<&str>>> = vec![vec![HashSet::new(); 1000]; 1000];
    let claim_decoder = Regex::new(r"#(\w+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    for claim in claim_decoder.captures_iter(claims) {
        // Need to use .get(i) instead of indexing, due to lifetime peculiarities
        // See https://docs.rs/regex/*/regex/struct.Captures.html#impl-Index%3Cusize%3E
        let claim_id = claim.get(1).unwrap().as_str();
        let top_left_column: usize = claim[2].parse().unwrap();
        let top_left_row: usize = claim[3].parse().unwrap();
        let width: usize = claim[4].parse().unwrap();
        let height: usize = claim[5].parse().unwrap();

        for row in top_left_row .. top_left_row + height {
            for col in top_left_column .. top_left_column + width {
                fabric.get_mut(row).unwrap().get_mut(col).unwrap().insert(claim_id);
            }
        }
    }

    fabric
}

pub fn count_conflicting_squares(fabric_with_claims: &Vec<Vec<HashSet<&str>>>) -> usize {
    fabric_with_claims.iter().fold(0, |count, row| {
        count + row.iter().filter(|square_claims| square_claims.len() > 1).count()
    })

//    let mut count = 0;
//
//    for row in fabric_with_claims {
//        for square_claims in row {
//            if square_claims.len() > 1 {
//                count += 1;
//            }
//        }
//    }
//
//    count
}

pub fn get_non_overlapping_claim_id<'a>(fabric_with_claims: &Vec<Vec<HashSet<&'a str>>>) -> &'a str {
    let mut all_claim_ids: HashSet<&'a str> = HashSet::new();
    let mut overlapping_claim_ids: HashSet<&'a str> = HashSet::new();

    for row in fabric_with_claims {
        for square_claims in row {
            if square_claims.len() > 1 {
                overlapping_claim_ids.extend(square_claims.iter())
            }

            all_claim_ids.extend(square_claims.iter());
        }
    }

    all_claim_ids.difference(&overlapping_claim_ids).next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counts_conflicting_squares() {
        assert_eq!(4, count_conflicting_squares(&get_fabric_with_claims(
            "#1 @ 1,3: 4x4
                #2 @ 3,1: 4x4
                #3 @ 5,5: 2x2")));
    }

    #[test]
    fn test_gets_non_overlapping_claim_id() {
        assert_eq!("3", get_non_overlapping_claim_id(&get_fabric_with_claims(
            "#1 @ 1,3: 4x4
                #2 @ 3,1: 4x4
                #3 @ 5,5: 2x2")));
    }
}
