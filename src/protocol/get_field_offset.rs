use crate::protocol::message_type::MessageType;
use crate::protocol::MatchOffsetFn;

fn match_offset_accelerometer_data(_: usize) -> Option<i16> {
    None
}
fn match_offset_activity(_: usize) -> Option<i16> {
    None
}
fn match_offset_ant_channel_id(_: usize) -> Option<i16> {
    None
}
fn match_offset_ant_rx(_: usize) -> Option<i16> {
    None
}
fn match_offset_ant_tx(_: usize) -> Option<i16> {
    None
}
fn match_offset_aviation_attitude(_: usize) -> Option<i16> {
    None
}
fn match_offset_barometer_data(_: usize) -> Option<i16> {
    None
}
fn match_offset_bike_profile(k: usize) -> Option<i16> {
    match k {
        19 => Some(-110i16),
        _ => None,
    }
}
fn match_offset_blood_pressure(_: usize) -> Option<i16> {
    None
}
fn match_offset_cadence_zone(_: usize) -> Option<i16> {
    None
}
fn match_offset_camera_event(_: usize) -> Option<i16> {
    None
}
fn match_offset_capabilities(_: usize) -> Option<i16> {
    None
}
fn match_offset_climb_pro(_: usize) -> Option<i16> {
    None
}
fn match_offset_connectivity(_: usize) -> Option<i16> {
    None
}
fn match_offset_course(_: usize) -> Option<i16> {
    None
}
fn match_offset_course_point(_: usize) -> Option<i16> {
    None
}
fn match_offset_developer_data_id(_: usize) -> Option<i16> {
    None
}
fn match_offset_device_aux_battery_info(_: usize) -> Option<i16> {
    None
}
fn match_offset_device_info(_: usize) -> Option<i16> {
    None
}
fn match_offset_device_settings(_: usize) -> Option<i16> {
    None
}
fn match_offset_dive_alarm(_: usize) -> Option<i16> {
    None
}
fn match_offset_dive_gas(_: usize) -> Option<i16> {
    None
}
fn match_offset_dive_settings(_: usize) -> Option<i16> {
    None
}
fn match_offset_dive_summary(_: usize) -> Option<i16> {
    None
}
fn match_offset_event(_: usize) -> Option<i16> {
    None
}
fn match_offset_exd_data_concept_configuration(_: usize) -> Option<i16> {
    None
}
fn match_offset_exd_data_field_configuration(_: usize) -> Option<i16> {
    None
}
fn match_offset_exd_screen_configuration(_: usize) -> Option<i16> {
    None
}
fn match_offset_exercise_title(_: usize) -> Option<i16> {
    None
}
fn match_offset_field_capabilities(_: usize) -> Option<i16> {
    None
}
fn match_offset_field_description(_: usize) -> Option<i16> {
    None
}
fn match_offset_file_capabilities(_: usize) -> Option<i16> {
    None
}
fn match_offset_file_creator(_: usize) -> Option<i16> {
    None
}
fn match_offset_file_id(_: usize) -> Option<i16> {
    None
}
fn match_offset_goal(_: usize) -> Option<i16> {
    None
}
fn match_offset_gps_metadata(k: usize) -> Option<i16> {
    match k {
        3 => Some(500i16),
        _ => None,
    }
}
fn match_offset_gyroscope_data(_: usize) -> Option<i16> {
    None
}
fn match_offset_hr(_: usize) -> Option<i16> {
    None
}
fn match_offset_hr_zone(_: usize) -> Option<i16> {
    None
}
fn match_offset_hrm_profile(_: usize) -> Option<i16> {
    None
}
fn match_offset_hrv(_: usize) -> Option<i16> {
    None
}
fn match_offset_jump(_: usize) -> Option<i16> {
    None
}
fn match_offset_lap(k: usize) -> Option<i16> {
    match k {
        42 => Some(500i16),
        43 => Some(500i16),
        62 => Some(500i16),
        112 => Some(500i16),
        113 => Some(500i16),
        114 => Some(500i16),
        _ => None,
    }
}
fn match_offset_length(_: usize) -> Option<i16> {
    None
}
fn match_offset_magnetometer_data(_: usize) -> Option<i16> {
    None
}
fn match_offset_memo_glob(_: usize) -> Option<i16> {
    None
}
fn match_offset_mesg_capabilities(_: usize) -> Option<i16> {
    None
}
fn match_offset_met_zone(_: usize) -> Option<i16> {
    None
}
fn match_offset_monitoring(_: usize) -> Option<i16> {
    None
}
fn match_offset_monitoring_info(_: usize) -> Option<i16> {
    None
}
fn match_offset_nmea_sentence(_: usize) -> Option<i16> {
    None
}
fn match_offset_obdii_data(_: usize) -> Option<i16> {
    None
}
fn match_offset_ohr_settings(_: usize) -> Option<i16> {
    None
}
fn match_offset_one_d_sensor_calibration(_: usize) -> Option<i16> {
    None
}
fn match_offset_power_zone(_: usize) -> Option<i16> {
    None
}
fn match_offset_record(k: usize) -> Option<i16> {
    match k {
        2 => Some(500i16),
        78 => Some(500i16),
        _ => None,
    }
}
fn match_offset_schedule(_: usize) -> Option<i16> {
    None
}
fn match_offset_sdm_profile(_: usize) -> Option<i16> {
    None
}
fn match_offset_segment_file(_: usize) -> Option<i16> {
    None
}
fn match_offset_segment_id(_: usize) -> Option<i16> {
    None
}
fn match_offset_segment_lap(k: usize) -> Option<i16> {
    match k {
        34 => Some(500i16),
        35 => Some(500i16),
        54 => Some(500i16),
        _ => None,
    }
}
fn match_offset_segment_leaderboard_entry(_: usize) -> Option<i16> {
    None
}
fn match_offset_segment_point(k: usize) -> Option<i16> {
    match k {
        4 => Some(500i16),
        _ => None,
    }
}
fn match_offset_session(k: usize) -> Option<i16> {
    match k {
        49 => Some(500i16),
        50 => Some(500i16),
        71 => Some(500i16),
        126 => Some(500i16),
        127 => Some(500i16),
        128 => Some(500i16),
        _ => None,
    }
}
fn match_offset_set(_: usize) -> Option<i16> {
    None
}
fn match_offset_slave_device(_: usize) -> Option<i16> {
    None
}
fn match_offset_software(_: usize) -> Option<i16> {
    None
}
fn match_offset_speed_zone(_: usize) -> Option<i16> {
    None
}
fn match_offset_sport(_: usize) -> Option<i16> {
    None
}
fn match_offset_stress_level(_: usize) -> Option<i16> {
    None
}
fn match_offset_three_d_sensor_calibration(_: usize) -> Option<i16> {
    None
}
fn match_offset_timestamp_correlation(_: usize) -> Option<i16> {
    None
}
fn match_offset_totals(_: usize) -> Option<i16> {
    None
}
fn match_offset_training_file(_: usize) -> Option<i16> {
    None
}
fn match_offset_user_profile(_: usize) -> Option<i16> {
    None
}
fn match_offset_video(_: usize) -> Option<i16> {
    None
}
fn match_offset_video_clip(_: usize) -> Option<i16> {
    None
}
fn match_offset_video_description(_: usize) -> Option<i16> {
    None
}
fn match_offset_video_frame(_: usize) -> Option<i16> {
    None
}
fn match_offset_video_title(_: usize) -> Option<i16> {
    None
}
fn match_offset_watchface_settings(_: usize) -> Option<i16> {
    None
}
fn match_offset_weather_alert(_: usize) -> Option<i16> {
    None
}
fn match_offset_weather_conditions(_: usize) -> Option<i16> {
    None
}
fn match_offset_weight_scale(_: usize) -> Option<i16> {
    None
}
fn match_offset_workout(_: usize) -> Option<i16> {
    None
}
fn match_offset_workout_session(_: usize) -> Option<i16> {
    None
}
fn match_offset_workout_step(_: usize) -> Option<i16> {
    None
}
fn match_offset_zones_target(_: usize) -> Option<i16> {
    None
}
fn match_offset_none(_: usize) -> Option<i16> {
    None
}

