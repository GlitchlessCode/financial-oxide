use std::{
    error::Error,
    fmt::{Debug, Display},
    iter::Peekable,
    str::FromStr,
};

use colored::Colorize;

#[derive(Debug)]
enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    fn new_from(
        value: u8,
        from: usize,
        to: usize,
        src: &String,
    ) -> Result<Self, DateConversionError> {
        use Month::*;
        Ok(match value {
            1 => January,
            2 => February,
            3 => March,
            4 => April,
            5 => May,
            6 => June,
            7 => July,
            8 => August,
            9 => September,
            10 => October,
            11 => November,
            12 => December,
            _ => {
                return Err(DateConversionError {
                    src: src.clone(),
                    reason: DateConversionErrorReason::FailedParse {
                        from,
                        to,
                        failed_type: std::any::type_name::<Self>(),
                    },
                })
            }
        })
    }

    fn to_numeric(&self) -> u8 {
        use Month::*;
        match self {
            &January => 1,
            &February => 2,
            &March => 3,
            &April => 4,
            &May => 5,
            &June => 6,
            &July => 7,
            &August => 8,
            &September => 9,
            &October => 10,
            &November => 11,
            &December => 12,
        }
    }
}

impl Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Month::*;
        write!(
            f,
            "{}",
            match self {
                &January => "January",
                &February => "February",
                &March => "March",
                &April => "April",
                &May => "May",
                &June => "June",
                &July => "July",
                &August => "August",
                &September => "September",
                &October => "October",
                &November => "November",
                &December => "December",
            }
        )
    }
}

#[derive(Debug)]
pub struct Date {
    year: u16,
    month: Month,
    day: u8,
}

#[derive(Debug)]
enum DateConversionErrorReason {
    InvalidInput {
        at: usize,
    },
    FailedParse {
        from: usize,
        to: usize,
        failed_type: &'static str,
    },
    BadDayBounds {
        failed_date: Date,
    },
}

impl DateConversionErrorReason {
    fn code(&self) -> u8 {
        use DateConversionErrorReason::*;
        match self {
            InvalidInput { .. } => 0,
            FailedParse { .. } => 1,
            BadDayBounds { .. } => 2,
        }
    }

    fn name(&self) -> &'static str {
        use DateConversionErrorReason::*;
        match self {
            InvalidInput { .. } => "Invalid input",
            FailedParse { .. } => "Failed to parse",
            BadDayBounds { .. } => "Day out of bounds",
        }
    }
}

pub struct DateConversionError {
    src: String,
    reason: DateConversionErrorReason,
}

