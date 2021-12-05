use std::collections::HashMap;
use std::hash::Hash;

pub trait IterUtils : Iterator {
    fn counting_set(self) -> HashMap<Self::Item, usize> 
        where Self: Sized, Self::Item: Clone + Eq + Hash
    {
        self.fold(HashMap::new(), |mut map, it| {
			*map.entry(it).or_insert(0) += 1;
			map
		})
    }
}

impl<T: ?Sized> IterUtils for T where T: Iterator { }