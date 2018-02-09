// Original ource: https://github.com/eliovir/rust-examples/blob/master/fibonacci.rs

fn main() {
	println!("{:?}", fibonacci_reccursive(40));
}

#[no_mangle]
pub fn fibonacci_reccursive(n: i32) -> i32 {
	if n < 0 {
		panic!("{} is negative!", n);
	}
	match n {
		0     => panic!("zero is not a right argument to fibonacci_reccursive()!"),
		1 | 2 => 1,
		3     => 2,
		/*
		50    => 12586269025,
		*/
		_     => fibonacci_reccursive(n - 1) + fibonacci_reccursive(n - 2)
	}
}

/**
 * Non reccursive function.
 */
 #[no_mangle]
pub fn fibonacci(n: i32) -> i32 {
	if n < 0 {
		panic!("{} is negative!", n);
	} else if n == 0 {
		panic!("zero is not a right argument to fibonacci()!");
	} else if n == 1 {
		return 1;
	}
	let mut i = 0;
	let mut sum = 0;
	let mut last = 0;
	let mut curr = 1;
	while i < n - 1 {
		sum = last + curr;
		last = curr;
		curr = sum;
		i += 1;
	}
    sum    
}

/**
 * Iterative fibonacci
 * https://github.com/rust-lang/rust-by-example
 */
pub struct Fibonacci {
	curr: u64,
	next: u64,
}

impl Iterator for Fibonacci {
	type Item = u64;
	fn next(&mut self) -> Option<u64> {
		let new_next = self.curr + self.next;

		self.curr = self.next;
		self.next = new_next;

		Some(self.curr)
	}
}

/**
 * A "constructor" for Iiterative fibonacci
 */
#[no_mangle]
pub fn iterative_fibonacci() -> Fibonacci {
	Fibonacci { curr: 1, next: 1 }
}
