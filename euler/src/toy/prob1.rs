/// Multiples of 3 and 5
/// Find the sum of all the multiples of 3 or 5 below 1000.
fn threefive (n:i32) -> i32 {
	let mut result:i32 = 0;
	for i in 1..n {
		if i % 3 == 0 || i % 5 == 0 {
			result += i;
		}
	}
	result
}

#[test]
fn euler1 () {
	assert_eq!(threefive(10), 23);
	assert_eq!(threefive(1000), 233168);
}