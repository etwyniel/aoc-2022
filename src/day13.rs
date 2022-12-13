use std::{cmp::Ordering, iter::Peekable};

use aoc_framework::*;

pub struct Day13;

impl_day!(Day13::{Part1, Part2}: 2022[13], r"
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
");

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Token {
    ListStart,
    ListEnd,
    Number(u8),
}

use Token::*;

struct Tokenizer<I: Iterator<Item = u8>> {
    it: Peekable<I>,
}

impl<I: Iterator<Item = u8>> Iterator for Tokenizer<I> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if self.it.peek() == Some(&b',') {
            self.it.next();
        }
        Some(match self.it.peek()? {
            b'[' => {
                self.it.next();
                ListStart
            }
            b']' => {
                self.it.next();
                ListEnd
            }
            b'0'..=b'9' => {
                let mut n = 0;
                while let Some(&digit) = self.it.peek() {
                    if !digit.is_ascii_digit() {
                        break;
                    }
                    n = n * 10 + (digit - b'0');
                    self.it.next();
                }
                Number(n)
            }
            _ => return None,
        })
    }
}

impl<I: Iterator<Item = u8>> Tokenizer<I> {
    fn new(it: I) -> Self {
        Tokenizer { it: it.peekable() }
    }
}

struct TokenStream<I> {
    it: I,
    buf: Vec<Token>,
}

impl<I: Iterator<Item = Token>> TokenStream<I> {
    fn new(iter: I) -> Self {
        TokenStream {
            it: iter,
            buf: Vec::new(),
        }
    }

    fn next(&mut self) -> Option<Token> {
        self.buf.pop().or_else(|| self.it.next())
    }

    fn push(&mut self, tok: Token) {
        self.buf.push(tok);
    }
}

fn compare_pair(left: &str, right: &str) -> Ordering {
    let mut left = TokenStream::new(Tokenizer::new(left.bytes()));
    let mut right = TokenStream::new(Tokenizer::new(right.bytes()));
    loop {
        let (Some(ltok), Some(rtok)) = (left.next(), right.next()) else {
            break
        };
        match (ltok, rtok) {
            (ListStart, ListStart) | (ListEnd, ListEnd) => {}
            (ListEnd, _) => return Ordering::Less,
            (_, ListEnd) => return Ordering::Greater,
            (Number(l), ListStart) => {
                left.push(ListEnd);
                left.push(Number(l));
                left.push(ListStart);

                right.push(ListStart);
            }
            (ListStart, Number(r)) => {
                right.push(ListEnd);
                right.push(Number(r));
                right.push(ListStart);

                left.push(ListStart);
            }
            (Number(l), Number(r)) => match l.cmp(&r) {
                ord @ (Ordering::Less | Ordering::Greater) => return ord,
                Ordering::Equal => (),
            },
        }
    }
    Ordering::Equal
}

pub struct Part1;

impl Part for Part1 {
    type D = Day13;
    const N: u8 = 1;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(13));

    fn run(mut input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let mut correct = 0;
        let mut n = 1;
        while let (Some(left), Some(right)) = (input.next(), input.next()) {
            if compare_pair(&left, &right) == Ordering::Less {
                correct += n
            }

            n += 1;
            input.next();
        }
        Ok(Num(correct))
    }
}

pub struct Part2;

const DIVIDERS: [&str; 2] = ["[[2]]", "[[6]]"];

impl Part for Part2 {
    type D = Day13;
    const N: u8 = 2;
    const EXAMPLE_RESULT: Option<Answer> = Some(Num(140));

    fn run(input: impl Iterator<Item = String>) -> anyhow::Result<Answer> {
        let mut packets = input.filter(|line| !line.is_empty()).collect::<Vec<_>>();
        DIVIDERS
            .iter()
            .for_each(|div| packets.push(div.to_string()));
        packets.sort_by(|l, r| compare_pair(l, r));
        Ok(Num(packets
            .iter()
            .enumerate()
            .filter(|(_, packet)| DIVIDERS.contains(&packet.as_str()))
            .map(|(pos, _)| pos as u64 + 1)
            .product()))
    }
}
