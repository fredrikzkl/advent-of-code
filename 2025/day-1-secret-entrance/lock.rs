pub struct Lock {
    val: i16,
    zero_hits: i32,
}

impl Lock {
    pub fn new() -> Self {
        Lock {
            val: 50,
            zero_hits: 0,
        }
    }

    pub fn dial(&mut self, val: &str) {
        let (first, rest) = val.split_at(1);
        let direction = first.chars().next().unwrap();
        let mut number: i16 = rest.parse().unwrap();

        while number > 100 {
            number -= 100;
            self.zero_hits += 1;
        }

        let starts_at_zero = self.val == 0;
        let mut did_hit_zero = false;

        if direction == 'R' {
            did_hit_zero = self.dial_right(number, starts_at_zero);
        }

        if direction == 'L' {
            did_hit_zero = self.dial_left(number, starts_at_zero);
        }

        if self.val == 0 && !starts_at_zero && !did_hit_zero {
            self.zero_hits += 1;
        }

        println!("Dialed {}{}. Current: {}", direction, number, self.val);
    }

    fn dial_right(&mut self, number: i16, starts_at_zero: bool) -> bool {
        self.val = self.val + number;
        if self.val >= 100 {
            self.val -= 100;

            if !starts_at_zero {
                self.zero_hits += 1;
                return true;
            }
        }
        return false;
    }

    fn dial_left(&mut self, number: i16, starts_at_zero: bool) -> bool {
        self.val = self.val - number;
        if self.val < 0 {
            self.val += 100;

            if !starts_at_zero {
                self.zero_hits += 1;
                return true;
            }
        }
        return false;
    }

    pub fn print_password(&self) {
        println!("Password: {}", self.zero_hits);
    }
}
