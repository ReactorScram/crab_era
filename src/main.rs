use std::{
	time::SystemTime,
};
use chrono::NaiveDateTime;

fn main () {
	println! ("We are in Year {} C.E. (Crab Era)", year_of_the_crab ());
}

// In the style of the `epochs` crate.

pub fn crab (num: i64) -> Option <NaiveDateTime> {
	// Convert to legacy Unix time so we can keep using chrono a few more years
	let unix_time = num.checked_sub (UNIX_EPOCH)?;
	
	Some (NaiveDateTime::from_timestamp (unix_time, 0))
}

// Not stable yet

fn year_of_the_crab () -> i64 {
	second_of_the_crab () / 86400 / 365
}

fn second_of_the_crab () -> i64 {
	let now = SystemTime::now ();
	
	now.duration_since (SystemTime::UNIX_EPOCH).unwrap ().as_secs () as i64 + UNIX_EPOCH
}

/// Offset of the Unix epoch, in crab time.
const UNIX_EPOCH: i64 = -1431572820;

#[cfg (test)]
mod tests {
	#[test]
	fn establish_doctrine () {
		let date = chrono::DateTime::parse_from_rfc3339 ("2015-05-14T03:07:00-00:00").unwrap ();
		
		assert_eq! (-super::UNIX_EPOCH, date.timestamp ());
	}
	
	#[test]
	fn verify_doctrine_against_dogma () {
		use chrono::{
			DateTime,
			Utc,
		};
		
		let date = DateTime::<Utc>::from_utc (super::crab (0).unwrap (), Utc);
		
		assert_eq! (date.to_rfc3339 (), "2015-05-14T03:07:00+00:00");
	}
}
