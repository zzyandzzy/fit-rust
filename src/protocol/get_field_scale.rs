use crate::protocol::message_type::MessageType;
use crate::protocol::MatchScaleFn;

fn match_scale_accelerometer_data(_: usize) -> Option<f32> {
    None
}
fn match_scale_activity(k: usize) -> Option<f32> {
    match k {
        0 => Some(1000.0f32),
        _ => None,
    }
}
fn match_scale_ant_channel_id(_: usize) -> Option<f32> {
    None
}
fn match_scale_ant_rx(k: usize) -> Option<f32> {
    match k {
        0 => Some(32768.0f32),
        _ => None,
    }
}
fn match_scale_ant_tx(k: usize) -> Option<f32> {
    match k {
        0 => Some(32768.0f32),
        _ => None,
    }
}
fn match_scale_aviation_attitude(k: usize) -> Option<f32> {
    match k {
        2 => Some(10430.38f32),
        3 => Some(10430.38f32),
        4 => Some(100.0f32),
        5 => Some(100.0f32),
        6 => Some(1024.0f32),
        9 => Some(10430.38f32),
        _ => None,
    }
}
fn match_scale_barometer_data(_: usize) -> Option<f32> {
    None
}
fn match_scale_bike_profile(k: usize) -> Option<f32> {
    match k {
        3 => Some(100.0f32),
        8 => Some(1000.0f32),
        9 => Some(1000.0f32),
        10 => Some(10.0f32),
        11 => Some(10.0f32),
        19 => Some(2.0f32),
        _ => None,
    }
}
fn match_scale_blood_pressure(_: usize) -> Option<f32> {
    None
}
fn match_scale_cadence_zone(_: usize) -> Option<f32> {
    None
}
fn match_scale_camera_event(_: usize) -> Option<f32> {
    None
}
fn match_scale_capabilities(_: usize) -> Option<f32> {
    None
}
fn match_scale_climb_pro(_: usize) -> Option<f32> {
    None
}
fn match_scale_connectivity(_: usize) -> Option<f32> {
    None
}
fn match_scale_course(_: usize) -> Option<f32> {
    None
}
fn match_scale_course_point(k: usize) -> Option<f32> {
    match k {
        4 => Some(100.0f32),
        _ => None,
    }
}
fn match_scale_developer_data_id(_: usize) -> Option<f32> {
    None
}
fn match_scale_device_aux_battery_info(k: usize) -> Option<f32> {
    match k {
        1 => Some(256.0f32),
        _ => None,
    }
}
fn match_scale_device_info(k: usize) -> Option<f32> {
    match k {
        5 => Some(100.0f32),
        10 => Some(256.0f32),
        _ => None,
    }
}
fn match_scale_device_settings(k: usize) -> Option<f32> {
    match k {
        5 => Some(4.0f32),
        _ => None,
    }
}
fn match_scale_dive_alarm(k: usize) -> Option<f32> {
    match k {
        0 => Some(1000.0f32),
        1 => Some(1.0f32),
        _ => None,
    }
}
fn match_scale_dive_gas(_: usize) -> Option<f32> {
    None
}
fn match_scale_dive_settings(k: usize) -> Option<f32> {
    match k {
        6 => Some(100.0f32),
        7 => Some(100.0f32),
        8 => Some(100.0f32),
        17 => Some(1.0f32),
        18 => Some(1.0f32),
        _ => None,
    }
}
fn match_scale_dive_summary(k: usize) -> Option<f32> {
    match k {
        2 => Some(1000.0f32),
        3 => Some(1000.0f32),
        4 => Some(1.0f32),
        5 => Some(1.0f32),
        6 => Some(1.0f32),
        7 => Some(1.0f32),
        8 => Some(1.0f32),
        11 => Some(1000.0f32),
        17 => Some(1000.0f32),
        22 => Some(1000.0f32),
        23 => Some(1000.0f32),
        24 => Some(1000.0f32),
        25 => Some(1000.0f32),
        _ => None,
    }
}
fn match_scale_event(k: usize) -> Option<f32> {
    match k {
        23 => Some(10.0f32),
        24 => Some(10.0f32),
        _ => None,
    }
}
fn match_scale_exd_data_concept_configuration(_: usize) -> Option<f32> {
    None
}
fn match_scale_exd_data_field_configuration(_: usize) -> Option<f32> {
    None
}
fn match_scale_exd_screen_configuration(_: usize) -> Option<f32> {
    None
}
fn match_scale_exercise_title(_: usize) -> Option<f32> {
    None
}
fn match_scale_field_capabilities(_: usize) -> Option<f32> {
    None
}
fn match_scale_field_description(_: usize) -> Option<f32> {
    None
}
fn match_scale_file_capabilities(_: usize) -> Option<f32> {
    None
}
fn match_scale_file_creator(_: usize) -> Option<f32> {
    None
}
fn match_scale_file_id(_: usize) -> Option<f32> {
    None
}
fn match_scale_goal(_: usize) -> Option<f32> {
    None
}
fn match_scale_gps_metadata(k: usize) -> Option<f32> {
    match k {
        3 => Some(5.0f32),
        4 => Some(1000.0f32),
        5 => Some(100.0f32),
        7 => Some(100.0f32),
        _ => None,
    }
}
fn match_scale_gyroscope_data(_: usize) -> Option<f32> {
    None
}
fn match_scale_hr(k: usize) -> Option<f32> {
    match k {
        0 => Some(32768.0f32),
        1 => Some(256.0f32),
        9 => Some(1024.0f32),
        10 => Some(1024.0f32),
        _ => None,
    }
}
fn match_scale_hr_zone(_: usize) -> Option<f32> {
    None
}
fn match_scale_hrm_profile(_: usize) -> Option<f32> {
    None
}
fn match_scale_hrv(k: usize) -> Option<f32> {
    match k {
        0 => Some(1000.0f32),
        _ => None,
    }
}
fn match_scale_jump(k: usize) -> Option<f32> {
    match k {
        7 => Some(1000.0f32),
        8 => Some(1000.0f32),
        _ => None,
    }
}
fn match_scale_lap(k: usize) -> Option<f32> {
    match k {
        7 => Some(1000.0f32),
        8 => Some(1000.0f32),
        9 => Some(100.0f32),
        13 => Some(1000.0f32),
        14 => Some(1000.0f32),
        37 => Some(100.0f32),
        42 => Some(5.0f32),
        43 => Some(5.0f32),
        45 => Some(100.0f32),
        46 => Some(100.0f32),
        47 => Some(100.0f32),
        48 => Some(100.0f32),
        49 => Some(100.0f32),
        52 => Some(1000.0f32),
        53 => Some(1000.0f32),
        54 => Some(1000.0f32),
        55 => Some(1000.0f32),
        56 => Some(1000.0f32),
        57 => Some(1000.0f32),
        58 => Some(1000.0f32),
        59 => Some(1000.0f32),
        60 => Some(1000.0f32),
        62 => Some(5.0f32),
        77 => Some(10.0f32),
        78 => Some(100.0f32),
        79 => Some(10.0f32),
        80 => Some(128.0f32),
        81 => Some(128.0f32),
        82 => Some(128.0f32),
        84 => Some(100.0f32),
        85 => Some(100.0f32),
        86 => Some(100.0f32),
        87 => Some(10.0f32),
        88 => Some(10.0f32),
        89 => Some(10.0f32),
        91 => Some(2.0f32),
        92 => Some(2.0f32),
        93 => Some(2.0f32),
        94 => Some(2.0f32),
        95 => Some(2.0f32),
        98 => Some(1000.0f32),
        102 => Some(0.7111111f32),
        103 => Some(0.7111111f32),
        104 => Some(0.7111111f32),
        105 => Some(0.7111111f32),
        110 => Some(1000.0f32),
        111 => Some(1000.0f32),
        112 => Some(5.0f32),
        113 => Some(5.0f32),
        114 => Some(5.0f32),
        117 => Some(2.0f32),
        118 => Some(100.0f32),
        119 => Some(100.0f32),
        120 => Some(10.0f32),
        121 => Some(1000.0f32),
        156 => Some(100.0f32),
        157 => Some(100.0f32),
        158 => Some(100.0f32),
        159 => Some(100.0f32),
        160 => Some(100.0f32),
        _ => None,
    }
}
fn match_scale_length(k: usize) -> Option<f32> {
    match k {
        3 => Some(1000.0f32),
        4 => Some(1000.0f32),
        6 => Some(1000.0f32),
        _ => None,
    }
}
fn match_scale_magnetometer_data(_: usize) -> Option<f32> {
    None
}
fn match_scale_memo_glob(_: usize) -> Option<f32> {
    None
}
fn match_scale_mesg_capabilities(_: usize) -> Option<f32> {
    None
}
fn match_scale_met_zone(k: usize) -> Option<f32> {
    match k {
        2 => Some(10.0f32),
        3 => Some(10.0f32),
        _ => None,
    }
}
fn match_scale_monitoring(k: usize) -> Option<f32> {
    match k {
        2 => Some(100.0f32),
        3 => Some(2.0f32),
        4 => Some(1000.0f32),
        12 => Some(100.0f32),
        14 => Some(100.0f32),
        15 => Some(100.0f32),
        28 => Some(10.0f32),
        31 => Some(1000.0f32),
        32 => Some(1000.0f32),
        _ => None,
    }
}
fn match_scale_monitoring_info(k: usize) -> Option<f32> {
    match k {
        3 => Some(5000.0f32),
        4 => Some(5000.0f32),
        _ => None,
    }
}
fn match_scale_nmea_sentence(_: usize) -> Option<f32> {
    None
}
fn match_scale_obdii_data(_: usize) -> Option<f32> {
    None
}
fn match_scale_ohr_settings(_: usize) -> Option<f32> {
    None
}
fn match_scale_one_d_sensor_calibration(_: usize) -> Option<f32> {
    None
}
fn match_scale_power_zone(_: usize) -> Option<f32> {
    None
}
fn match_scale_record(k: usize) -> Option<f32> {
    match k {
        2 => Some(5.0f32),
        5 => Some(100.0f32),
        6 => Some(1000.0f32),
        8 => Some(100.0f32),
        9 => Some(100.0f32),
        11 => Some(1000.0f32),
        12 => Some(100.0f32),
        17 => Some(16.0f32),
        32 => Some(1000.0f32),
        39 => Some(10.0f32),
        40 => Some(100.0f32),
        41 => Some(10.0f32),
        43 => Some(2.0f32),
        44 => Some(2.0f32),
        45 => Some(2.0f32),
        46 => Some(2.0f32),
        47 => Some(2.0f32),
        48 => Some(128.0f32),
        51 => Some(100.0f32),
        52 => Some(256.0f32),
        53 => Some(128.0f32),
        54 => Some(100.0f32),
        55 => Some(100.0f32),
        56 => Some(100.0f32),
        57 => Some(10.0f32),
        58 => Some(10.0f32),
        59 => Some(10.0f32),
        69 => Some(0.7111111f32),
        70 => Some(0.7111111f32),
        71 => Some(0.7111111f32),
        72 => Some(0.7111111f32),
        73 => Some(1000.0f32),
        78 => Some(5.0f32),
        81 => Some(2.0f32),
        83 => Some(100.0f32),
        84 => Some(100.0f32),
        85 => Some(10.0f32),
        92 => Some(1000.0f32),
        93 => Some(1000.0f32),
        94 => Some(1.0f32),
        95 => Some(1.0f32),
        96 => Some(1.0f32),
        98 => Some(1.0f32),
        139 => Some(100.0f32),
        _ => None,
    }
}
fn match_scale_schedule(_: usize) -> Option<f32> {
    None
}
fn match_scale_sdm_profile(k: usize) -> Option<f32> {
    match k {
        2 => Some(10.0f32),
        3 => Some(100.0f32),
        _ => None,
    }
}
fn match_scale_segment_file(_: usize) -> Option<f32> {
    None
}
fn match_scale_segment_id(_: usize) -> Option<f32> {
    None
}
fn match_scale_segment_lap(k: usize) -> Option<f32> {
    match k {
        7 => Some(1000.0f32),
        8 => Some(1000.0f32),
        9 => Some(100.0f32),
        13 => Some(1000.0f32),
        14 => Some(1000.0f32),
        34 => Some(5.0f32),
        35 => Some(5.0f32),
        37 => Some(100.0f32),
        38 => Some(100.0f32),
        39 => Some(100.0f32),
        40 => Some(100.0f32),
        41 => Some(100.0f32),
        44 => Some(1000.0f32),
        45 => Some(1000.0f32),
        46 => Some(1000.0f32),
        47 => Some(1000.0f32),
        48 => Some(1000.0f32),
        49 => Some(1000.0f32),
        50 => Some(1000.0f32),
        51 => Some(1000.0f32),
        52 => Some(1000.0f32),
        54 => Some(5.0f32),
        56 => Some(1000.0f32),
        59 => Some(2.0f32),
        60 => Some(2.0f32),
        61 => Some(2.0f32),
        62 => Some(2.0f32),
        63 => Some(2.0f32),
        66 => Some(128.0f32),
        67 => Some(128.0f32),
        68 => Some(128.0f32),
        71 => Some(1000.0f32),
        75 => Some(0.7111111f32),
        76 => Some(0.7111111f32),
        77 => Some(0.7111111f32),
        78 => Some(0.7111111f32),
        89 => Some(100.0f32),
        90 => Some(100.0f32),
        _ => None,
    }
}
fn match_scale_segment_leaderboard_entry(k: usize) -> Option<f32> {
    match k {
        4 => Some(1000.0f32),
        _ => None,
    }
}
fn match_scale_segment_point(k: usize) -> Option<f32> {
    match k {
        3 => Some(100.0f32),
        4 => Some(5.0f32),
        5 => Some(1000.0f32),
        _ => None,
    }
}
fn match_scale_session(k: usize) -> Option<f32> {
    match k {
        7 => Some(1000.0f32),
        8 => Some(1000.0f32),
        9 => Some(100.0f32),
        14 => Some(1000.0f32),
        15 => Some(1000.0f32),
        24 => Some(10.0f32),
        35 => Some(10.0f32),
        36 => Some(1000.0f32),
        41 => Some(10.0f32),
        42 => Some(100.0f32),
        44 => Some(100.0f32),
        49 => Some(5.0f32),
        50 => Some(5.0f32),
        52 => Some(100.0f32),
        53 => Some(100.0f32),
        54 => Some(100.0f32),
        55 => Some(100.0f32),
        56 => Some(100.0f32),
        59 => Some(1000.0f32),
        60 => Some(1000.0f32),
        61 => Some(1000.0f32),
        62 => Some(1000.0f32),
        63 => Some(1000.0f32),
        65 => Some(1000.0f32),
        66 => Some(1000.0f32),
        67 => Some(1000.0f32),
        68 => Some(1000.0f32),
        69 => Some(1000.0f32),
        71 => Some(5.0f32),
        87 => Some(100.0f32),
        88 => Some(100.0f32),
        89 => Some(10.0f32),
        90 => Some(100.0f32),
        91 => Some(10.0f32),
        92 => Some(128.0f32),
        93 => Some(128.0f32),
        94 => Some(128.0f32),
        95 => Some(100.0f32),
        96 => Some(100.0f32),
        97 => Some(100.0f32),
        98 => Some(10.0f32),
        99 => Some(10.0f32),
        100 => Some(10.0f32),
        101 => Some(2.0f32),
        102 => Some(2.0f32),
        103 => Some(2.0f32),
        104 => Some(2.0f32),
        105 => Some(2.0f32),
        112 => Some(1000.0f32),
        116 => Some(0.7111111f32),
        117 => Some(0.7111111f32),
        118 => Some(0.7111111f32),
        119 => Some(0.7111111f32),
        124 => Some(1000.0f32),
        125 => Some(1000.0f32),
        126 => Some(5.0f32),
        127 => Some(5.0f32),
        128 => Some(5.0f32),
        131 => Some(2.0f32),
        132 => Some(100.0f32),
        133 => Some(100.0f32),
        134 => Some(10.0f32),
        137 => Some(10.0f32),
        139 => Some(1000.0f32),
        168 => Some(65536.0f32),
        199 => Some(100.0f32),
        200 => Some(100.0f32),
        208 => Some(100.0f32),
        209 => Some(100.0f32),
        210 => Some(100.0f32),
        _ => None,
    }
}
fn match_scale_set(k: usize) -> Option<f32> {
    match k {
        0 => Some(1000.0f32),
        4 => Some(16.0f32),
        _ => None,
    }
}
fn match_scale_slave_device(_: usize) -> Option<f32> {
    None
}
fn match_scale_software(k: usize) -> Option<f32> {
    match k {
        3 => Some(100.0f32),
        _ => None,
    }
}
fn match_scale_speed_zone(k: usize) -> Option<f32> {
    match k {
        0 => Some(1000.0f32),
        _ => None,
    }
}
fn match_scale_sport(_: usize) -> Option<f32> {
    None
}
fn match_scale_stress_level(_: usize) -> Option<f32> {
    None
}
fn match_scale_three_d_sensor_calibration(k: usize) -> Option<f32> {
    match k {
        5 => Some(65535.0f32),
        _ => None,
    }
}
fn match_scale_timestamp_correlation(k: usize) -> Option<f32> {
    match k {
        0 => Some(32768.0f32),
        2 => Some(32768.0f32),
        _ => None,
    }
}
fn match_scale_totals(_: usize) -> Option<f32> {
    None
}
fn match_scale_training_file(_: usize) -> Option<f32> {
    None
}
fn match_scale_user_profile(k: usize) -> Option<f32> {
    match k {
        3 => Some(100.0f32),
        4 => Some(10.0f32),
        31 => Some(1000.0f32),
        32 => Some(1000.0f32),
        _ => None,
    }
}
fn match_scale_video(_: usize) -> Option<f32> {
    None
}
fn match_scale_video_clip(_: usize) -> Option<f32> {
    None
}
fn match_scale_video_description(_: usize) -> Option<f32> {
    None
}
fn match_scale_video_frame(_: usize) -> Option<f32> {
    None
}
fn match_scale_video_title(_: usize) -> Option<f32> {
    None
}
fn match_scale_watchface_settings(_: usize) -> Option<f32> {
    None
}
fn match_scale_weather_alert(_: usize) -> Option<f32> {
    None
}
fn match_scale_weather_conditions(k: usize) -> Option<f32> {
    match k {
        4 => Some(1000.0f32),
        _ => None,
    }
}
fn match_scale_weight_scale(k: usize) -> Option<f32> {
    match k {
        0 => Some(100.0f32),
        1 => Some(100.0f32),
        2 => Some(100.0f32),
        3 => Some(100.0f32),
        4 => Some(100.0f32),
        5 => Some(100.0f32),
        7 => Some(4.0f32),
        9 => Some(4.0f32),
        _ => None,
    }
}
fn match_scale_workout(k: usize) -> Option<f32> {
    match k {
        14 => Some(100.0f32),
        _ => None,
    }
}
fn match_scale_workout_session(k: usize) -> Option<f32> {
    match k {
        4 => Some(100.0f32),
        _ => None,
    }
}
fn match_scale_workout_step(k: usize) -> Option<f32> {
    match k {
        12 => Some(100.0f32),
        _ => None,
    }
}
fn match_scale_zones_target(_: usize) -> Option<f32> {
    None
}
fn match_scale_none(_: usize) -> Option<f32> {
    return None;
}

