use crate::E;
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use proc_macro2::{Ident, Punct, Spacing, Span, TokenStream};
use quote::{ToTokens, TokenStreamExt};

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum Day {
    D01,
    D02,
    D03,
    D04,
    D05,
    D06,
    D07,
    D08,
    D09,
    D10,
    D11,
    D12,
    D13,
    D14,
    D15,
    D16,
    D17,
    D18,
    D19,
    D20,
    D21,
    D22,
    D23,
    D24,
    D25,
}

impl Day {
    pub fn num_repr(&self) -> &'static str {
        match self {
            Day::D01 => "01",
            Day::D02 => "02",
            Day::D03 => "03",
            Day::D04 => "04",
            Day::D05 => "05",
            Day::D06 => "06",
            Day::D07 => "07",
            Day::D08 => "08",
            Day::D09 => "09",
            Day::D10 => "10",
            Day::D11 => "11",
            Day::D12 => "12",
            Day::D13 => "13",
            Day::D14 => "14",
            Day::D15 => "15",
            Day::D16 => "16",
            Day::D17 => "17",
            Day::D18 => "18",
            Day::D19 => "19",
            Day::D20 => "20",
            Day::D21 => "21",
            Day::D22 => "22",
            Day::D23 => "23",
            Day::D24 => "24",
            Day::D25 => "25",
        }
    }

    pub fn folder_name(&self) -> String {
        format!("d{}", self.num_repr())
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day {}", self.num_repr())
    }
}

impl Debug for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "D{}", self.num_repr())
    }
}

impl ToTokens for Day {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Ident::new("Day", Span::call_site()));
        tokens.append(Punct::new(':', Spacing::Joint));
        tokens.append(Punct::new(':', Spacing::Alone));
        tokens.append(Ident::new(
            format!("D{}", self.num_repr()).as_str(),
            Span::call_site(),
        ))
    }
}

impl FromStr for Day {
    type Err = E;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "01" => Ok(Day::D01),
            "02" => Ok(Day::D02),
            "03" => Ok(Day::D03),
            "04" => Ok(Day::D04),
            "05" => Ok(Day::D05),
            "06" => Ok(Day::D06),
            "07" => Ok(Day::D07),
            "08" => Ok(Day::D08),
            "09" => Ok(Day::D09),
            "10" => Ok(Day::D10),
            "11" => Ok(Day::D11),
            "12" => Ok(Day::D12),
            "13" => Ok(Day::D13),
            "14" => Ok(Day::D14),
            "15" => Ok(Day::D15),
            "16" => Ok(Day::D16),
            "17" => Ok(Day::D17),
            "18" => Ok(Day::D18),
            "19" => Ok(Day::D19),
            "20" => Ok(Day::D20),
            "21" => Ok(Day::D21),
            "22" => Ok(Day::D22),
            "23" => Ok(Day::D23),
            "24" => Ok(Day::D24),
            "25" => Ok(Day::D25),
            _ => {
                println!("Unable to parse {s} into `Day`");
                Err(E::ParseError)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Part {
    A,
    B,
}

impl Part {
    pub fn lower_repr(&self) -> &'static str {
        match self {
            Self::A => "a",
            Self::B => "b",
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::A => "A",
                Self::B => "B",
            }
        )
    }
}

impl ToTokens for Part {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Ident::new("Part", Span::call_site()));
        tokens.append(Punct::new(':', Spacing::Joint));
        tokens.append(Punct::new(':', Spacing::Alone));
        match *self {
            Self::A => tokens.append(Ident::new("A", Span::call_site())),
            Self::B => tokens.append(Ident::new("B", Span::call_site())),
        };
    }
}

impl FromStr for Part {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            println!("Unable to parse {s} into `Part`");
            return Err(E::ParseError);
        }
        match s.chars().next().unwrap().to_ascii_lowercase() {
            'a' => Ok(Self::A),
            'b' => Ok(Self::B),
            _ => {
                println!("Unable to parse {s} into `Part`");
                Err(E::ParseError)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Run {
    pub day: Day,
    pub part: Part,
}

impl Display for Run {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.day.num_repr(), self.part.to_string())
    }
}

impl FromStr for Run {
    type Err = E;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 {
            println!("Unable to parse {s} into `Run` {}", s);
            return Err(E::ParseError);
        }
        Ok(Self {
            day: s[..2].parse()?,
            part: s[2..].parse()?,
        })
    }
}
