#![allow(clippy::all)]
use crate::return_none_unless;
use crate::tools::get_input_or_panic;
use std::cmp::Ordering;
use std::fmt::Display;
use std::marker::PhantomData;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Card {
    sign: char,
    value: Bid,
}

impl PartialEq<Bid> for Card {
    fn eq(&self, other: &Bid) -> bool {
        return self.value == *other;
    }
}

impl Card {
    const CARD_CHARS: &'static str = "23456789TJQKA";
    const JOKER: Bid = 11;

    fn new(sign: char) -> Option<Card> {
        let value_index = Self::CARD_CHARS.find(sign)?;
        Some(Card {
            sign,
            value: (value_index as Bid) + 2,
        })
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard = 1,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

type CardStack = [Card; 5];
type Bid = u32;

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: Bid,
}

#[derive(Debug)]
struct AnalyzedHand<T: CardComparer> {
    cards: CardStack,
    bid: Bid,
    hand_type: HandType,
    _marker: PhantomData<T>,
}

trait CardComparer {
    fn cmp(a: &Card, b: &Card) -> Ordering;
    fn analyze_stack(card_stack: CardStack) -> HandType;
}

impl<T: CardComparer> AnalyzedHand<T> {
    fn new(hand: &Hand) -> Option<AnalyzedHand<T>> {
        return_none_unless!(hand.cards.len() == 5);
        let cards: CardStack = hand
            .cards
            .chars()
            .map(Card::new)
            .collect::<Option<Vec<Card>>>()?
            .try_into()
            .ok()?;
        let hand_type = <T as CardComparer>::analyze_stack(cards.clone());

        Some(AnalyzedHand {
            cards,
            hand_type,
            bid: hand.bid,
            _marker: PhantomData,
        })
    }
}

impl<T: CardComparer> Eq for AnalyzedHand<T> {}

impl<T: CardComparer> PartialEq<Self> for AnalyzedHand<T> {
    fn eq(&self, other: &Self) -> bool {
        return self.cmp(other).is_eq();
    }
}

impl<T: CardComparer> PartialOrd<Self> for AnalyzedHand<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl<T: CardComparer> Ord for AnalyzedHand<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_order = self.hand_type.cmp(&other.hand_type);
        if type_order.is_eq() {
            for i in 0..5 {
                let card_order = <T as CardComparer>::cmp(&self.cards[i], &other.cards[i]);
                if !card_order.is_eq() {
                    return card_order;
                }
            }
        }
        type_order
    }
}

impl<T: CardComparer> Display for AnalyzedHand<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stack: String = self.cards.clone().map(|c| c.sign).iter().collect();
        write!(f, "{stack} -> {:?} with bid {}", self.hand_type, self.bid)
    }
}

fn parse_input_to_analyzed_hand<T: CardComparer>(input: String) -> Option<Vec<AnalyzedHand<T>>> {
    input
        .lines()
        .map(|line| {
            assert!(line.len() >= 7); // at least 5 chars, a space and a bid of at least 1
            AnalyzedHand::new(&Hand {
                cards: line[0..5].to_string(),
                bid: line[6..].parse().ok()?,
            })
        })
        .collect()
}

struct SimpleAnalyzer {}

impl CardComparer for SimpleAnalyzer {
    fn cmp(a: &Card, b: &Card) -> Ordering {
        return a.value.cmp(&b.value);
    }

    fn analyze_stack(mut card_stack: CardStack) -> HandType {
        card_stack.sort_by(Self::cmp);

        let mut counts = [1, 1, 1];
        let mut offset = 0;
        for i in 1..5 {
            if &card_stack[i - 1] == &card_stack[i] {
                counts[offset] += 1;
            } else {
                assert!(offset <= 2);
                if offset == 2 {
                    // This might be the last or second to last card (otherwise offset could not have
                    // been increased twice). Let's assert this to be sure :-)
                    assert!(i == 3 || i == 4);
                    if i == 3 {
                        // we're at the second to last card and, since offset was not increased before,
                        // all previous cards were distinct. This only leaves two options:
                        return if &card_stack[3] == &card_stack[4] {
                            HandType::OnePair
                        } else {
                            HandType::HighCard
                        };
                    } else {
                        // We're at the last card and it's equal to nothing that came before. So we
                        // don't need to do anything. Not even break, since it's the last iteration
                        // anyway :)
                    }
                } else {
                    // We do not need to increase count in this case since we initialized it with one
                    offset += 1;
                }
            }
        }

        // Now that we counted duplicates, kinda, let's reap the result
        counts.sort();

        match counts {
            [1, 1, 5] => HandType::FiveOfAKind,
            [1, 1, 4] => HandType::FourOfAKind,
            [1, 2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 2] => HandType::OnePair,
            _ => panic!(
                "This should not happen! Cards {:?} counts {:?}",
                card_stack, counts
            ),
        }
    }
}

