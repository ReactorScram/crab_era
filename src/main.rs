use chrono::{
	NaiveDateTime,
	Utc,
};
use crab_era::*;

fn main () {
	let now = Utc::now ().naive_utc ();
	let c = to_crab (now);
	let year = year_of_the_crab (now);
	
	println! ("The Crab Epoch is {}", c);
	println! ("We are in Year {} C.E. (Crab Era)", year);
	
	match year {
		6 => println! ("Attend eagerly Year 7 C.E., on 2022-05-12T03:07:00-00:00!!"),
		7 => {
			println! ("Happy Year 7 C.E.!");
			println! ("Attend eagerly Year 8 C.E., on 2023-05-12T03:07:00-00:00!!");
		},
		8 => {
			println! ("Happy Year 8 C.E.!");
			println! ("Do not attend anything eagerly!");
		}
		_ => {},
	}
}

fn year_of_the_crab (ndt: NaiveDateTime) -> i64 {
	to_crab (ndt) / 86400 / 365
}

#[cfg (test)]
mod tests {
	#[test]
	fn reckon_by_the_stars () {
		for (input, expected) in [
			("2022-05-12T03:06:00-00:00", 6),
			("2022-05-12T03:08:00-00:00", 7),
			
			("2023-05-12T03:06:00-00:00", 7),
			("2023-05-12T03:08:00-00:00", 8),
		].into_iter () {
			let input = chrono::DateTime::parse_from_rfc3339 (input).unwrap ()
			.naive_utc ();
			assert_eq! (super::year_of_the_crab (input), expected);
		}
	}
}
