//! https://leetcode.com/problems/most-common-word

use std::collections::{HashMap, HashSet};

pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
    let mut dict: HashMap<String, u32> = HashMap::new();
    let banned: HashSet<String> = banned.into_iter().collect();
    paragraph
        .to_ascii_lowercase()
        .split(|c: char| !c.is_ascii_alphabetic())
        .for_each(|word| {
            if word.is_empty() || banned.contains(word) {
                return;
            }
            match dict.get_mut(word) {
                Some(count) => {
                    *count += 1;
                }
                None => {
                    dict.insert(word.to_string(), 0);
                }
            }
        });
    dict.iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .unwrap()
        .0
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            most_common_word(
                "Bob hit a ball, the hit BALL flew far after it was hit.".to_string(),
                vec!["hit".to_string()]
            ),
            "ball".to_string()
        )
    }

    #[test]
    fn test_case_2() {
        assert_eq!(most_common_word("a.".to_string(), vec![]), "a".to_string())
    }

    #[test]
    fn test_case_3() {
        assert_eq!(most_common_word("K; P; u, M' o. W! S; y? y, k, T' t; M, E, N? Q! J, w' x. s; S. Y. R; V, P? U; P? o; g? W, N; N' S, W, h, Z' T? t! l' T? x! K; o. F? q. w? Q, s? Q! m; g? x? R; L' q, C; f! E? x? T; f, r! O. K! P, x. z; l' j. Y, S! w? w, j. U; s, M? r' J? Z. x; T. z; Q; Q? J' w' W, p! V, P; t! x? o; G' z' u, V, C' S; O? I; g, r. C? y. t! O, t' n. y. Q. N? L. t; I, X' W; w! M; m? Y? f? Z. s? x? U. q! I, i, v! p, V! Z; z; F! D. R, r' y. r' v. j! Y, M! U. M, p; Y, u' P, t, R. w? S! W. X. U? R? X' s, e. w' V; I' Y; z? l! s, u! z! R' a; R' y' S' Q? l, j, L; W. V! w; V, y? R! h. V. L, Y. X, Y' Q? U; y, n! V, y? N; V; x' H' U? K; O! X! d. U, W' U' x. Y; L' X, T? t, V, L; r! k! u' N. y; P. S, e! j, X! t' Z; s? y! u; n' K, T, Z. w. l! Y; g' x, H; U' D! Y? e, b, W. X! u; W; v, S. Z, y? o? K, P? Q? Z, Y' P; n. V? h; J, v! v; X; V. s. y. g' m? l, X. K! u? O? j; u, n' T. O' S; W. U; m. G! Z! Z, K, v. q. Z? q; Z; o? P? I. X, z! t! w' v. z! N' o! M' Z' Z? V. S; Q. O; Z! k. X! r! w. T! q. M? n, I, M; R, d, h; Z, z' n,".to_string(), ["n","q","l","i","u","d","h","o","y","b","c","t","v","a","x","m","k","w","s","z"].iter().map(|x|x.to_string()).collect()), "r".to_string())
    }
}
