#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    x: i32,
    y: i32,
    z: i32,
}

impl Node {
    fn new(x:i32, y: i32, z: i32) ->  Node {
        return Node { x, y, z }
    }

    fn new_default() -> Node {
        return  Node { x: 0, y: 0, z: 0 }
    }
}

impl Node {
    fn calculate_sum(&self) -> i32 {
        return self.x + self.y + self.z
    }

    fn calculate_product(&self) -> i32 {
        return self.x * self.y * self.z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let node = Node::new(1, 2, 3);
        assert_eq!(node.x, 1);
        assert_eq!(node.y, 2);
        assert_eq!(node.z, 3);
    }

    #[test]
    fn test_new_default() {
        let node = Node::new_default();
        assert_eq!(node.x, 0);
        assert_eq!(node.y, 0);
        assert_eq!(node.x, 0);
    }

    #[test]
    fn test_product(){
        let n = Node::new(1, 2, 3);
        let product = n.calculate_product();
        assert_eq!(product, 6);
    }

    #[test]
    fn test_sum(){
        let n = Node::new(1, 2, 3);
        let sum = n.calculate_sum();
        assert_eq!(sum, 6)
    }
}