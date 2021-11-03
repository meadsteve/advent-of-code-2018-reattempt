use std::fmt::Formatter;
use std::str::FromStr;

struct LinkedLetter {
    letter: char,
    next: Option<Box<LinkedLetter>>,
}

impl LinkedLetter {
    fn iter(&self) -> LinkedLetterIterator {
        LinkedLetterIterator { current: self }
    }
}

impl std::fmt::Display for LinkedLetter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for element in self.iter() {
            write!(f, "{}", element.letter)?;
        }
        Ok(())
    }
}

impl FromStr for LinkedLetter {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut linked_letters: Option<Box<LinkedLetter>> = None;
        for c in s.chars().rev() {
            linked_letters = Some(Box::new(LinkedLetter {
                letter: c,
                next: linked_letters,
            }));
        }
        let first = LinkedLetter {
            letter: ' ',
            next: linked_letters,
        };
        Ok(first)
    }
}

struct LinkedLetterIterator<'a> {
    current: &'a LinkedLetter,
}

impl<'a> Iterator for LinkedLetterIterator<'a> {
    type Item = &'a LinkedLetter;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.current.next {
            Some(n) => {
                self.current = n;
                Some(self.current)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day_05::linked_letters::*;

    #[test]
    fn test_iterators_work() {
        let ab: LinkedLetter = "ab".parse().unwrap();
        let mut iter = ab.iter();
        assert_eq!('a', iter.next().unwrap().letter);
        assert_eq!('b', iter.next().unwrap().letter);
    }

    #[test]
    fn test_iterators_are_peekable() {
        let ab: LinkedLetter = "ab".parse().unwrap();
        let mut iter = ab.iter().peekable();
        assert_eq!('a', iter.next().unwrap().letter);
        assert_eq!('b', iter.peek().unwrap().letter);
        assert_eq!('b', iter.next().unwrap().letter);
    }

    #[test]
    fn test_linked_letters_can_be_printed_as_a_string() {
        let ab: LinkedLetter = "ab".parse().unwrap();
        assert_eq!("ab", format!("{}", ab));
    }
}
