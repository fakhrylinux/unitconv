#[derive(Debug, PartialEq)]
pub enum UnitCategory {
    Temperature,
    Length,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unit {
    Celsius,
    Kelvin,
    Fahrenheit,
    Centimeter,
    Kilometer,
    Inch,
    Miles,
}

impl Unit {
    fn suffix(&self) -> &'static str {
        match self {
            Unit::Celsius => "C",
            Unit::Kelvin => "K",
            Unit::Fahrenheit => "F",
            Unit::Centimeter => "cm",
            Unit::Kilometer => "km",
            Unit::Inch => "inch",
            Unit::Miles => "miles",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Unit::Celsius => "celsius",
            Unit::Kelvin => "kelvin",
            Unit::Fahrenheit => "fahrenheit",
            Unit::Centimeter => "cm",
            Unit::Kilometer => "km",
            Unit::Inch => "inch",
            Unit::Miles => "miles",
        }
    }

    pub fn from_str(s: &str) -> Option<Unit> {
        match s {
            "celsius" => Some(Unit::Celsius),
            "kelvin" => Some(Unit::Kelvin),
            "fahrenheit" => Some(Unit::Fahrenheit),
            "cm" => Some(Unit::Centimeter),
            "km" => Some(Unit::Kilometer),
            "inch" => Some(Unit::Inch),
            "miles" => Some(Unit::Miles),
            _ => None,
        }
    }

    pub fn category(&self) -> UnitCategory {
        match self {
            Unit::Celsius | Unit::Kelvin | Unit::Fahrenheit => UnitCategory::Temperature,
            Unit::Centimeter | Unit::Kilometer | Unit::Inch | Unit::Miles => UnitCategory::Length,
        }
    }
}

pub fn temperature_conversion(from: Unit, to: Unit, value: f64) -> String {
    let kelvin = match from {
        Unit::Celsius => value + 273.15,
        Unit::Kelvin => value,
        Unit::Fahrenheit => (value - 32.0) * 5.0 / 9.0 + 273.15,
        _ => unreachable!(),
    };
    let result = match to {
        Unit::Celsius => kelvin - 273.15,
        Unit::Kelvin => kelvin,
        Unit::Fahrenheit => (kelvin - 273.15) * 9.0 / 5.0 + 32.0,
        _ => unreachable!(),
    };
    let result_text = format!(
        "{} \u{00B0}{} = {} \u{00B0}{}",
        value,
        from.suffix(),
        result,
        to.suffix(),
    );
    println!("{}", result_text);
    result_text
}

pub fn length_conversion(from: Unit, to: Unit, value: f64) -> String {
    let cm = match from {
        Unit::Centimeter => value,
        Unit::Kilometer => value * 100_000.0,
        Unit::Inch => value * 2.54,
        Unit::Miles => value * 160934.4,
        _ => unreachable!(),
    };
    let result = match to {
        Unit::Centimeter => cm,
        Unit::Kilometer => cm / 100_000.0,
        Unit::Inch => cm / 2.54,
        Unit::Miles => cm / 160934.4,
        _ => unreachable!(),
    };
    let result_text = format!("{} {} = {} {}", value, from.suffix(), result, to.suffix());
    println!("{}", result_text);
    result_text
}
