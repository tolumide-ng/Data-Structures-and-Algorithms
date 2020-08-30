pub fn execute() {
    let mut max_stack = MaxStack::new();
    max_stack.push(6);
    max_stack.push(13);
    max_stack.push(5);
    max_stack.push(1);
    max_stack.push(12);
    max_stack.push(18);
    // println!("the max stack {:#?}", max_stack);
    max_stack.pop();
    println!("the max stack {:#?}", max_stack);
}

#[derive(Debug)]
pub struct MaxStack {
    stack: Vec<i8>,
    max_value: Option<i8>,
}

impl MaxStack {
    pub fn new() -> Self {
        return MaxStack {
            stack: vec![],
            max_value: None,
        };
    }
    pub fn push(&mut self, value: i8) {
        if self.stack.len() == 0 {
            self.max_value = Some(value);
            self.stack.push(value);
        } else if self.max_value.unwrap() < value {
            let new_val = (value * 2) - self.max_value.unwrap();
            self.max_value = Some(value);
            self.stack.push(new_val);
        } else {
            self.stack.push(value);
        }
    }

    pub fn pop(&mut self) {
        let last_val = self.stack[self.stack.len() - 1];
        if last_val <= self.max_value.unwrap() {
            self.stack.pop();
        } else {
            self.max_value = Some((2 * self.max_value.unwrap()) - last_val);
            self.stack.pop();
        }
    }

    pub fn max_value(&self) -> i8 {
        return self.max_value.unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::MaxStack;

    #[test]
    fn test_max_stack() {
        let mut max_stack = MaxStack::new();
        max_stack.push(2);
        max_stack.push(1);
        assert_eq!(max_stack.max_value(), 2);
        max_stack.pop();
        assert_eq!(max_stack.max_value(), 2);
    }
}
