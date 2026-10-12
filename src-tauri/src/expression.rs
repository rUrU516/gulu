// 按顺序拆解表达式：加减 → 乘除 → 数字、正负号或括号。
// 乘除在加减之前计算；括号内则重新从加减这一层开始。
pub fn evaluate(expression: &str) -> Result<f64, String> {
    let mut parser = Parser {
        input: expression.as_bytes(),
        position: 0,
    };
    let value = parser.sum()?;

    if parser.position != parser.input.len() {
        return Err("请检查运算符和括号".to_string());
    }
    if !value.is_finite() {
        return Err("计算结果超出了可表示的范围".to_string());
    }

    // 避免把零显示成 -0。
    Ok(if value == 0.0 { 0.0 } else { value })
}

struct Parser<'a> {
    input: &'a [u8],
    position: usize,
}

impl Parser<'_> {
    // 这一层处理加减。
    fn sum(&mut self) -> Result<f64, String> {
        let mut value = self.product()?;
        while let Some(operator @ (b'+' | b'-')) = self.peek() {
            self.position += 1;
            let right = self.product()?;
            value = if operator == b'+' {
                value + right
            } else {
                value - right
            };
        }
        Ok(value)
    }

    // 这一层先处理乘除。
    fn product(&mut self) -> Result<f64, String> {
        let mut value = self.factor()?;
        while let Some(operator @ (b'*' | b'/')) = self.peek() {
            self.position += 1;
            let right = self.factor()?;
            if operator == b'/' && right == 0.0 {
                return Err("不能除以 0".to_string());
            }
            value = if operator == b'*' {
                value * right
            } else {
                value / right
            };
        }
        Ok(value)
    }

    fn factor(&mut self) -> Result<f64, String> {
        match self.peek() {
            Some(b'+') => {
                self.position += 1;
                self.factor()
            }
            Some(b'-') => {
                self.position += 1;
                Ok(-self.factor()?)
            }
            Some(b'(') => {
                self.position += 1;
                let value = self.sum()?;
                if self.peek() != Some(b')') {
                    return Err("括号没有配对".to_string());
                }
                self.position += 1;
                Ok(value)
            }
            _ => self.number(),
        }
    }

    fn number(&mut self) -> Result<f64, String> {
        let start = self.position;
        while self.peek().is_some_and(|c| c.is_ascii_digit() || c == b'.') {
            self.position += 1;
        }
        if start == self.position {
            return Err("表达式不完整，这里需要一个数字".to_string());
        }

        let number = std::str::from_utf8(&self.input[start..self.position])
            .map_err(|_| "数字格式不正确".to_string())?;
        number
            .parse::<f64>()
            .map_err(|_| "数字格式不正确".to_string())
    }

    fn peek(&self) -> Option<u8> {
        self.input.get(self.position).copied()
    }
}
