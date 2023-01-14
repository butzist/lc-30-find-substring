pub struct Solution;

impl Solution {
    pub fn find_substring<S: AsRef<[u8]>, V: AsRef<[S]>>(s: S, words: V) -> Vec<i32> {
        let (words, expected_counts) = {
            let unique: std::collections::HashSet<_> =
                words.as_ref().iter().map(|s| s.as_ref()).collect();

            let mapping: std::collections::HashMap<_, _> = unique
                .into_iter()
                .enumerate()
                .map(|(ix, w)| (w, ix as u16))
                .collect();

            let mut expected_counts = vec![0; mapping.len()];
            for word in words.as_ref() {
                let ix = mapping.get(word.as_ref()).unwrap();
                expected_counts[*ix as usize] += 1;
            }

            (mapping, expected_counts)
        };

        let mut result = vec![];
        let s = s.as_ref();
        let word_len = words.keys().next().unwrap().len();
        let nun_words = expected_counts.iter().sum::<usize>();
        let substring_len = nun_words * word_len;

        for substring_start in 0..=s.len() - substring_len {
            let mut counts = vec![0; words.len()];
            let substring = &s[substring_start..substring_start + substring_len];

            for word_start in (0..nun_words).map(|s| s * word_len) {
                let word = &substring[word_start..word_start + word_len];
                if let Some(ix) = words.get(word) {
                    counts[*ix as usize] += 1;
                }
            }

            if counts == expected_counts {
                result.push(substring_start as i32);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("barfoothefoobarman", &["foo","bar"], &[0,9])]
    #[case("wordgoodgoodgoodbestword", &["word","good","best","word"], &[])]
    #[case("barfoofoobarthefoobarman", &["bar","foo","the"], &[6,9,12])]
    #[case("wordgoodgoodgoodbestword", &["word","good","best","good"], &[8])]
    fn test(#[case] s: &str, #[case] words: &[&str], #[case] result: &[i32]) {
        assert_eq!(&Solution::find_substring(s, words), result);
    }
}
