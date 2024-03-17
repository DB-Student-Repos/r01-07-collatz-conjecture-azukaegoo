pub fn collatz(n: u64) -> Option<u64> {
    let mut counting = 0;
    let mut num = n;
    loop {
        if num == 0 {
            return None;
        } else if num == 1 {
            return Some(counting);
        } else if num % 2 == 0 {
            num = num / 2;
        } else {
            // Checking for potential overflow before performing the multiplication
            if let Some(next_num) = num.checked_mul(3).and_then(|x| x.checked_add(1)) {
                num = next_num;
            } else {
                // Returns None if overflow occurs
                return None;
            }
        }
        counting += 1;
    }
}