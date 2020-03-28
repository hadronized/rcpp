use std::iter::Peekable;
use std::marker::PhantomData;

pub struct Parser<'a, C>
where
  C: Iterator,
{
  chars: Peekable<C>,
  line: usize,
  col: usize,
  _phantom: PhantomData<&'a ()>,
}

impl<'a> Parser<'a, std::str::Chars<'a>> {
  fn new(s: &'a str) -> Self {
    Parser {
      chars: s.chars().peekable(),
      line: 1,
      col: 1,
      _phantom: PhantomData,
    }
  }

  fn line(&self) -> usize {
    self.line
  }

  fn col(&self) -> usize {
    self.col
  }
}

impl<'a, C> Parser<'a, C>
where
  C: Iterator<Item = char>,
{
  fn advance_state(&mut self, c: char) {
    match c {
      '\n' => {
        self.line += 1;
        self.col = 1;
      }

      _ => {
        self.col += 1;
      }
    }
  }

  fn char(&mut self) -> Option<char> {
    self.chars.next().and_then(|c| match c {
      '\\' => {
        // special case for \, as it marks a line break if the next character is a \n; in this
        // case, we have to peek the next caracter and ensure it’s a \n
        self.chars.peek().cloned().and_then(|c| match c {
          '\n' => {
            let _ = self.chars.next();
            self.char()
          }

          _ => {
            self.advance_state(c);
            Some('\\')
          }
        })
      }

      _ => {
        self.advance_state(c);
        Some(c)
      }
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn char() {
    let mut parser = Parser::new("abc");

    assert_eq!(parser.char(), Some('a'));
    assert_eq!(parser.char(), Some('b'));
    assert_eq!(parser.char(), Some('c'));
    assert_eq!(parser.char(), None);
    assert_eq!(parser.char(), None);
  }

  #[test]
  fn col() {
    let mut parser = Parser::new("abc");

    assert_eq!(parser.col(), 1);

    assert_eq!(parser.char(), Some('a'));
    assert_eq!(parser.col(), 2);

    assert_eq!(parser.char(), Some('b'));
    assert_eq!(parser.col(), 3);

    assert_eq!(parser.char(), Some('c'));
    assert_eq!(parser.col(), 4);

    assert_eq!(parser.char(), None);
    assert_eq!(parser.col(), 4);

    assert_eq!(parser.char(), None);
    assert_eq!(parser.col(), 4);
  }

  #[test]
  fn line() {
    let mut parser = Parser::new("a\nb\nc\n");

    assert_eq!(parser.line(), 1);

    assert_eq!(parser.char(), Some('a'));
    assert_eq!(parser.line(), 1);

    assert_eq!(parser.char(), Some('\n'));
    assert_eq!(parser.line(), 2);

    assert_eq!(parser.char(), Some('b'));
    assert_eq!(parser.line(), 2);

    assert_eq!(parser.char(), Some('\n'));
    assert_eq!(parser.line(), 3);

    assert_eq!(parser.char(), Some('c'));
    assert_eq!(parser.line(), 3);

    assert_eq!(parser.char(), Some('\n'));
    assert_eq!(parser.line(), 4);

    assert_eq!(parser.char(), None);
    assert_eq!(parser.line(), 4);

    assert_eq!(parser.char(), None);
    assert_eq!(parser.line(), 4);
  }

  #[test]
  fn line_col() {
    let mut parser = Parser::new("abc\n fo\no");

    assert_eq!(parser.col(), 1);
    assert_eq!(parser.line(), 1);

    assert_eq!(parser.char(), Some('a'));
    assert_eq!(parser.col(), 2);
    assert_eq!(parser.line(), 1);

    assert_eq!(parser.char(), Some('b'));
    assert_eq!(parser.col(), 3);
    assert_eq!(parser.line(), 1);

    assert_eq!(parser.char(), Some('c'));
    assert_eq!(parser.col(), 4);
    assert_eq!(parser.line(), 1);

    assert_eq!(parser.char(), Some('\n'));
    assert_eq!(parser.col(), 1);
    assert_eq!(parser.line(), 2);

    assert_eq!(parser.char(), Some(' '));
    assert_eq!(parser.col(), 2);
    assert_eq!(parser.line(), 2);

    assert_eq!(parser.char(), Some('f'));
    assert_eq!(parser.col(), 3);
    assert_eq!(parser.line(), 2);

    assert_eq!(parser.char(), Some('o'));
    assert_eq!(parser.col(), 4);
    assert_eq!(parser.line(), 2);

    assert_eq!(parser.char(), Some('\n'));
    assert_eq!(parser.col(), 1);
    assert_eq!(parser.line(), 3);

    assert_eq!(parser.char(), Some('o'));
    assert_eq!(parser.col(), 2);
    assert_eq!(parser.line(), 3);

    assert_eq!(parser.char(), None);
    assert_eq!(parser.col(), 2);
    assert_eq!(parser.line(), 3);

    assert_eq!(parser.char(), None);
    assert_eq!(parser.col(), 2);
    assert_eq!(parser.line(), 3);
  }

  #[test]
  fn multiline() {
    let mut parser = Parser::new("a\\\nb\\ ");

    assert_eq!(parser.char(), Some('a'));
    assert_eq!(parser.char(), Some('b'));
    assert_eq!(parser.char(), Some('\\'));
    assert_eq!(parser.char(), Some(' '));
    assert_eq!(parser.char(), None);
  }
}
