use chrono::{
	NaiveDateTime,
};

// In the style of the `epochs` crate.

pub fn crab (num: i64) -> Option <NaiveDateTime> {
	// Convert to legacy Unix time so we can keep using chrono a few more years
	let unix_time = num.checked_sub (UNIX_EPOCH)?;
	
	Some (NaiveDateTime::from_timestamp (unix_time, 0))
}

/// Convert the given NaiveDateTime to Crab Era.

pub fn to_crab (ndt: NaiveDateTime) -> i64 {
	ndt.timestamp () + UNIX_EPOCH
}

/// Offset of the Unix epoch, in crab time.
/// Citation: https://github.com/rust-lang/rust/releases/tag/1.0.0
/// https://github.com/rust-lang/rust/commit/a59de37e99060162a2674e3ff45409ac73595c0e

const UNIX_EPOCH: i64 = -1431572820;

#[cfg (test)]
mod tests {
	use super::{
		crab,
	};
	
	#[test]
	fn establish_doctrine () {
		let date = chrono::DateTime::parse_from_rfc3339 ("2015-05-14T03:07:00-00:00").unwrap ();
		
		assert_eq! (-super::UNIX_EPOCH, date.timestamp ());
	}
	
	#[test]
	fn verify_doctrine_against_itself () {
		use chrono::{
			DateTime,
			Utc,
		};
		
		let date = DateTime::<Utc>::from_utc (crab (0).unwrap (), Utc);
		
		assert_eq! (date.to_rfc3339 (), "2015-05-14T03:07:00+00:00");
		
		
	}
}
