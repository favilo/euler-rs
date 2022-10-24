#[tracing::instrument]
pub fn fibonacci() -> impl Iterator<Item = u64> {
    itertools::unfold((1, 2), |(x1, x2)| {
        let next = *x1 + *x2;

        let ret = *x1;
        *x1 = *x2;
        *x2 = next;
        Some(ret)
    })
}

#[tracing::instrument]
pub fn primes(max: u64) -> impl Iterator<Item = u64> {
    let to_check: Vec<u64> = (2..=max).rev().collect();
    itertools::unfold(Box::new(to_check), |list| {
        let head = list.pop()?;
        list.retain(|&n| n % head != 0);
        Some(head)
    })
}