impl Display for DateConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use DateConversionErrorReason::*;
        write!(f, "\n")?;

        f.write_str(
            &format!("[E{:0>2}] Error: ", self.reason.code())
                .red()
                .bold()
                .to_string()
                .as_str(),
        )?;

        f.write_str(
            &format!("{0}\n\n  ", self.reason.name())
                .bold()
                .to_string()
                .as_str(),
        )?;

        match self.reason {
            InvalidInput { at } => {
                f.write_str(self.src.chars().take(at).collect::<String>().as_str())?;
                f.write_str(
                    &self
                        .src
                        .chars()
                        .skip(at)
                        .collect::<String>()
                        .red()
                        .to_string()
                        .as_str(),
                )?;

                let error_width = {
                    let count = self.src.chars().skip(at).count();
                    if count > 0 {
                        count - 1
                    } else {
                        0
                    }
                };
                let left_half = (error_width) / 2;
                let right_half = error_width - left_half;
                f.write_str(
                    format!(
                        "\n  {0}{1}┬{2}",
                        std::iter::repeat(' ').take(at).collect::<String>(),
                        std::iter::repeat('─').take(left_half).collect::<String>(),
                        std::iter::repeat('─').take(right_half).collect::<String>()
                    )
                    .red()
                    .to_string()
                    .as_str(),
                )?;

                f.write_str(
                    format!(
                        "\n  {0}╰{1} ",
                        std::iter::repeat(' ')
                            .take(at + left_half)
                            .collect::<String>(),
                        std::iter::repeat('─')
                            .take(right_half + 1)
                            .collect::<String>()
                    )
                    .red()
                    .to_string()
                    .as_str(),
                )?;

                f.write_str("Invalid input past this point at index ")?;

                f.write_str(at.to_string().red().bold().to_string().as_str())?;
            }
            FailedParse {
                from,
                to,
                failed_type,
            } => {
                {
                    f.write_str(self.src.chars().take(from).collect::<String>().as_str())?;
                    f.write_str(
                        &self
                            .src
                            .chars()
                            .skip(from)
                            .take(to - from)
                            .collect::<String>()
                            .red()
                            .to_string()
                            .as_str(),
                    )?;
                    f.write_str(self.src.chars().skip(to).collect::<String>().as_str())?;

                    let error_width = {
                        let count = to - from;
                        if count > 0 {
                            count - 1
                        } else {
                            0
                        }
                    };
                    let left_half = (error_width) / 2;
                    let right_half = error_width - left_half;
                    f.write_str(
                        format!(
                            "\n  {0}{1}┬{2}",
                            std::iter::repeat(' ').take(from).collect::<String>(),
                            std::iter::repeat('─').take(left_half).collect::<String>(),
                            std::iter::repeat('─').take(right_half).collect::<String>()
                        )
                        .red()
                        .to_string()
                        .as_str(),
                    )?;

                    f.write_str(
                        format!(
                            "\n  {0}╰{1} ",
                            std::iter::repeat(' ')
                                .take(from + left_half)
                                .collect::<String>(),
                            std::iter::repeat('─')
                                .take(right_half + 1)
                                .collect::<String>()
                        )
                        .red()
                        .to_string()
                        .as_str(),
                    )?;

                    f.write_str("Could not parse ")?;

                    f.write_str(failed_type.green().bold().to_string().as_str())?;

                    f.write_str(" from ")?;

                    f.write_str(
                        &self
                            .src
                            .chars()
                            .skip(from)
                            .take(to - from)
                            .collect::<String>()
                            .red()
                            .bold()
                            .to_string()
                            .as_str(),
                    )?;
                };
            }
            BadDayBounds { ref failed_date } => {
                let Date { year, month, day } = failed_date;

                f.write_str(self.src.red().to_string().as_str())?;

                f.write_str("\n\n  Day ")?;

                f.write_str(day.to_string().red().bold().to_string().as_str())?;

                f.write_str(" is not within bounds of ")?;

                f.write_str(
                    format!("{0}, {1}", month, year)
                        .green()
                        .bold()
                        .to_string()
                        .as_str(),
                )?;
            }
        }

        write!(f, "\n")?;

        Ok(())
    }
}

impl Debug for DateConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use DateConversionErrorReason::*;
        writeln!(
            f,
            "{} [in '{}']: {}",
            self.reason.name(),
            &self.src,
            match self.reason {
                InvalidInput { at } => format!(
                    "Invalid past index {at} [at '{}']",
                    self.src.chars().skip(at).collect::<String>()
                ),
                FailedParse {
                    from,
                    to,
                    failed_type,
                } => format!(
                    "Could not parse type '{failed_type}' [at '{}']",
                    self.src
                        .chars()
                        .skip(from)
                        .take(to - from)
                        .collect::<String>()
                ),
                BadDayBounds {
                    failed_date:
                        Date {
                            ref year,
                            ref month,
                            ref day,
                        },
                } => format!("Day '{day}' out of bounds [in {month}, {year}]"),
            }
        )
    }
}

impl Error for DateConversionError {}

