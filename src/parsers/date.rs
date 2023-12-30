use super::error::ParsingError;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
	pub year: u16,
	pub month: u16,
	pub day: u16,
}

impl Date {
	pub fn new(year: u16, month: u16, day: u16) -> Self {
		Self { year, month, day }
	}

	pub fn parse(date: &str) -> Result<Date, ParsingError> {
		let err = || ParsingError::InvalidDate(date.to_string());
		let (year, date) = date.split_once("-").ok_or_else(err)?;
		let (month, day) = date.split_once("-").ok_or_else(err)?;

		let month = month.parse().map_err(|_| err())?;

		if month < 1 || month > 12 {
			return Err(err());
		}

		Ok(Date {
			year: year.parse().map_err(|_| err())?,
			month,
			day: day.parse().map_err(|_| err())?,
		})
	}

	pub fn pretty_no_day(&self) -> String {
		format!("{} {}", self.month_name(), self.year)
	}

	pub fn pretty(&self) -> String {
		format!("{} {}, {}", self.month_name(), self.day, self.year)
	}

	pub fn iso_8601(&self) -> String {
		format!("{}-{}-{}", self.year, self.month, self.day)
	}

	pub fn rfc2822(&self) -> String {
		let short_month = &self.month_name()[0..3];
		// let short_year = &format!("{:04}", self.year)[2..4];

		format!("{} {} {} 00:00:00 -0700", self.day, short_month, self.year)
	}

	pub fn month_name(&self) -> &'static str {
		match self.month {
			1 => "January",
			2 => "February",
			3 => "March",
			4 => "April",
			5 => "May",
			6 => "June",
			7 => "July",
			8 => "August",
			9 => "September",
			10 => "October",
			11 => "November",
			12 => "December",
			_ => unreachable!(),
		}
	}
}
