use rand::{distributions::Alphanumeric, thread_rng, Rng};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone)]
pub struct SubscriptionToken(String);

impl Default for SubscriptionToken {
    fn default() -> Self {
        let s: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(25)
            .map(char::from)
            .collect();
        Self(s)
    }
}

impl SubscriptionToken {
    pub fn parse(input: &str) -> Result<Self, String> {
        match input.graphemes(true).count() < 26 && input.chars().all(char::is_alphanumeric) {
            true => Ok(Self(input.into())),
            false => Err(format!("{} is not a valid subscription token", input)),
        }
    }
}

impl AsRef<str> for SubscriptionToken {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use claims::{assert_err, assert_ok};

    use crate::domain::SubscriptionToken;

    #[test]
    fn a_25_grapheme_long_name_is_valid() {
        let input = "a".repeat(25);
        assert_ok!(SubscriptionToken::parse(&input));
    }

    #[test]
    fn a_26_grapheme_long_name_is_valid() {
        let input = "a".repeat(26);
        assert_err!(SubscriptionToken::parse(&input));
    }

    impl quickcheck::Arbitrary for SubscriptionToken {
        fn arbitrary<G: quickcheck::Gen>(_: &mut G) -> Self {
            SubscriptionToken::default()
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_tokens_are_parsed_successfully(valid_token: SubscriptionToken) -> bool {
        SubscriptionToken::parse(&valid_token.0).is_ok()
    }

    #[test]
    fn inputs_containing_an_invalid_character_are_rejected() {
        for input in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = input.to_string();
            assert_err!(SubscriptionToken::parse(&name));
        }
    }
}
