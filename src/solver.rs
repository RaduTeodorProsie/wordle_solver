use Info::{Green, Yellow};

use std::cmp::PartialEq;
use std::collections::HashMap;

use std::sync::OnceLock;

#[derive(Clone, PartialEq, Debug, Copy, Eq, Hash)]
pub enum Info {
    Green,
    Yellow,
    Grey,
}

pub type Word = [u8; 5];
type Pattern = [Info; 5];

static WORDS: OnceLock<Vec<Word>> = OnceLock::new();
static ANSWERS: OnceLock<Vec<Word>> = OnceLock::new();

pub fn get_words() -> &'static Vec<Word> {
    WORDS.get_or_init(|| {
        include_str!("../valid-words.txt")
            .lines()
            .map(|l| l.trim())
            .filter_map(|line| line.as_bytes().try_into().ok())
            .collect::<Vec<Word>>()
    })
}

fn get_answers() -> &'static Vec<Word> {
    ANSWERS.get_or_init(|| {
        include_str!("../valid-answers.txt")
            .lines()
            .map(|l| l.trim())
            .filter_map(|line| line.as_bytes().try_into().ok())
            .collect::<Vec<Word>>()
    })
}

fn get_feedback(guess: &Word, good: &Word) -> Pattern {
    let mut ans = [Info::Grey; 5];
    let mut freq = [0; 26];
    for i in 0..5 {
        if guess[i] == good[i] {
            ans[i] = Green;
        } else {
            let letter = (good[i] - b'a') as usize;
            freq[letter] += 1;
        }
    }

    for i in 0..5 {
        if ans[i] == Green {
            continue;
        }

        let letter = (guess[i] - b'a') as usize;
        if freq[letter] > 0 {
            ans[i] = Yellow;
            freq[letter] -= 1;
        }
    }

    ans
}

fn matches_info(word: &Word, guess: &Word, info: &Pattern) -> bool {
    get_feedback(guess, word) == *info
}

