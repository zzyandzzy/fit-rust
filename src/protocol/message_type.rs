use crate::protocol::io::write_bin;
use binrw::BinResult;
use tracing::warn;

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

// an enum of all defined messages in the Fit SDK
enum_from_primitive! {
    MessageType, u16,
    FileId = 0,
    Capabilities = 1,
    DeviceSettings = 2,
    UserProfile = 3,
    HrmProfile = 4,
    SdmProfile = 5,
    BikeProfile = 6,
    ZonesTarget = 7,
    HrZone = 8,
    PowerZone = 9,
    MetZone = 10,
    Sport = 12,
    Goal = 15,
    Session = 18,
    Lap = 19,
    Record = 20,
    Event = 21,
    DeviceInfo = 23,
    Workout = 26,
    WorkoutStep = 27,
    Schedule = 28,
    WeightScale = 30,
    Course = 31,
    CoursePoint = 32,
    Totals = 33,
    Activity = 34,
    Software = 35,
    FileCapabilities = 37,
    MesgCapabilities = 38,
    FieldCapabilities = 39,
    FileCreator = 49,
    BloodPressure = 51,
    SpeedZone = 53,
    Monitoring = 55,
    TrainingFile = 72,
    Hrv = 78,
    AntRx = 80,
    AntTx = 81,
    AntChannelId = 82,
    Length = 101,
    MonitoringInfo = 103,
    Pad = 105,
    SlaveDevice = 106,
    Connectivity = 127,
    WeatherConditions = 128,
    WeatherAlert = 129,
    CadenceZone = 131,
    Hr = 132,
    SegmentLap = 142,
    MemoGlob = 145,
    SegmentId = 148,
    SegmentLeaderboardEntry = 149,
    SegmentPoint = 150,
    SegmentFile = 151,
    WorkoutSession = 158,
    WatchfaceSettings = 159,
    GpsMetadata = 160,
    CameraEvent = 161,
    TimestampCorrelation = 162,
    GyroscopeData = 164,
    AccelerometerData = 165,
    ThreeDSensorCalibration = 167,
    VideoFrame = 169,
    ObdiiData = 174,
    NmeaSentence = 177,
    AviationAttitude = 178,
    Video = 184,
    VideoTitle = 185,
    VideoDescription = 186,
    VideoClip = 187,
    OhrSettings = 188,
    ExdScreenConfiguration = 200,
    ExdDataFieldConfiguration = 201,
    ExdDataConceptConfiguration = 202,
    FieldDescription = 206,
    DeveloperDataId = 207,
    MagnetometerData = 208,
    BarometerData = 209,
    OneDSensorCalibration = 210,
    Set = 225,
    StressLevel = 227,
    DiveSettings = 258,
    DiveGas = 259,
    DiveAlarm = 262,
    ExerciseTitle = 264,
    DiveSummary = 268,
    Jump = 285,
    ClimbPro = 317,
    DeviceAuxBatteryInfo = 375,
    DiveApneaAlarm = 393,
    MfgRange = 0xFF00,
    None = 0xFFFF,
}

#[allow(unused)]
#[binrw::writer(writer, endian)]
pub fn write_message_type(value: &MessageType) -> BinResult<()> {
    write_bin(writer, value.to_primitive(), endian)?;
    Ok(())
}

#[binrw::parser()]
pub fn parse_message_type(value: u16) -> BinResult<MessageType> {
    if let Some(message_type) = MessageType::from_primitive(value) {
        return Ok(message_type);
    } else {
        match value {
            0xFF00..=0xFFFE => Ok(MessageType::MfgRange),
            v => {
                warn!("v: {}, MessageType::None!", v);
                Ok(MessageType::None)
            }
        }
    }
}
