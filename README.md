`crab_era` computes time relative to the Crab Epoch (the commit of Rust 1.0, a59de37e).

Example:

```rust
use chrono::{
	DateTime,
	Utc,
};

use crab_era::{
	crab,
	to_crab,
};

let t_3339 = "2022-04-14T23:41:20+00:00";
let ndt = DateTime::parse_from_rfc3339 (t_3339).unwrap ().naive_utc ();

let c = to_crab (ndt);
let roundtripped = crab (c).unwrap ();

assert_eq! (c, 218406860);
assert_eq! (DateTime::<Utc>::from_utc (roundtripped, Utc).to_rfc3339 (), t_3339);
```
