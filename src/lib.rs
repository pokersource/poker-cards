// Copyright © 2019 The Pokersource Project
// [This work is made available under the "Affero GPL v3".]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

//! Playing cards, as used in Poker.

use std::cmp::Ordering;
use std::convert::TryFrom;
use Ordering::*;

/// Suit of a card.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Clubs,
    Hearts,
    Diamonds,
    Spades,
}
use Suit::*;

impl Default for Suit {
    fn default() -> Suit {
        Spades
    }
}

impl TryFrom<&str> for Suit {
    type Error = String;

    fn try_from(s: &str) -> Result<Suit, String> {
        match s {
            "C" => Ok(Clubs),
            "H" => Ok(Hearts),
            "D" => Ok(Diamonds),
            "S" => Ok(Spades),
            s => Err(format!("invalid rank: {}", s)),
        }
    }
}

/// Rank of a card.
///
/// When doing a partial-ordered comparison between ranks,
/// Aces are treated as unordered with respect to other
/// ranks, since they might be high or low.  Since ace-high
/// is so common, this is the default order for a
/// total-ordered comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Spot(u8),
}
use Rank::*;

impl Default for Rank {
    fn default() -> Rank {
        Ace
    }
}

impl From<Rank> for u8 {
    fn from(r: Rank) -> u8 {
        match r {
            Ace => 14,
            King => 13,
            Queen => 12,
            Jack => 11,
            Ten => 10,
            Spot(n) => n,
        }
    }
}

impl TryFrom<&str> for Rank {
    type Error = String;

    fn try_from(s: &str) -> Result<Rank, String> {
        match s {
            "A" => Ok(Ace),
            "K" => Ok(King),
            "Q" => Ok(Queen),
            "J" => Ok(Jack),
            "T" => Ok(Ten),
            s => {
                if s.is_empty() {
                    return Err("empty rank".to_string());
                }
                if s == "10" {
                    return Ok(Ten);
                }
                if s.len() >= 2 {
                    return Err(format!("overlong rank: {}", s));
                }
                let r = s.chars().next().unwrap();
                if let Some(d) = char::to_digit(r, 10) {
                    if d >= 2 {
                        return Ok(Spot(d as u8));
                    }
                }
                Err(format!("invalid rank {}", s))
            }
        }
    }
}

impl PartialOrd<Rank> for Rank {
    fn partial_cmp(&self, other: &Rank) -> Option<Ordering> {
        match (*self, *other) {
            (Ace, Ace) => Some(Equal),
            (Ace, _) => None,
            (_, Ace) => None,
            (c1, c2) => Some(u8::from(c1).cmp(&u8::from(c2))),
        }
    }
}

impl Ord for Rank {
    fn cmp(&self, other: &Rank) -> Ordering {
        u8::from(*self).cmp(&u8::from(*other))
    }
}

/// Playing card with rank and suit.
#[derive(Debug, Clone, Copy, Default)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl TryFrom<&str> for Card {
    type Error = String;

    fn try_from(s: &str) -> Result<Card, String> {
        let ns = s.len();
        if ns < 2 || ns > 3 {
            return Err(format!(
                "invalid card description length: {}",
                s
            ));
        }
        let (rs, ss) = s.split_at(ns - 1);
        let rank = Rank::try_from(rs)?;
        let suit = Suit::try_from(ss)?;
        Ok(Card { rank, suit })
    }
}

impl PartialEq<Card> for Card {
    fn eq(&self, other: &Card) -> bool {
        self.rank == other.rank && self.suit == other.suit
    }
}

impl Eq for Card {}

impl PartialOrd<Card> for Card {
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        self.rank.partial_cmp(&other.rank)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Card) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}
