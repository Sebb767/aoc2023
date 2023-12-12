use std::cmp::Ordering;
use std::fmt::Display;
use crate::return_none_unless;
use crate::tools::get_input_or_panic;

#[derive(Debug)]
#[derive(Clone)]
struct Card {
    sign : char,
    value : Bid,
}

impl Card {
    const CARD_CHARS : &'static str = "23456789TJQKA";
    fn new(sign : char) -> Option<Card> {
        let value_index = Self::CARD_CHARS.find(sign)?;
        Some(Card {
            sign,
            value: (value_index as Bid) + 2
        })
    }
}


impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.value.cmp(&other.value);
    }
}

impl PartialEq<Self> for Card {
    fn eq(&self, other: &Self) -> bool {
        return self.value.eq(&other.value);
    }
}

impl Eq for Card {}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.value.cmp(&other.value));
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
    cards : String,
    bid : Bid,
}

#[derive(Debug)]
struct AnalyzedHand {
    cards : CardStack,
    bid : Bid,
    hand_type : HandType
}
impl AnalyzedHand {
    fn new(hand : &Hand) -> Option<AnalyzedHand> {
        return_none_unless!(hand.cards.len() == 5);
        let cards : CardStack = hand.cards
            .chars()
            .map(Card::new)
            .collect::<Option<Vec<Card>>>()?
            .try_into()
            .ok()?;
        let hand_type = analayze_stack(cards.clone());

        Some(AnalyzedHand {
            cards,
            hand_type,
            bid: hand.bid
        })
    }

}

impl Eq for AnalyzedHand {}

impl PartialEq<Self> for AnalyzedHand {
    fn eq(&self, other: &Self) -> bool {
        return self.cmp(other).is_eq();
    }
}

impl PartialOrd<Self> for AnalyzedHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for AnalyzedHand {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_order = self.hand_type.cmp(&other.hand_type);
        if type_order.is_eq() {
            for i in 0..5 {
                let card_order = self.cards[i].cmp(&other.cards[i]);
                if !card_order.is_eq() {
                    return card_order
                }
            }
        }
        type_order
    }
}


impl Display for AnalyzedHand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stack : String = self.cards.clone().map(|c| c.sign).iter().collect();
        write!(f, "{stack} -> {:?} with bid {}", self.hand_type, self.bid)
    }
}

fn parse_input_to_analyzed_hand(input : String) -> Option<Vec<AnalyzedHand>> {
    input
        .lines()
        .map(|line| {
            assert!(line.len() >= 7); // at least 5 chars, a space and a bid of at least 1
            AnalyzedHand::new(&Hand {
                cards: line[0..5].to_string(),
                bid: line[6..].parse().ok()?
            })
        })
        .collect()
}

fn analayze_stack(mut card_stack: CardStack) -> HandType {
    card_stack.sort();

    let mut counts = [ 1, 1, 1 ];
    let mut offset = 0;
    for i in 1..5 {
        if card_stack[i-1] == card_stack[i] {
            counts[offset] += 1;
        }
        else {
            assert!(offset <= 2);
            if offset == 2 {
                // This might be the last or second to last card (otherwise offset could not have
                // been increased twice). Let's assert this to be sure :-)
                assert!(i == 3 || i == 4);
                if i == 3 {
                    // we're at the second to last card and, since offset was not increased before,
                    // all previous cards were distinct. This only leaves two options:
                    return if card_stack[3] == card_stack[4] { HandType::OnePair } else { HandType::HighCard };
                }
                else {
                    // We're at the last card and it's equal to nothing that came before. So we
                    // don't need to do anything. Not even break, since it's the last iteration
                    // anyway :)
                }
            }
            else {
                // We do not need to increase count in this case since we initialized it with one
                offset += 1;
            }
        }
    }

    // Now that we counted duplicates, kinda, let's reap the result
    counts.sort();

    match counts {
        [ 1, 1, 5 ] => HandType::FiveOfAKind,
        [ 1, 1, 4 ] => HandType::FourOfAKind,
        [ 1, 2, 3 ] => HandType::FullHouse,
        [ 1, 1, 3 ] => HandType::ThreeOfAKind,
        [ 1, 2, 2 ] => HandType::TwoPair,
        [ 1, 1, 2 ] => HandType::OnePair,
        _ => panic!("This should not happen! Cards {:?} counts {:?}", card_stack, counts)
    }
}

fn score_hands(hands : &mut Vec<AnalyzedHand>) -> Vec<Bid> {
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
    let input = get_input_or_panic("7-1");
    let mut parsed = parse_input_to_analyzed_hand(input).unwrap();
    let score = score_hands(&mut parsed);
    let result : Bid = score.iter().sum();

    //assert_eq!(result, 1);
    println!("Sum of all scores: {result}");
}

fn day7_2() {
    let input = get_input_or_panic("7-1");
}