pub fn best_guess(guesses: &Vec<Word>, feedback: &Vec<Pattern>) -> Word {
    if feedback.is_empty() {
        return b"roate".to_owned();
    }
    let words = get_words();
    let possible_answers: Vec<&Word> = get_answers()
        .iter()
        .filter(|&candidate| {
            guesses
                .iter()
                .zip(feedback.iter())
                .all(|(g, f)| matches_info(candidate, g, f))
        })
        .collect();
    if possible_answers.len() == 1 {
        return *possible_answers[0];
    }
    if possible_answers.is_empty() {
        panic!(
            "No matching answers for the provided feedback; check your feedback parsing and guess histories"
        );
    }
    let list_size_if_guessed: Vec<(Word, f64)> = words
        .iter()
        .map(|word| {
            let mut frq: HashMap<Pattern, usize> = HashMap::new();
            possible_answers.iter().for_each(|&answer| {
                *frq.entry(get_feedback(word, answer)).or_insert(0) += 1;
            });

            let mut expected = 0f64;
            let total = possible_answers.len() as f64;
            for (p, count) in &frq {
                let still_valid = possible_answers
                    .iter()
                    .filter(|&&check| matches_info(check, word, p))
                    .count() as f64;
                expected += *count as f64 * still_valid / total;
            }

            (*word, expected)
        })
        .collect();
    if let Some((best_guess, _)) = list_size_if_guessed
        .iter()
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    {
        return best_guess.clone().to_owned();
    };

    b"Soare".to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_word(s: &str) -> Word {
        let b = s.as_bytes();
        [b[0], b[1], b[2], b[3], b[4]]
    }

    fn pat_str(p: Pattern) -> String {
        p.iter()
            .map(|c| match c {
                Info::Green => 'G',
                Info::Yellow => 'Y',
                Info::Grey => 'B',
            })
            .collect()
    }

    #[test]
    fn all_green_when_same_word() {
        let guess = to_word("crane");
        let answer = to_word("crane");
        let p = get_feedback(&guess, &answer);
        assert_eq!(p, [Green; 5]);
    }

    #[test]
    fn no_matches_all_grey() {
        let guess = to_word("spout");
        let answer = to_word("crane"); // no shared letters
        let p = get_feedback(&guess, &answer);
        assert_eq!(p, [Info::Grey; 5]);
    }

    #[test]
    fn duplicated_letters_example_alee_eagle() {
        // from earlier: guess "allee", answer "eagle" -> Y Y B Y G
        let guess = to_word("allee");
        let answer = to_word("eagle");
        let p = get_feedback(&guess, &answer);
        let expected = [
            Info::Yellow,
            Info::Yellow,
            Info::Grey,
            Info::Yellow,
            Info::Green,
        ];
        assert_eq!(
            p,
            expected,
            "got {} expected {}",
            pat_str(p),
            pat_str(expected)
        );
    }

    #[test]
    fn allee_vs_eagle_duplicates_in_guess() {
        // from earlier: guess "allee", answer "eagle" -> Y Y B Y G
        let guess = to_word("allee");
        let answer = to_word("eagle");
        let p = get_feedback(&guess, &answer);
        let expected = [
            Info::Yellow,
            Info::Yellow,
            Info::Grey,
            Info::Yellow,
            Info::Green,
        ];
        assert_eq!(
            p,
            expected,
            "got {} expected {}",
            pat_str(p),
            pat_str(expected)
        );
    }

    #[test]
    fn mixed_example_soare_vs_crane() {
        // guess "soare", answer "crane" -> B B G Y G
        let guess = to_word("soare");
        let answer = to_word("crane");
        let p = get_feedback(&guess, &answer);
        let expected = [
            Info::Grey,
            Info::Grey,
            Info::Green,
            Info::Yellow,
            Info::Green,
        ];
        assert_eq!(
            p,
            expected,
            "got {} expected {}",
            pat_str(p),
            pat_str(expected)
        );
    }

    #[test]
    fn matches_info_should_return_true_for_valid_candidate() {
        // Compute feedback for guess "soare" against answer "crane" and ensure
        // matches_info accepts "crane" as a valid candidate for that feedback.
        let guess = to_word("soare");
        let candidate = to_word("crane");
        let info = get_feedback(&guess, &candidate); // soare -> B B G Y G

        // Sanity-check expected pattern to make the test explicit
        let expected = [
            Info::Grey,
            Info::Grey,
            Info::Green,
            Info::Yellow,
            Info::Green,
        ];
        assert_eq!(
            info, expected,
            "computed feedback must match the expected pattern"
        );

        assert!(
            matches_info(&candidate, &guess, &info),
            "'crane' should be a valid candidate for guess 'soare' with the computed feedback"
        );
    }

    #[test]
    fn matches_info_should_return_false_for_invalid_candidate() {
        let guess = to_word("soare");
        let valid_candidate = to_word("crane");
        let invalid_candidate = to_word("spout");
        let info = get_feedback(&guess, &valid_candidate); // soare -> B B G Y G

        assert!(
            !matches_info(&invalid_candidate, &guess, &info),
            "'spout' should NOT be a valid candidate for guess 'soare' with the computed feedback"
        );
    }

    #[test]
    fn matches_info_with_duplicates_in_guess() {
        let guess = to_word("allee");
        let candidate = to_word("eagle");
        let info = get_feedback(&guess, &candidate); // expect Y Y B Y G

        let expected = [
            Info::Yellow,
            Info::Yellow,
            Info::Grey,
            Info::Yellow,
            Info::Green,
        ];
        assert_eq!(
            info, expected,
            "computed feedback for duplicated-letters case must match expected"
        );

        assert!(
            matches_info(&candidate, &guess, &info),
            "'eagle' should be a valid candidate for guess 'allee' with the computed feedback"
        );
    }
}
