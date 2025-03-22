// enums3.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint enums3` or use the `hint` watch subcommand for a
// hint.



enum Message {
        ChangeColor(u8, u8, u8),
        Echo(String),
        Move(Point),
        Quit,
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
    message: String
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&mut self, s: String) { self.message = s }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        match message {
        // 如果是 `ChangeColor` 变体，解构出 RGB 分量
        Message::ChangeColor(r, g, b) => {
            // 调用 `change_color` 方法改变颜色
            self.change_color((r, g, b));
        }
        // 如果是 `Echo` 变体，解构出字符串
        Message::Echo(s) => {
            // 调用 `echo` 方法更新消息
            self.echo(s);
        }
        // 如果是 `Move` 变体，解构出点
        Message::Move(p) => {
            // 调用 `move_position` 方法改变位置
            self.move_position(p);
        }
        // 如果是 `Quit` 变体，不需要解构
        Message::Quit => {
            // 调用 `quit` 方法设置退出状态
            self.quit();
        }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
            message: "hello world".to_string(),
        };
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
        assert_eq!(state.message, "hello world");
    }
}
