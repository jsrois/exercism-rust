#[derive(Debug, Eq)]
pub struct CustomSet<T> {
    values: Vec<T>,
}

impl<T: std::cmp::PartialEq> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.values.len() == other.values.len()
            && self.values.iter().all(|x| other.values.contains(x))
    }
}
impl<'a, T: std::cmp::PartialEq + std::clone::Clone + std::cmp::Ord> CustomSet<T> {
    pub fn new(input: &'a [T]) -> Self {
        let mut v: Vec<T> = Vec::from(input);
        v.sort();
        v.dedup();
        CustomSet { values: v }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.values.contains(element)
    }

    pub fn add(&mut self, element: T) {
        if !self.values.contains(&element) {
            self.values.push(element);
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.values.iter().all(|x| other.contains(x))
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        !self.values.iter().any(|x| other.contains(x))
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        Self {
            values: other
                .values
                .iter()
                .filter(|x| self.values.contains(x))
                .cloned()
                .collect(),
        }
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        Self {
            values: self
                .values
                .iter()
                .filter(|x| !other.values.contains(x))
                .cloned()
                .collect(),
        }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut values = self.values.clone();
        for x in &other.values {
            if !values.contains(x) {
                values.push(x.clone());
            }
        }
        Self { values }
    }
}