/// Determines whether any SDK-defined `Message` defines an offset for any of its fields.
///
/// The method is called with a `MessageType` argument and returns a static closure which is called with a
/// field_id `usize` which yields an `Option<i16>`.
///
/// # Example
///
/// ```
/// let message_type = MessageType::Session;
/// let parsed_value = 71;
/// let offset_fn = match_message_offset(message_type);
/// let offset = offset_fn(parsed_value);
/// assert_eq!(offset, Some(500.0));
/// ```
pub fn get_field_offset_fn(m: MessageType) -> MatchOffsetFn {
    match m {
        MessageType::FileId => match_offset_file_id,
        MessageType::FileCreator => match_offset_file_creator,
        MessageType::TimestampCorrelation => match_offset_timestamp_correlation,
        MessageType::Software => match_offset_software,
        MessageType::SlaveDevice => match_offset_slave_device,
        MessageType::Capabilities => match_offset_capabilities,
        MessageType::FileCapabilities => match_offset_file_capabilities,
        MessageType::MesgCapabilities => match_offset_mesg_capabilities,
        MessageType::FieldCapabilities => match_offset_field_capabilities,
        MessageType::DeviceSettings => match_offset_device_settings,
        MessageType::UserProfile => match_offset_user_profile,
        MessageType::HrmProfile => match_offset_hrm_profile,
        MessageType::SdmProfile => match_offset_sdm_profile,
        MessageType::BikeProfile => match_offset_bike_profile,
        MessageType::Connectivity => match_offset_connectivity,
        MessageType::WatchfaceSettings => match_offset_watchface_settings,
        MessageType::OhrSettings => match_offset_ohr_settings,
        MessageType::ZonesTarget => match_offset_zones_target,
        MessageType::Sport => match_offset_sport,
        MessageType::HrZone => match_offset_hr_zone,
        MessageType::SpeedZone => match_offset_speed_zone,
        MessageType::CadenceZone => match_offset_cadence_zone,
        MessageType::PowerZone => match_offset_power_zone,
        MessageType::MetZone => match_offset_met_zone,
        MessageType::DiveSettings => match_offset_dive_settings,
        MessageType::DiveAlarm => match_offset_dive_alarm,
        MessageType::DiveGas => match_offset_dive_gas,
        MessageType::Goal => match_offset_goal,
        MessageType::Activity => match_offset_activity,
        MessageType::Session => match_offset_session,
        MessageType::Lap => match_offset_lap,
        MessageType::Length => match_offset_length,
        MessageType::Record => match_offset_record,
        MessageType::Event => match_offset_event,
        MessageType::DeviceInfo => match_offset_device_info,
        MessageType::DeviceAuxBatteryInfo => match_offset_device_aux_battery_info,
        MessageType::TrainingFile => match_offset_training_file,
        MessageType::WeatherConditions => match_offset_weather_conditions,
        MessageType::WeatherAlert => match_offset_weather_alert,
        MessageType::GpsMetadata => match_offset_gps_metadata,
        MessageType::CameraEvent => match_offset_camera_event,
        MessageType::GyroscopeData => match_offset_gyroscope_data,
        MessageType::AccelerometerData => match_offset_accelerometer_data,
        MessageType::MagnetometerData => match_offset_magnetometer_data,
        MessageType::BarometerData => match_offset_barometer_data,
        MessageType::ThreeDSensorCalibration => match_offset_three_d_sensor_calibration,
        MessageType::OneDSensorCalibration => match_offset_one_d_sensor_calibration,
        MessageType::VideoFrame => match_offset_video_frame,
        MessageType::ObdiiData => match_offset_obdii_data,
        MessageType::NmeaSentence => match_offset_nmea_sentence,
        MessageType::AviationAttitude => match_offset_aviation_attitude,
        MessageType::Video => match_offset_video,
        MessageType::VideoTitle => match_offset_video_title,
        MessageType::VideoDescription => match_offset_video_description,
        MessageType::VideoClip => match_offset_video_clip,
        MessageType::Set => match_offset_set,
        MessageType::Jump => match_offset_jump,
        MessageType::ClimbPro => match_offset_climb_pro,
        MessageType::FieldDescription => match_offset_field_description,
        MessageType::DeveloperDataId => match_offset_developer_data_id,
        MessageType::Course => match_offset_course,
        MessageType::CoursePoint => match_offset_course_point,
        MessageType::SegmentId => match_offset_segment_id,
        MessageType::SegmentLeaderboardEntry => match_offset_segment_leaderboard_entry,
        MessageType::SegmentPoint => match_offset_segment_point,
        MessageType::SegmentLap => match_offset_segment_lap,
        MessageType::SegmentFile => match_offset_segment_file,
        MessageType::Workout => match_offset_workout,
        MessageType::WorkoutSession => match_offset_workout_session,
        MessageType::WorkoutStep => match_offset_workout_step,
        MessageType::ExerciseTitle => match_offset_exercise_title,
        MessageType::Schedule => match_offset_schedule,
        MessageType::Totals => match_offset_totals,
        MessageType::WeightScale => match_offset_weight_scale,
        MessageType::BloodPressure => match_offset_blood_pressure,
        MessageType::MonitoringInfo => match_offset_monitoring_info,
        MessageType::Monitoring => match_offset_monitoring,
        MessageType::Hr => match_offset_hr,
        MessageType::StressLevel => match_offset_stress_level,
        MessageType::MemoGlob => match_offset_memo_glob,
        MessageType::AntChannelId => match_offset_ant_channel_id,
        MessageType::AntRx => match_offset_ant_rx,
        MessageType::AntTx => match_offset_ant_tx,
        MessageType::ExdScreenConfiguration => match_offset_exd_screen_configuration,
        MessageType::ExdDataFieldConfiguration => match_offset_exd_data_field_configuration,
        MessageType::ExdDataConceptConfiguration => match_offset_exd_data_concept_configuration,
        MessageType::DiveSummary => match_offset_dive_summary,
        MessageType::Hrv => match_offset_hrv,
        _ => match_offset_none,
    }
}
