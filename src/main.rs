use std::fmt;

pub trait Set<T> {
    fn has_element(&self, x: T) -> ElementOf;
    fn include(&self, x: &Self) -> Included;
    fn union(&self, x: &Self) -> Self;
    fn intersect(&self, x: &Self) -> Self;
}

pub struct FiniteSet<T> {
    label: String,
    values: Vec<T>,
}

pub enum ElementOf{
    Yep,
    Nop
}

pub enum Included{
    Yep,
    Nop
}

impl fmt::Display for ElementOf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Yep => "∈",
            Self::Nop => "∉"
        })
    }
}

impl fmt::Display for Included {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Yep => "C",
            Self::Nop => "Ȼ"
        })
    }
}


impl<T: PartialEq + Clone> Set<T> for FiniteSet<T> {
    fn has_element(&self, x: T) -> ElementOf {
        if self.values.contains(&x) {
            return ElementOf::Yep
        }

        ElementOf::Nop
    }

    fn include(&self, x: &Self) -> Included {
        if x.values.iter().all(|v| self.values.contains(v)) {
            return Included::Yep
        }

        Included::Nop
    }

    fn union(&self, x: &Self) -> Self {
        let mut values = self.values.clone();
        let x_values: Vec<T> = x.values.iter().cloned().filter(|v| !values.contains(v)).collect();
        
        values.extend(x_values);
        FiniteSet {
            label: format!("{} ∪ {}", self.label, x.label),
            values,
        }
    }

    fn intersect(&self, x: &Self) -> Self {
        FiniteSet {
            label: format!("{} ∪ {}", self.label, x.label),
            values: x.values.iter().cloned().filter(|v| self.values.contains(v)).collect(),
        }
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
        values: vec![1, 2, 8],
    };

    println!("{} {} {}", a.label, a.has_element(4), 4);
    println!("{} {} {}", b.label, b.has_element(10), 10);

    println!("{} {} {}", a.label, a.include(&b), b.label);
    println!("{} {} {}", b.label, b.include(&a), a.label);

    println!("{} -> {:?}", a.union(&b).label, a.union(&b).values);
    println!("{} -> {:?}", b.intersect(&a).label, b.intersect(&a).values);
}
