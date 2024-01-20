pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;

    let mut groups: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
    for s in strs.into_iter() {
        let mut bytes: Vec<u8> = s.as_bytes().to_vec();
        bytes.sort_unstable();
        let group = groups.entry(bytes).or_insert(Vec::new());
        group.push(s);
    }
    groups.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verify(mut actual: Vec<Vec<String>>, mut expected: Vec<Vec<String>>) {
        actual.iter_mut().for_each(|v| v.sort_unstable());
        expected.iter_mut().for_each(|v| v.sort_unstable());
        actual.sort_unstable();
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_1() {
        let input: Vec<String> = ["eat", "tea", "tan", "ate", "nat", "bat"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let expected: Vec<Vec<String>> =
            [vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]]
                .into_iter()
                .map(|v| v.into_iter().map(|s| s.to_string()).collect())
                .collect();
        verify(group_anagrams(input), expected);
    }
    #[test]
    fn test_2() {
        let input: Vec<String> = [""].into_iter().map(|s| s.to_string()).collect();
        let expected: Vec<Vec<String>> = [[""]]
            .into_iter()
            .map(|v| v.into_iter().map(|s| s.to_string()).collect())
            .collect();
        verify(group_anagrams(input), expected);
    }
    #[test]
    fn test_3() {
        let input: Vec<String> = ["a"].into_iter().map(|s| s.to_string()).collect();
        let expected: Vec<Vec<String>> = [["a"]]
            .into_iter()
            .map(|v| v.into_iter().map(|s| s.to_string()).collect())
            .collect();
        verify(group_anagrams(input), expected);
    }
}
