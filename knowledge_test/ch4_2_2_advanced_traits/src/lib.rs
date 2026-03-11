use std::fmt;
use std::ops::Add;

#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub trait Pilot {
    fn fly(&self) -> &'static str;
}

pub trait Wizard {
    fn fly(&self) -> &'static str;
}

pub struct Human;

impl Pilot for Human {
    fn fly(&self) -> &'static str {
        "This is your captain speaking."
    }
}

impl Wizard for Human {
    fn fly(&self) -> &'static str {
        "Up!"
    }
}

impl Human {
    pub fn fly(&self) -> &'static str {
        "*waving arms furiously*"
    }
}

pub trait Animal {
    fn baby_name() -> String;
}

pub struct Dog;

impl Dog {
    pub fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

pub trait OutlinePrint: fmt::Display {
    fn outline_print(&self) -> String {
        let output = self.to_string();
        let border = "*".repeat(output.len() + 4);
        let padding = format!("*{}*", " ".repeat(output.len() + 2));

        format!("{border}\n{padding}\n* {output} *\n{padding}\n{border}")
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

pub struct Wrapper(pub Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::{Animal, Dog, Human, OutlinePrint, Pilot, Point, Wizard, Wrapper};

    #[test]
    fn add_trait_overloads_plus_for_point() {
        assert_eq!(
            Point { x: 1, y: 2 } + Point { x: 3, y: 4 },
            Point { x: 4, y: 6 }
        );
    }

    #[test]
    fn fully_qualified_calls_can_disambiguate_methods() {
        let person = Human;

        assert_eq!(Pilot::fly(&person), "This is your captain speaking.");
        assert_eq!(Wizard::fly(&person), "Up!");
        assert_eq!(person.fly(), "*waving arms furiously*");
    }

    #[test]
    fn fully_qualified_syntax_works_for_associated_functions() {
        assert_eq!(Dog::baby_name(), "Spot");
        assert_eq!(<Dog as Animal>::baby_name(), "puppy");
    }

    #[test]
    fn supertrait_method_can_use_display_output() {
        let point = Point { x: 1, y: 3 };
        let boxed = point.outline_print();

        assert!(boxed.contains("(1, 3)"));
        assert!(boxed.contains('*'));
    }

    #[test]
    fn newtype_wrapper_can_implement_display() {
        let wrapper = Wrapper(vec![String::from("hello"), String::from("world")]);
        assert_eq!(wrapper.to_string(), "[hello, world]");
    }
}
