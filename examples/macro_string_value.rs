use paste::paste;

macro_rules! define_field_mappings {
    ($(($type:ident, { $($key:expr => $value:expr),* $(,)* }) ),* $(,)*) => {
        $(
            paste! {
                pub fn [<stringify_field_value_ $type>](k: usize) -> Option<&'static str> {
                    match k {
                        $(
                            $key => Some($value),
                        )*
                        _ => None,
                    }
                }

                pub fn [<parse_field_value_ $type>](value: &str) -> Option<usize> {
                    match value {
                        $(
                            $value => Some($key),
                        )*
                        _ => None,
                    }
                }
            }
        )*
    }
}

define_field_mappings! {
    (Activity, {
        0 => "manual",
        1 => "auto_multi_sport",
    }),
    (ActivityClass, {
        127 => "level",
        1 => "level_max",
        128 => "athlete",
    }),
}

// 假设的FieldType枚举
#[derive(Debug)]
pub enum FieldType {
    Activity,
    ActivityClass,
    None,
}

pub fn get_field_string_value(f: FieldType, k: usize) -> Option<&'static str> {
    match f {
        FieldType::Activity => stringify_field_value_Activity(k),
        FieldType::ActivityClass => stringify_field_value_ActivityClass(k),
        FieldType::None => None,
        _ => None,
    }
}

pub fn get_field_key_from_string(f: FieldType, value: &str) -> Option<usize> {
    match f {
        FieldType::Activity => parse_field_value_Activity(value),
        FieldType::ActivityClass => parse_field_value_ActivityClass(value),
        FieldType::None => None,
        _ => None,
    }
}

fn main() {
    let f_type = FieldType::Activity;
    let k_value = 1;
    if let Some(literal) = get_field_string_value(f_type, k_value) {
        println!("The string value for key {} is: {}", k_value, literal);
    }

    let f_type = FieldType::ActivityClass;
    let literal_value = "level_max";
    if let Some(k) = get_field_key_from_string(f_type, literal_value) {
        println!("The key for string value '{}' is: {}", literal_value, k);
    }
}
