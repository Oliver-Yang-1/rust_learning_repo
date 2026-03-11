//! # Mini Art
//!
//! `ch3_2_2_publishing_crates` 是一个用来练习以下内容的小型库：
//!
//! - 文档注释
//! - 文档测试
//! - `pub use` 重导出
//! - 发布前的 `Cargo.toml` 元信息

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// RGB 模型中的三原色。
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// 由两种原色混合得到的次级颜色。
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::{PrimaryColor, SecondaryColor};

    /// 等量混合两种原色，得到一种次级颜色。
    ///
    /// # Examples
    ///
    /// ```
    /// use ch3_2_2_publishing_crates::{mix, PrimaryColor, SecondaryColor};
    ///
    /// let mixed = mix(PrimaryColor::Red, PrimaryColor::Yellow);
    /// assert_eq!(mixed, SecondaryColor::Orange);
    /// ```
    pub fn mix(first: PrimaryColor, second: PrimaryColor) -> SecondaryColor {
        match (first, second) {
            (PrimaryColor::Red, PrimaryColor::Yellow)
            | (PrimaryColor::Yellow, PrimaryColor::Red) => SecondaryColor::Orange,
            (PrimaryColor::Yellow, PrimaryColor::Blue)
            | (PrimaryColor::Blue, PrimaryColor::Yellow) => SecondaryColor::Green,
            (PrimaryColor::Red, PrimaryColor::Blue)
            | (PrimaryColor::Blue, PrimaryColor::Red) => SecondaryColor::Purple,
            (PrimaryColor::Red, PrimaryColor::Red) => SecondaryColor::Orange,
            (PrimaryColor::Yellow, PrimaryColor::Yellow) => SecondaryColor::Green,
            (PrimaryColor::Blue, PrimaryColor::Blue) => SecondaryColor::Purple,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PrimaryColor, SecondaryColor, mix};

    #[test]
    fn re_exports_make_api_shorter() {
        assert_eq!(mix(PrimaryColor::Red, PrimaryColor::Blue), SecondaryColor::Purple);
    }
}
