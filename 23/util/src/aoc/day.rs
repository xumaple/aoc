use std::fmt::{Display, Debug};

#[derive(Hash, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
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
    D25
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