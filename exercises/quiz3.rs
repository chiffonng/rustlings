// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

/// 'grade' module contains the Grade trait, which converts a grade to a string
mod grade {
    /// Trait to convert a grade (f32 or &str) to a string
    pub trait Grade {
        /// Method to convert a grade to a string
        fn grade(&self) -> String;
    }

    impl Grade for f32 {
        /// Method to convert a numeric grade to an alphabetic grade
        fn grade(&self) -> String {
            if *self >= 4.5 {
                "A+".to_string()
            } else if *self >= 4.0 {
                "A".to_string()
            } else if *self >= 3.5 {
                "B+".to_string()
            } else if *self >= 3.0 {
                "B".to_string()
            } else if *self >= 2.5 {
                "C+".to_string()
            } else if *self >= 2.0 {
                "C".to_string()
            } else if *self >= 1.5 {
                "D+".to_string()
            } else if *self >= 1.0 {
                "D".to_string()
            } else {
                "F-".to_string()
            }
        }
    }

    impl Grade for &str {
        fn grade(&self) -> String {
            self.to_string()
        }
    }
}

/// 'report_card' module contains the ReportCard struct, which prints the report card with the student's name, age, and grade
mod report_card {
    use super::grade::Grade;
    use std::fmt::Display;

    /// Change grade to a generic type T that implements the Grade trait
    pub struct ReportCard<T: Grade> {
        pub grade: T,
        pub student_name: String,
        pub student_age: u8,
    }

    // Add trait bound syntax to the impl block to allow the use of the Grade trait, and Display (for self.grade)
    impl<T: Grade + Display> ReportCard<T> {
        /// Print the report card, showing the student's name, age, and grade
        pub fn print(&self) -> String {
            format!(
                "{} ({}) - achieved a grade of {}",
                &self.student_name, &self.student_age, &self.grade
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::grade::Grade;
    use super::report_card::ReportCard;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
