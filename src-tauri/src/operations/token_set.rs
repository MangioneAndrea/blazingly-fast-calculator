use crate::generic_error::ParsingTokenError;

use super::{token::Token, token_tree::TokenTree};

#[derive(Debug)]
pub struct TokenSet<T>(pub Vec<Token>, std::marker::PhantomData<T>);

#[derive(Debug)]
pub struct Invalid;
#[derive(Debug)]
pub struct Valid;

impl TokenSet<Invalid> {
    pub fn new(s: &str) -> Result<TokenSet<Invalid>, ParsingTokenError> {
        let mut result = vec![];
        let mut current_token = Token::None;
        for c in s.chars() {
            current_token = match current_token.digest(c) {
                // If a token failed, the whole thing fails
                Err(e) => return Err(e),
                // If it yielded 2 tokens, the previous one can be "committed"
                Ok((old, Some(new))) => {
                    result.push(old);
                    new
                }
                // The old token was edited
                Ok((old, None)) => old,
            };
        }

        if !current_token.in_none() {
            result.push(current_token);
        }

        Ok(TokenSet(result, std::marker::PhantomData::<Invalid>))
    }

    pub fn validate(self) -> Result<TokenSet<Valid>, ParsingTokenError> {
        let mut parenthesis_opened = 0;

        let mut previous_token: &Token = &Token::None;

        for token in &self.0 {
            if !previous_token.can_be_followed_by(token) {
                return Err(ParsingTokenError::InvalidSequence);
            }

            parenthesis_opened += match &token {
                Token::ParenthesisClose => -1,
                Token::ParenthesisOpen => 1,
                _ => 0,
            };

            if parenthesis_opened < 0 {
                return Err(ParsingTokenError::ParenthesisClosedWithoutOpening);
            }
            previous_token = token;
        }

        if parenthesis_opened != 0 {
            return Err(ParsingTokenError::ParenthesisOpenedWithoutClosing);
        }

        Ok(TokenSet(self.0, std::marker::PhantomData::<Valid>))
    }
}

impl TokenSet<Valid> {
    /*
     * The idea is the following. Given a valid token set, try to split it as follows

     7 + ( 9 - 2 ) * 7 + 3 * 4
                       +
                    /      \
     7 + ( 9 - 2 ) * 7     3 * 4
       +
     /   \
    7     ( 9 - 2 ) * 7
                    *
                  /  \
          ( 9 - 2 )    7
              -
            /   \
           9     2
     */

    // First, search for + and - from right to left. What's in parenthesis has to be done last
    // The alg for the priority is in the function get_grade
    pub fn split(self) -> TokenTree {
        // println!("{:?}", self.0); // This is the main debug thing in case stuff doesn't work
        if self.0.is_empty() {
            return TokenTree::Single(Token::Integer(String::from("0")));
        }
        if self.0.len() == 1 {
            return TokenTree::Single(self.0[0].clone());
        }

        // Ignore outer parenthesis, as they don't mean anything
        if self.0[0] == Token::ParenthesisOpen && self.0.last().unwrap() == &Token::ParenthesisClose
        {
            return TokenSet(
                Vec::from_iter(self.0[1..self.0.len() - 1].iter().cloned()),
                std::marker::PhantomData::<Valid>,
            )
            .split();
        }

        let mut parenthesis = 0;

        let mut lowest_grade = usize::MAX;
        let mut lowest_grade_index = 0;
        for index in (0..self.0.len()).rev() {
            match self.0[index] {
                Token::ParenthesisClose => {
                    parenthesis += 1;
                }
                Token::ParenthesisOpen => {
                    parenthesis -= 1;
                }
                _ => {
                    let grade = self.0[index].get_grade(parenthesis);
                    if let Some(g) = grade {
                        if g < lowest_grade {
                            lowest_grade_index = index;
                            lowest_grade = g;
                        }
                    }
                }
            };
        }

        // Only unary ops can be at the beginning
        if lowest_grade_index == 0 {
            return TokenTree::UnaryOperation(
                self.0[lowest_grade_index].clone(),
                Box::new(
                    TokenSet(
                        Vec::from_iter(
                            self.0[lowest_grade_index + 1..self.0.len()].iter().cloned(),
                        ),
                        std::marker::PhantomData::<Valid>,
                    )
                    .split(),
                ),
            );
        }

        return TokenTree::BinaryOperation(
            Box::new(
                TokenSet(
                    Vec::from_iter(self.0[0..lowest_grade_index].iter().cloned()),
                    std::marker::PhantomData::<Valid>,
                )
                .split(),
            ),
            self.0[lowest_grade_index].clone(),
            Box::new(
                TokenSet(
                    Vec::from_iter(self.0[lowest_grade_index + 1..self.0.len()].iter().cloned()),
                    std::marker::PhantomData::<Valid>,
                )
                .split(),
            ),
        );
    }
}
