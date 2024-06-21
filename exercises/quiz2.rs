// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// No hints this time!

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // Define the transformer parameters and return type
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        /// Transforms a list of strings according to specified commands.
        ///
        /// # Arguments
        /// - `input`: Vector of tuples, each containing a string and a command to apply.
        ///
        /// The commands can:
        /// - Uppercase the string
        /// - Trim the string
        /// - Append "bar" to the string a specified number of times
        ///
        /// # Returns
        /// A vector of strings, each modified according to its corresponding command.
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            match command {
                Command::Uppercase => {
                    output.push(string.to_uppercase());
                }
                Command::Trim => {
                    output.push(string.trim().to_string());
                }
                Command::Append(n) => {
                    let mut appended = string.clone();
                    for _ in 0..*n {
                        appended.push_str("bar");
                    }
                    output.push(appended);
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // Use 'super::*' to bring the functions from the parent module into scope.
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