struct JokerAwareAnalyzer;

impl JokerAwareAnalyzer {
    fn stack_without_joker(card_stack: &CardStack) -> Vec<&Card> {
        card_stack
            .iter()
            .filter(|c| c.value != Card::JOKER)
            .collect()
    }
}

impl CardComparer for JokerAwareAnalyzer {
    fn cmp(a: &Card, b: &Card) -> Ordering {
        match (a.value, b.value) {
            (Card::JOKER, Card::JOKER) => Ordering::Equal,
            (_, Card::JOKER) => Ordering::Greater,
            (Card::JOKER, _) => Ordering::Less,
            _ => a.value.cmp(&b.value),
        }
    }

    fn analyze_stack(card_stack: CardStack) -> HandType {
        let min_stack = Self::stack_without_joker(&card_stack);
        if min_stack.len() == 5 {
            // no jokers - the old rules apply
            return SimpleAnalyzer::analyze_stack(card_stack);
        } else if min_stack.len() < 2 {
            // if we only have jokers or jokers plus one card, we go for five of a kind
            return HandType::FiveOfAKind;
        } else if min_stack.len() == 2 {
            // If we have two non-jokers, we can still make five of a kind if those are equal. If
            // they aren't, we go for four of a kind, as it is the next highest
            return if min_stack[0] == min_stack[1] {
                HandType::FiveOfAKind
            } else {
                HandType::FourOfAKind
            };
        }

        // we now know we have 1-2 Jokers and 3-4 remaining cards
        if min_stack.len() == 3 {
            /*
            We have three cases:
            - All cards equal -> Five of a kind
            - One pair -> Four of a kind
            - All unique -> Three of a kind
             */
            let a = min_stack[0] == min_stack[1];
            return if min_stack[1] == min_stack[2] {
                if a {
                    HandType::FiveOfAKind
                } else {
                    HandType::FourOfAKind
                }
            } else if a || min_stack[0] == min_stack[2] {
                HandType::FourOfAKind
            } else {
                // no pair
                HandType::ThreeOfAKind
            };
        } else {
            assert_eq!(min_stack.len(), 4);
            /*
            Let's see:
            - All cards equal -> Five of a kind
            - Three cards equal -> Four of a kind
            - Two pairs -> Full house
            - One pair -> Three of a kind
            - All unique -> One Pair

            We can simply map these results from our parent - what a nice feature!
             */
            return match SimpleAnalyzer::analyze_stack(card_stack) {
                HandType::FourOfAKind => HandType::FiveOfAKind,
                HandType::ThreeOfAKind => HandType::FourOfAKind,
                HandType::TwoPair => HandType::FullHouse,
                HandType::OnePair => HandType::ThreeOfAKind,
                HandType::HighCard => HandType::OnePair,
                _ => unreachable!("This should not happen!"),
            };
        }

        #[allow(unreachable_code)]
        {
            unreachable!();
        }
    }
}

fn score_hands<T: CardComparer>(hands: &mut Vec<AnalyzedHand<T>>) -> Vec<Bid> {
    hands.sort();
    // With the hands sorted in ascending order, calculating the scores is easy
    hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| {
            //println!("{}", &hand);
            hand.bid * (idx as Bid + 1)
        })
        .collect()
}

#[allow(dead_code)]
pub fn day7() {
    day7_1();
    day7_2();
}

fn day7_1() {
    let input = get_input_or_panic("7-1", 2023);
    let mut parsed = parse_input_to_analyzed_hand::<SimpleAnalyzer>(input).unwrap();
    let score = score_hands(&mut parsed);
    let result: Bid = score.iter().sum();

    assert_eq!(result, 251029473);
    println!("Sum of all scores: {result}");
}

fn day7_2() {
    let input = get_input_or_panic("7-1", 2023);

    let mut parsed = parse_input_to_analyzed_hand::<JokerAwareAnalyzer>(input).unwrap();
    let score = score_hands(&mut parsed);
    let result: Bid = score.iter().sum();

    //assert_eq!(result, 251029473);
    println!("Sum of all scores: {result}");
}
