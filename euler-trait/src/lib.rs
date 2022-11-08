#[doc(hidden)]
pub use inventory;

pub struct Problems;

impl Problems {
    pub fn get(i: usize) -> Box<dyn Problem> {
        Problems::collect()
            .into_iter()
            .find_map(|p| (p.problem() == i).then_some(p))
            .unwrap_or_else(|| panic!("should have created problem {i}"))
            .clone_box()
    }

    pub fn collect() -> Vec<Box<dyn Problem>> {
        let mut collected: Vec<_> = inventory::iter::<_Problem>
            .into_iter()
            // Safety: Zero sized types???
            .map(|p| unsafe { &*p.0 as &dyn Problem }.clone_box())
            .collect();
        collected.sort_by_key(|p| p.problem());
        collected
    }
}

#[doc(hidden)]
pub struct _Problem(pub *const (dyn Problem + Sync + Send + 'static));

// Safety: These are zero sized types???
unsafe impl Sync for _Problem {}
unsafe impl Send for _Problem {}

inventory::collect!(_Problem);

pub trait Problem: ProblemClone {
    fn solve(&self) -> Box<dyn std::fmt::Display>;
    fn problem(&self) -> usize;
}

pub trait ProblemClone: std::fmt::Debug {
    fn clone_box(&self) -> Box<dyn Problem>;
}

impl<T> ProblemClone for T
where
    T: 'static + Clone + std::fmt::Debug + Problem,
{
    fn clone_box(&self) -> Box<dyn Problem> {
        Box::new(self.clone())
    }
}