/// Determines whether any SDK-defined `Message` defines a scale for any of its fields.
///
/// The method is called with a `MessageType` argument and returns a static closure which is called with a field_id `usize`
/// and yields an `Option<f32>`.
///
/// # Example
///
/// ```
/// let message_type = MessageType::Workout;
/// let parsed_value = 14;
/// let scale_fn = match_message_scale(message_type);
/// let scale = scale_fn(parsed_value);
/// assert_eq!(scale, Some(100.0));
/// ```
pub fn get_field_scale_fn(m: MessageType) -> MatchScaleFn {
    match m {
        MessageType::FileId => match_scale_file_id,
        MessageType::FileCreator => match_scale_file_creator,
        MessageType::TimestampCorrelation => match_scale_timestamp_correlation,
        MessageType::Software => match_scale_software,
        MessageType::SlaveDevice => match_scale_slave_device,
        MessageType::Capabilities => match_scale_capabilities,
        MessageType::FileCapabilities => match_scale_file_capabilities,
        MessageType::MesgCapabilities => match_scale_mesg_capabilities,
        MessageType::FieldCapabilities => match_scale_field_capabilities,
        MessageType::DeviceSettings => match_scale_device_settings,
        MessageType::UserProfile => match_scale_user_profile,
        MessageType::HrmProfile => match_scale_hrm_profile,
        MessageType::SdmProfile => match_scale_sdm_profile,
        MessageType::BikeProfile => match_scale_bike_profile,
        MessageType::Connectivity => match_scale_connectivity,
        MessageType::WatchfaceSettings => match_scale_watchface_settings,
        MessageType::OhrSettings => match_scale_ohr_settings,
        MessageType::ZonesTarget => match_scale_zones_target,
        MessageType::Sport => match_scale_sport,
        MessageType::HrZone => match_scale_hr_zone,
        MessageType::SpeedZone => match_scale_speed_zone,
        MessageType::CadenceZone => match_scale_cadence_zone,
        MessageType::PowerZone => match_scale_power_zone,
        MessageType::MetZone => match_scale_met_zone,
        MessageType::DiveSettings => match_scale_dive_settings,
        MessageType::DiveAlarm => match_scale_dive_alarm,
        MessageType::DiveGas => match_scale_dive_gas,
        MessageType::Goal => match_scale_goal,
        MessageType::Activity => match_scale_activity,
        MessageType::Session => match_scale_session,
        MessageType::Lap => match_scale_lap,
        MessageType::Length => match_scale_length,
        MessageType::Record => match_scale_record,
        MessageType::Event => match_scale_event,
        MessageType::DeviceInfo => match_scale_device_info,
        MessageType::DeviceAuxBatteryInfo => match_scale_device_aux_battery_info,
        MessageType::TrainingFile => match_scale_training_file,
        MessageType::WeatherConditions => match_scale_weather_conditions,
        MessageType::WeatherAlert => match_scale_weather_alert,
        MessageType::GpsMetadata => match_scale_gps_metadata,
        MessageType::CameraEvent => match_scale_camera_event,
        MessageType::GyroscopeData => match_scale_gyroscope_data,
        MessageType::AccelerometerData => match_scale_accelerometer_data,
        MessageType::MagnetometerData => match_scale_magnetometer_data,
        MessageType::BarometerData => match_scale_barometer_data,
        MessageType::ThreeDSensorCalibration => match_scale_three_d_sensor_calibration,
        MessageType::OneDSensorCalibration => match_scale_one_d_sensor_calibration,
        MessageType::VideoFrame => match_scale_video_frame,
        MessageType::ObdiiData => match_scale_obdii_data,
        MessageType::NmeaSentence => match_scale_nmea_sentence,
        MessageType::AviationAttitude => match_scale_aviation_attitude,
        MessageType::Video => match_scale_video,
        MessageType::VideoTitle => match_scale_video_title,
        MessageType::VideoDescription => match_scale_video_description,
        MessageType::VideoClip => match_scale_video_clip,
        MessageType::Set => match_scale_set,
        MessageType::Jump => match_scale_jump,
        MessageType::ClimbPro => match_scale_climb_pro,
        MessageType::FieldDescription => match_scale_field_description,
        MessageType::DeveloperDataId => match_scale_developer_data_id,
        MessageType::Course => match_scale_course,
        MessageType::CoursePoint => match_scale_course_point,
        MessageType::SegmentId => match_scale_segment_id,
        MessageType::SegmentLeaderboardEntry => match_scale_segment_leaderboard_entry,
        MessageType::SegmentPoint => match_scale_segment_point,
        MessageType::SegmentLap => match_scale_segment_lap,
        MessageType::SegmentFile => match_scale_segment_file,
        MessageType::Workout => match_scale_workout,
        MessageType::WorkoutSession => match_scale_workout_session,
        MessageType::WorkoutStep => match_scale_workout_step,
        MessageType::ExerciseTitle => match_scale_exercise_title,
        MessageType::Schedule => match_scale_schedule,
        MessageType::Totals => match_scale_totals,
        MessageType::WeightScale => match_scale_weight_scale,
        MessageType::BloodPressure => match_scale_blood_pressure,
        MessageType::MonitoringInfo => match_scale_monitoring_info,
        MessageType::Monitoring => match_scale_monitoring,
        MessageType::Hr => match_scale_hr,
        MessageType::StressLevel => match_scale_stress_level,
        MessageType::MemoGlob => match_scale_memo_glob,
        MessageType::AntChannelId => match_scale_ant_channel_id,
        MessageType::AntRx => match_scale_ant_rx,
        MessageType::AntTx => match_scale_ant_tx,
        MessageType::ExdScreenConfiguration => match_scale_exd_screen_configuration,
        MessageType::ExdDataFieldConfiguration => match_scale_exd_data_field_configuration,
        MessageType::ExdDataConceptConfiguration => match_scale_exd_data_concept_configuration,
        MessageType::DiveSummary => match_scale_dive_summary,
        MessageType::Hrv => match_scale_hrv,
        _ => match_scale_none,
    }
}
