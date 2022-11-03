use primes::is_prime;

const ONES: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const TEENS: [&str; 10] = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: [&str; 9] = [
    "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

pub fn number_to_words(n: u64) -> String {
    match n {
        _ones if n < 10 => format!("{}", ONES[n as usize]),
        _teens if n < 20 => {
            let ones = TEENS[(n as usize % 10)];
            format!("{}", ones)
        }
        _tens if n < 100 => {
            let ones = n as usize % 10;
            let ones = if ones == 0 { "" } else { ONES[ones] };
            let tens = n as usize / 10;
            let tens = TENS[tens - 1];
            format!("{}{}", tens, ones)
        }
        _hunds if n < 1000 => {
            let hunds = ONES[n as usize / 100];
            let small = if n % 100 == 0 {
                "".into()
            } else {
                format!("and{}", number_to_words(n % 100))
            };
            format!("{}hundred{}", hunds, small)
        }
        _thou if n == 1000 => "onethousand".into(),
        _ => panic!("Don't support those numbers"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ones() {
        (1..10usize).for_each(|n: usize| assert_eq!(&number_to_words(n as u64), ONES[n]));
    }

    #[test]
    fn teens() {
        (10..20).for_each(|n: usize| assert_eq!(&number_to_words(n as u64), TEENS[n - 10]));
    }

    #[test]
    fn random_tens() {
        assert_eq!(number_to_words(22), "twentytwo");
        assert_eq!(number_to_words(33), "thirtythree");
        assert_eq!(number_to_words(99), "ninetynine");
    }

    #[test]
    fn random_hunds() {
        assert_eq!(number_to_words(123), "onehundredandtwentythree");
        assert_eq!(number_to_words(523), "fivehundredandtwentythree");
        assert_eq!(number_to_words(547), "fivehundredandfortyseven");
    }

    #[test]
    fn euler() {
        assert_eq!(number_to_words(342).len(), 23);
        assert_eq!(number_to_words(115).len(), 20);
        assert_eq!(
            (1..=5)
                .map(|n| number_to_words(n).len() as u64)
                .sum::<u64>(),
            19
        );
    }
}

pub fn reciprocal_cycle_length(denom: usize) -> usize {
    let mut remainders = vec![None::<usize>; denom];
    let mut idx = 0;
    let mut check = 10;

    loop {
        let rem = check % denom;
        if rem == 0 {
            return 0;
        }
        if let Some(start) = remainders[rem] {
            return idx - start;
        }
        remainders[rem] = Some(idx);
        idx += 1;
        check = rem * 10;
    }
}

pub fn quadratic_prime_run_length(a: i32, b: i32) -> usize {
    (0i32..)
        .map(|n| n.pow(2) + a * n + b)
        .take_while(|&f| f > 0 && is_prime(f as u64))
        .count()
}
