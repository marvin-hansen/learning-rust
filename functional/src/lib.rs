#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// we call into_iter to create an iterator that takes ownership of the vector.
// Then we call filter to adapt that iterator into a new iterator
// that only contains elements for which the closure returns true.
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {

        let shoes = get_sample_shoes();
        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(in_my_size,  vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]);

    }

    fn get_sample_shoes() -> Vec<Shoe> {
        return vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];
    }


}