impl TryFrom<String> for Date {
    type Error = DateConversionError;
    fn try_from(src: String) -> Result<Self, Self::Error> {
        // Get peekable enumerated characters of src
        let mut input_iter = src.chars().enumerate().peekable();

        // Try to get year, month and day from src
        let year: u16 = expect_number(&mut input_iter, 4, &src)?;

        expect_hyphen(&mut input_iter, &src)?;

        let month = expect_number(&mut input_iter, 2, &src)?;
        let current_idx = match input_iter.peek() {
            Some((idx, _)) => *idx,
            None => src.chars().count(),
        };
        let month = Month::new_from(month, current_idx - 2, current_idx, &src)?;

        expect_hyphen(&mut input_iter, &src)?;

        let day: u8 = expect_number(&mut input_iter, 2, &src)?;

        // Expect end of input
        expect_eoi(&mut input_iter, &src)?;

        // Validation check
        let day_count = days_in_month_by_year(&month, year);

        // Create date
        let date = Date { year, month, day };

        if day > 0 && day <= day_count {
            Ok(date)
        } else {
            Err(Self::Error {
                src,
                reason: DateConversionErrorReason::BadDayBounds { failed_date: date },
            })
        }
    }
}

impl Date {
    pub fn to_pretty(&self) -> String {
        format!("{} {}, {}", self.month, self.day, self.year)
    }

    pub fn to_numeric(&self) -> String {
        format!(
            "{:0>4}-{:0>2}-{:0>2}",
            self.year,
            self.month.to_numeric(),
            self.day
        )
    }
}

fn expect_number<T: FromStr>(
    iter: &mut Peekable<impl Iterator<Item = (usize, char)>>,
    len: usize,
    src: &String,
) -> Result<T, DateConversionError> {
    let mut num_str = String::new();
    let init_idx = match iter.peek() {
        Some((idx, _)) => *idx,
        None => {
            return Err(DateConversionError {
                src: src.clone(),
                reason: DateConversionErrorReason::InvalidInput {
                    at: src.chars().count(),
                },
            })
        }
    };
    for _ in 0..len {
        match iter.next() {
            Some((at, c)) => match c {
                '0'..='9' => num_str.push(c),
                _ => {
                    return Err(DateConversionError {
                        src: src.clone(),
                        reason: DateConversionErrorReason::InvalidInput { at },
                    })
                }
            },
            None => {
                return Err(DateConversionError {
                    src: src.clone(),
                    reason: DateConversionErrorReason::InvalidInput {
                        at: src.chars().count(),
                    },
                })
            }
        }
    }

    match num_str.parse::<T>() {
        Ok(result) => Ok(result),
        Err(_) => Err(DateConversionError {
            src: src.clone(),
            reason: DateConversionErrorReason::FailedParse {
                from: init_idx,
                to: init_idx + len,
                failed_type: std::any::type_name::<T>(),
            },
        }),
    }
}

fn expect_hyphen(
    iter: &mut impl Iterator<Item = (usize, char)>,
    src: &String,
) -> Result<(), DateConversionError> {
    match iter.next() {
        Some((_, '-')) => Ok(()),
        Some((at, _)) => Err(DateConversionError {
            src: src.clone(),
            reason: DateConversionErrorReason::InvalidInput { at },
        }),
        None => Err(DateConversionError {
            src: src.clone(),
            reason: DateConversionErrorReason::InvalidInput {
                at: src.chars().count(),
            },
        }),
    }
}

fn expect_eoi(
    iter: &mut impl Iterator<Item = (usize, char)>,
    src: &String,
) -> Result<(), DateConversionError> {
    match iter.next() {
        Some((at, _)) => Err(DateConversionError {
            src: src.clone(),
            reason: DateConversionErrorReason::InvalidInput { at },
        }),
        None => Ok(()),
    }
}

fn days_in_month_by_year(month: &Month, year: u16) -> u8 {
    use Month::*;
    let leap_year = is_leap_year(year);

    match month {
        &January => 31,
        &February if leap_year == true => 28,
        &February => 29,
        &March => 31,
        &April => 30,
        &May => 31,
        &June => 30,
        &July => 31,
        &August => 31,
        &September => 30,
        &October => 31,
        &November => 30,
        &December => 31,
    }
}

#[inline]
fn is_leap_year(year: u16) -> bool {
    ((year % 4 == 0) && (year % 100 != 0)) || (year % 400 == 0)
}
