macro_rules! enum_from_primitive {
    ($typ:ident, $t:ty, $($name:ident = $value:expr),+,) => {
        #[derive(Debug, Copy, Clone, PartialEq)]
        #[repr($t)]
        pub enum $typ {
            $($name = $value),+,
        }

        impl $typ {
            pub fn from_primitive(value: $t) -> Option<Self> {
                match value {
                    $(x if x == $typ::$name as $t => Some($typ::$name),)+
                    _ => None,
                }
            }

            pub fn to_primitive(self) -> $t {
                self as $t
            }
        }
    };
}

enum_from_primitive! {
    MessageType, u16,
    FileId = 100,
    TextId = 200,
    ImageId = 300,
    VideoId = 400,
}

fn main() {
    // 测试从 u32 到 MessageType 的转换
    let message_type_num = 200;
    if let Some(message_type) = MessageType::from_primitive(message_type_num) {
        println!("Converted number to enum: {:?}", message_type);
    } else {
        println!("Unknown message type number: {}", message_type_num);
    }

    // 测试从 MessageType 到 u32 的转换
    let message_type = MessageType::TextId;
    let message_type_num = message_type.to_primitive();
    println!("Converted enum to number: {}", message_type_num);
}
