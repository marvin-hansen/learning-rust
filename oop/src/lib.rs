
// The struct is marked pub so that other code can use it,
// but the fields within the struct remain private.
// This is important in this case because we want to ensure that
// whenever a value is added or removed from the list, the average is also updated.
pub struct AveragedCollection{
    list: Vec<i32>,
    average: f64
}

// The public methods add, remove, and average are the only ways to access or modify data
// in an instance of AveragedCollection.
// When an item is added to list using the add method or removed using the remove method,
// the implementations of each call the private update_average method
// that handles updating the average field as well.
impl AveragedCollection {

    pub fn new() -> AveragedCollection {
        AveragedCollection{
            list: vec![],
            average: 0.0
        }
    }

    pub fn add(&mut self, value: i32){
        self.list.push(value);
        self.update_average();
    }

    pub fn length(&self) -> usize {
        return self.list.len()
    }

    pub fn remove_last(&mut self) -> Option<i32>{
        let result = self.list.pop();
        return match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        return self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Because we’ve encapsulated the implementation details of the struct AveragedCollection,
    // we can easily change aspects, such as the data structure, in the future.
    // For instance, we could use a HashSet<i32> instead of a Vec<i32> for the list field.
    // As long as the signatures of the add, remove, and average public methods stay the same,
    // code using AveragedCollection would not need to change as seen in the tests below.

    #[test]
    fn test_add() {
        let mut ac = AveragedCollection::new();

        ac.add(1);
        assert_eq!(ac.length(), 1);

        ac.add(2);
        assert_eq!(ac.length(), 2);

        ac.add(3);
        assert_eq!(ac.length(), 3);
    }

    #[test]
    fn test_average() {
        let mut ac = AveragedCollection::new();

        ac.add(1);
        assert_eq!(ac.average(), 1.0);

        ac.add(2);
        assert_eq!(ac.average(), 1.5);

        ac.add(3);
        assert_eq!(ac.average(), 2.0);
    }

    fn test_remove_last() {
        let mut ac = AveragedCollection::new();
        ac.add(1);
        assert_eq!(ac.length(), 1);
        assert_eq!(ac.average(), 1.0);

        ac.add(2);
        assert_eq!(ac.length(), 2);
        assert_eq!(ac.average(), 1.5);

        let last = ac.remove_last();
        assert_eq!(last.unwrap(), 2);
        assert_eq!(ac.length(), 1);

        let last = ac.remove_last();
        assert_eq!(last.unwrap(), 1);
        assert_eq!(ac.length(), 0);

        let last = ac.remove_last();
        assert_eq!(last, None);
        assert_eq!(ac.length(), 0);
        assert_eq!(ac.average(), 0.0);
    }
}
