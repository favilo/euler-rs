use euler_macro::{create_registry, problem};

create_registry!();

#[allow(dead_code)]
#[problem(1)]
fn problem1() -> u64 {
    todo!()
}

#[allow(dead_code)]
#[problem(2)]
fn problem2() -> u64 {
    todo!()
}

fn main() {
    for problem in PROBLEMS::iter() {
        problem.solve();
    }
}
