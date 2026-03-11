trait Draw {
    fn draw(&self) -> String;
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) -> Vec<String> {
        self.components.iter().map(|component| component.draw()).collect()
    }
}

struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) -> String {
        format!("Button({}, {}, {})", self.width, self.height, self.label)
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) -> String {
        format!(
            "SelectBox({}, {}, [{}])",
            self.width,
            self.height,
            self.options.join(" | ")
        )
    }
}

fn main() {
    // 任务 1：把不同具体类型都装进 Box<dyn Draw>。
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    // 任务 2：统一遍历并调用 draw，而不关心具体类型。
    for line in screen.run() {
        println!("{line}");
    }

    // 完成后请让程序输出严格匹配 README 中的目标输出。
}
