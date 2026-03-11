fn circle_area(radius: f64) -> f64 {
    std::f64::consts::PI * radius * radius
}

fn main() {
    // 这题对应附录 D。代码本身保持正确，README 会引导你用 fmt / fix / clippy
    // 去观察工具如何帮助整理、修复和改进代码。
    let radii = [1.0, 2.5, 4.0];
    for radius in radii {
        println!("radius={radius}, area={:.2}", circle_area(radius));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_uses_standard_pi() {
        let area = circle_area(1.0);
        assert!((area - std::f64::consts::PI).abs() < 1e-10);
    }
}
