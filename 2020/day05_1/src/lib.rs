use std::str::FromStr;

enum RowDigit {
    F,
    B,
}

impl RowDigit {
    pub fn from_char(c: char) -> Result<Self, &'static str> {
        match c {
            'F' => Ok(Self::F),
            'B' => Ok(Self::B),
            _ => Err("Could not parse RowDigit"),
        }
    }
}

enum ColumnDigit {
    L,
    R,
}

impl ColumnDigit {
    pub fn from_char(c: char) -> Result<Self, &'static str> {
        match c {
            'L' => Ok(Self::L),
            'R' => Ok(Self::R),
            _ => Err("Could not parse ColumnDigit"),
        }
    }
}

struct BinaryBoarding {
    row: Vec<RowDigit>,
    column: Vec<ColumnDigit>,
}

impl FromStr for BinaryBoarding {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut r_digits = Vec::new();
        let mut c_digits = Vec::new();

        for c in s[..7].chars() {
            r_digits.push(RowDigit::from_char(c)?);
        }
        for c in s[7..].chars() {
            c_digits.push(ColumnDigit::from_char(c)?);
        }

        Ok(Self {
            row: r_digits,
            column: c_digits,
        })
    }
}

impl BinaryBoarding {
    pub fn numeric_row(&self) -> u32 {
        let mut l = 0;
        let mut h = 127;
        let mut last = 0;
        for rows in self.row.iter() {
            match rows {
                RowDigit::F => {
                    h = l + (h - l) / 2;
                    last = h;
                }
                RowDigit::B => {
                    l = l + (h - l) / 2 + 1;
                    last = l;
                }
            }
        }
        last
    }

    pub fn numeric_column(&self) -> u32 {
        let mut l = 0;
        let mut h = 7;
        let mut last = 0;
        for column in self.column.iter() {
            match column {
                ColumnDigit::L => {
                    h = l + (h - l) / 2;
                    last = h;
                }
                ColumnDigit::R => {
                    l = l + (h - l) / 2 + 1;
                    last = l;
                }
            }
        }
        last
    }
}

pub fn solve(input: &str) -> u32 {
    input
        .trim_end()
        .split("\n")
        .map(|x| {
            let p = BinaryBoarding::from_str(x).expect("failed to parse pass");
            return p.numeric_row() * 8 + p.numeric_column();
        })
        .max()
        .expect("failure")
}

#[test]
fn example_row() {
    let input = "FBFBBFFRLR";
    let bb = BinaryBoarding::from_str(input).expect("failure");
    let result = bb.numeric_row();
    let expected = 44;

    assert_eq!(expected, result);
}

#[test]
fn example_column() {
    let input = "FBFBBFFRLR";
    let bb = BinaryBoarding::from_str(input).expect("failure");
    let result = bb.numeric_column();
    let expected = 5;

    assert_eq!(expected, result);
}

#[test]
fn examples() {
    let input = "\
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";
    let expected = 820;
    let result = solve(input);
    assert_eq!(expected, result);
}
