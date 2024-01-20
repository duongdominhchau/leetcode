pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    if temperatures.len() == 0 {
        return Vec::new();
    }

    let mut out: Vec<i32> = Vec::new();
    out.resize(temperatures.len(), 0);

    let mut stack = vec![0];
    // For each new element
    for current_index in 1..temperatures.len() {
        // Pop out all old elements whose value is lower than current value
        let current = temperatures[current_index];
        while !stack.is_empty() && current > temperatures[*stack.last().unwrap()] {
            let index = stack.pop().unwrap();
            out[index] = (current_index - index) as i32;
        }
        stack.push(current_index);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }
    #[test]
    fn test_2() {
        assert_eq!(daily_temperatures(vec![30, 40, 50, 60]), vec![1, 1, 1, 0]);
    }
    #[test]
    fn test_3() {
        assert_eq!(daily_temperatures(vec![30, 60, 90]), vec![1, 1, 0]);
    }
}
