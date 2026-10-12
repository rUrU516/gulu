use serde::Serialize;

use crate::expression::evaluate;

// 输入的表达式和上次的结果都保存在 Rust 中。
#[derive(Default)]
pub struct Calculator {
    expression: String,
    result: Option<f64>,
    error: Option<String>,
}

// 每次按键后，Rust 把要显示的内容交给前端。
#[derive(Serialize)]
pub struct CalculatorView {
    pub expression: String,
    pub display: String,
    pub error: Option<String>,
}

impl Calculator {
    pub fn press_key(&mut self, key: &str) -> Result<CalculatorView, String> {
        match key {
            "snapshot" => {}
            "clear" => *self = Self::default(),
            "backspace" => {
                self.expression.pop();
                self.result = None;
                self.error = None;
            }
            "equals" => {
                if !self.expression.is_empty() && self.result.is_none() {
                    match evaluate(&self.expression) {
                        Ok(value) => {
                            self.result = Some(value);
                            self.error = None;
                        }
                        Err(message) => self.error = Some(message),
                    }
                }
            }
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "." | "+" | "-" | "*"
            | "/" | "(" | ")" => self.append(key),
            _ => return Err("不支持这个按键".to_string()),
        }

        Ok(self.view())
    }

    fn append(&mut self, key: &str) {
        // 得到结果后，运算符继续计算；数字或左括号开始新的计算。
        if let Some(value) = self.result.take() {
            self.expression = if matches!(key, "+" | "-" | "*" | "/") {
                value.to_string()
            } else {
                String::new()
            };
        }
        self.error = None;

        let last = self.expression.chars().last();
        // 例如 2(3+4) 和 (2+3)4，自动补上乘号。
        if (key == "(" && last.is_some_and(|c| c.is_ascii_digit() || c == '.' || c == ')'))
            || (last == Some(')') && (key == "." || key.as_bytes()[0].is_ascii_digit()))
        {
            self.expression.push('*');
        }

        if key == "."
            && (self.expression.is_empty() || self.expression.ends_with(['+', '-', '*', '/', '(']))
        {
            self.expression.push('0');
        }
        self.expression.push_str(key);
    }

    fn view(&self) -> CalculatorView {
        let expression = self
            .expression
            .replace('*', " × ")
            .replace('/', " ÷ ")
            .replace('+', " + ")
            .replace('-', " − ");

        CalculatorView {
            expression: if self.result.is_some() {
                format!("{expression} =")
            } else if self.error.is_some() {
                expression.clone()
            } else {
                String::new()
            },
            display: if self.error.is_some() {
                "错误".to_string()
            } else if let Some(value) = self.result {
                value.to_string()
            } else if expression.is_empty() {
                "0".to_string()
            } else {
                expression
            },
            error: self.error.clone(),
        }
    }
}
