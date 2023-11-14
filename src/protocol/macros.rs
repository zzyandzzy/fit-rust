use crate::protocol::data_field::DataField;
use crate::protocol::value::Value;

// Helper functions and macros for the merge_sessions function
#[macro_export]
macro_rules! update_field {
    ($fields:expr, $field_num:expr, $value:expr) => {
        if let Some(field) = $fields.iter_mut().find(|f| f.field_num == $field_num) {
            field.value = $value;
        }
    };
}

pub fn get_field_value(field_num: u8, values: &[DataField]) -> Option<Value> {
    values
        .iter()
        .find(|field| field.field_num == field_num)
        .and_then(|field| Some(field.value.clone()))
}

#[macro_export]
macro_rules! merge_stats {
    // Base case
    () => {};

    // Summation case
    (sum $field_num:expr, $total_value:expr, $session:expr, $($rest:tt)*) => {
         if let Some(value) = get_field_value($field_num, &$session.message.values) {
            match value {
                Value::U32(val) => {
                    if let Value::U32(total_value) = $total_value {
                        $total_value = Value::U32(total_value + val);
                    }
                },
                Value::U16(val) => {
                    if let Value::U16(total_value) = $total_value {
                        $total_value = Value::U16(total_value + val);
                    }
                },
                _ => {}
            }
        }
        merge_stats!($($rest)*);
    };

    // Maximum value case
    (max $field_num:expr, $max_value:expr, $session:expr, $($rest:tt)*) => {
         if let Some(value) = get_field_value($field_num, &$session.message.values) {
            match value {
                Value::Time(val) => {
                    if let Value::Time(max_val) = $max_value {
                        $max_value = Value::Time(max_val.max(val));
                    }
                },
                Value::U16(val) => {
                    if let Value::U16(max_val) = $max_value {
                        $max_value = Value::U16(max_val.max(val));
                    }
                },
                Value::I16(val) => {
                    if let Value::I16(max_val) = $max_value {
                        $max_value = Value::I16(max_val.max(val));
                    }
                },
                Value::U8(val) => {
                    if let Value::U8(max_val) = $max_value {
                        $max_value = Value::U8(max_val.max(val));
                    }
                },
                Value::I8(val) => {
                    if let Value::I8(max_val) = $max_value {
                        $max_value = Value::I8(max_val.max(val));
                    }
                },
                _ => {}
            }
        }
        merge_stats!($($rest)*);
    };

    // Minimum value case
    (min $field_num:expr, $min_value:expr, $session:expr, $($rest:tt)*) => {
        if let Some(value) = get_field_value($field_num, &$session.message.values) {
            match value {
                Value::Time(val) => {
                    if let Value::Time(min_value) = $min_value {
                        $min_value = Value::Time(min_value.min(val));
                    }
                },
                Value::U16(val) => {
                    if let Value::U16(min_value) = $min_value {
                        $min_value = Value::U16(min_value.min(val));
                    }
                },
                Value::U8(val) => {
                    if let Value::U8(min_value) = $min_value {
                        $min_value = Value::U8(min_value.min(val));
                    }
                },
                _ => {}
            }
        }
        merge_stats!($($rest)*);
    };


    // Average value case
    (avg $field_num:expr, $total_value:expr, $count:expr, $session:expr, $($rest:tt)*) => {
         if let Some(value) = get_field_value($field_num, &$session.message.values) {
            match value {
                Value::U16(val) => {
                    if let Value::I32(total_value) = $total_value {
                        $total_value = Value::I32(total_value + (val as i32));
                        $count = $count + 1;
                    }
                },
                Value::I16(val) => {
                    if let Value::I32(total_value) = $total_value {
                        $total_value = Value::I32(total_value + (val as i32));
                        $count = $count + 1;
                    }
                },
                Value::U8(val) => {
                    if let Value::I32(total_value) = $total_value {
                        $total_value = Value::I32(total_value + (val as i32));
                        $count = $count + 1;
                    }
                },
                Value::I8(val) => {
                    if let Value::I32(total_value) = $total_value {
                        $total_value = Value::I32(total_value + (val as i32));
                        $count = $count + 1;
                    }
                },
                _ => {}
            }
        }
        merge_stats!($($rest)*);
    };
}
