// Concept: for loops with a descending range.
pub fn countdown(start: i32) -> Vec<i32> {
    let mut values = Vec::new();

    for value in (0..=start).rev() {
        values.push(value);
    }

    values
}

// Concept: accumulating totals with a loop and integer arithmetic.
pub fn sum_first_n(n: u32) -> u32 {
    let mut total = 0;

    for number in 1..=n {
        total += number;
    }

    total
}

// Concept: filtering values inside a loop using an if expression.
pub fn even_numbers_up_to(limit: u32) -> Vec<u32> {
    let mut evens = Vec::new();

    for number in 0..=limit {
        if number % 2 == 0 {
            evens.push(number);
        }
    }

    evens
}

// Concept: grouping learning topics into a simple list of ideas to revisit.
pub fn concept_map() -> Vec<&'static str> {
    vec![
        "for loops",
        "while loops",
        "Vec<T>",
        "ownership and borrowing",
        "if expressions",
        "functions",
    ]
}

#[cfg(test)]
mod tests {
    use super::{countdown, even_numbers_up_to, sum_first_n};

    #[test]
    fn countdown_starts_at_the_given_number_and_counts_down_to_zero() {
        assert_eq!(countdown(3), vec![3, 2, 1, 0]);
    }

    #[test]
    fn sum_first_n_adds_numbers_in_order() {
        assert_eq!(sum_first_n(5), 15);
    }

    #[test]
    fn even_numbers_up_to_collects_only_even_values() {
        assert_eq!(even_numbers_up_to(8), vec![0, 2, 4, 6, 8]);
    }
}
