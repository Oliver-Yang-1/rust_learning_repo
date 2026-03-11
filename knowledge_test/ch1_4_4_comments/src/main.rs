fn main() {
    // 练习要求：
    // 1. 在文件顶部补 2 到 3 行注释，说明这个程序在做什么。
    // 2. 在折扣计算逻辑前补注释，解释为什么会有学生折扣。
    // 3. 给某一行变量定义添加一个行尾注释。
    // 4. 保持输出不变。

    let price = 100;
    let is_student = true;

    let final_price = if is_student { price - 20 } else { price };
    let tag = if is_student { "student" } else { "regular" };

    println!("price: {}", price);
    println!("final price: {}", final_price);
    println!("tag: {}", tag);
}
