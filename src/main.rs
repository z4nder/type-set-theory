use std::fmt;

pub trait Set<T> {
    fn contains(&self, x: &Self) -> Relationship;
}

pub struct FiniteSet<T> {
    label: String,
    values: Vec<T>,
}

pub enum Relationship{
    Contains,
    NotContains
}

impl fmt::Display for Relationship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Relationship::Contains => "∈",
            Relationship::NotContains => "∉"
        })
    }
}

impl<T: PartialEq> Set<T> for FiniteSet<T> {
    fn contains(&self, x: &Self) -> Relationship {
        if x.values.iter().all(|v| self.values.contains(v)) {
            return Relationship::Contains
        }

        Relationship::NotContains
    }
}

fn main() {
    let a: FiniteSet<i32> = FiniteSet {
        label: "A".to_string(),
        values: vec![1, 2, 3, 4, 5, 6, 7],
    };

    let b = FiniteSet {
        label: "B".to_string(),
        values: vec![1, 2],
    };

    println!("{} {} {}", a.label, a.contains(&b), b.label);
    println!("{} {} {}", b.label, b.contains(&a), a.label);
}
