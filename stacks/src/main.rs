/// A Stack is defined here.
/// Currently limited to a maximum of 256 elements
pub struct Stack {
    /// We always keep the next element at TOP
    /// // index access should be of type usize
    top: usize,

    /// The actual stack storage
    /// // the size must be known before hand (Vec would be a good idea here)
    stk: [char; 256],
}

impl Stack {
    /// Creates a new Stack
    /// TODO - be able ot specify the size (or even better, it should grow as needed)
    fn new() -> Stack {
        Stack {
            top: 0,
            stk: [' '; 256],
        }
    }

    /// Push a new element to the top
    /// Does not return anything
    pub fn push(&mut self, element: char) {
        if self.top == 256 {
            panic!("Stack full")
        }
        self.stk[self.top] = element;
        self.top += 1;
    }

    /// Pop the top element
    pub fn pop(&mut self) -> char {
        if self.top == 0 {
            panic!("Stack empty")
        }
        self.top -= 1;
        self.stk[self.top]
    }
}

fn is_palindrome(s: String) -> bool {

    let l = s.chars().count() ;

    let mut st = Stack::new();
    let mut i = 0;
    for c in s.chars(){

        if i < l/2 {
            st.push(c);
            i += 1;
            continue
        }

        // The middle can be skipped
        if i == l/2  && l % 2 == 1 {
            i += 1;
            continue;
        }

        // i += 1 -- need not really add
        let ch = st.pop();
        if c != ch {
            return false
        }
    }

    return true
}

fn is_matched_brackets(s: String) -> bool {
    return false // Please implement
}

fn main() {
    println!(")) is a palindrome: {}", is_palindrome("))".to_string()));
    println!("malayalam is a palindrome: {}", is_palindrome("malayalam".to_string()));
    println!("helloworld is a palindrome: {}", is_palindrome("helloworld".to_string()));
    println!("helloolleh is a palindrome: {}", is_palindrome("helloolleh".to_string()));
    println!("()() is a palindrome: {}", is_palindrome("()()".to_string()));
}
