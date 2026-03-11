#[allow(dead_code)]
fn first_item<T: Copy>(list: &[T]) -> T {
    list[0]
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let numbers = vec![10, 20, 30];
    let chars = vec!['r', 'u', 's', 't'];
    println!("first number: {}", first_item(&numbers));
    println!("first char: {}", first_item(&chars));

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "hello", y: 'z' };
    let mixed = p1.mixup(p2);
    println!("mixed point: x = {}, y = {}", mixed.x, mixed.y);

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
