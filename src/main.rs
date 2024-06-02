use std::fmt;

pub trait Set<T> {
    fn has_element(&self, x: T) -> ElementOf;
    fn include(&self, x: &Self) -> Included;
}

pub struct FiniteSet<T> {
    label: String,
    values: Vec<T>,
}

pub enum ElementOf{
    Yes,
    No
}

pub enum Included{
    Yes,
    No
}

impl fmt::Display for ElementOf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Yes => "∈",
            Self::No => "∉"
        })
    }
}

impl fmt::Display for Included {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Yes => "C",
            Self::No => "Ȼ"
        })
    }
}


impl<T: PartialEq> Set<T> for FiniteSet<T> {
    fn has_element(&self, x: T) -> ElementOf {
        if self.values.contains(&x) {
            return ElementOf::Yes
        }

        ElementOf::No
    }

    fn include(&self, x: &Self) -> Included {
        if x.values.iter().all(|v| self.values.contains(v)) {
            return Included::Yes
        }

        Included::No
    }
}

fn main() {
    let a: FiniteSet<i32> = FiniteSet {
        label: "A".to_string(),
        values: vec![
            1, 2, 3, 4, 5, 6, 7
        ],
    };

    let b = FiniteSet {
        label: "B".to_string(),
        values: vec![1, 2],
    };

    println!("{} {} {}", a.label, a.has_element(4), 4);
    println!("{} {} {}", b.label, b.has_element(10), 10);

    println!("{} {} {}", a.label, a.include(&b), b.label);
    println!("{} {} {}", b.label, b.include(&a), a.label);
}
