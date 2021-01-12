
	/*This function divides a by greatest
	divisible power of b*/
	fn max_divide(mut a: u32, b: u32) -> u32 {
		while a % b == 0 {
			a = a/b;
		}
		return a;
	}

	/* Function to check if a number
	is ugly or not */
	fn is_ugly(mut no: u32) -> bool	{
		no = max_divide(no, 2);
		no = max_divide(no, 3);
		no = max_divide(no, 5);

		return no == 1;
	}

	/* Function to get the nth ugly
	number*/
	fn get_nth_ugly_no(n: u32) ->	u32 {
		let mut i = 1;

		// ugly number count
		let mut count = 1;

		// check for all integers
		// until count becomes n
		while n > count {
			i+=1;
			if is_ugly(i) == true {
				count+=1;
			}
		}
		return i;
	}

	/* Driver program to test above
	functions */
	fn main() {
		let no = get_nth_ugly_no(10);
		println!("10th ugly no. is {}",  no);
	}
