pub fn coins_possible(total: u64, coins: &[u64]) -> u64 {
    let coin = coins[0];

    if coins.len() == 1 {
        let divisible = (total % coin) == 0;
        return if divisible { 1 } else { 0 };
    }
    (0..=(total / coin))
        .map(|n| {
            let left = total - n * coin;
            if left == 0 {
                1
            } else {
                coins_possible(left, &coins[1..])
            }
        })
        .sum::<u64>()
}
