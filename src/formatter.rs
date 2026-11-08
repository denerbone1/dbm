
pub fn format_number(value: f64) -> Result<String, String> {
    if !value.is_finite() {
        return Err("Cannot format a non-finite number".to_string());
    }

    if value == 0.0 {
        return Ok("()".to_string());
    }

    if value == 1.0 {
        return Ok("1".to_string());
    }

    // Отрицательные целые числа.
    if value < 0.0 && value.fract() == 0.0 {
        let integer = value as i64;

        if integer == -1 {
            return Ok("-1".to_string());
        }

        return Ok(format!("{}+1", integer - 1));
    }

    if value < 0.0 {
        let positive = format_number(-value)?;
        return Ok(format!("-{{{positive}}}"));
    }
    // Получаем кратчайшую десятичную запись, сохраняющую исходный f64.
    let decimal = value.to_string();
    let (integer_part, fractional_part) = decimal
        .split_once('.')
        .unwrap_or((&decimal, ""));

    let integer: i64 = integer_part
        .parse()
        .map_err(|_| "Integer part is out of range".to_string())?;

    let mut result = match integer {
        0 => "()".to_string(),
        1 => "1".to_string(),
        _ => encode_component(integer),
    };

    // Кодируем дробную часть.
    for digit in fractional_part.chars() {
        let digit = digit
            .to_digit(10)
            .ok_or_else(|| "Invalid decimal digit".to_string())?;

        result.push(';');
        result.push_str(&encode_decimal_digit(digit));
    }

    Ok(format!("|{result}|"))
}

fn encode_decimal_digit(value: u32) -> String {
    if value == 0 {
        "()".to_string()
    } else {
        value.to_string()
    }
}


fn encode_component(value: i64) -> String {
    format!("-{}-1", value - 1)
}