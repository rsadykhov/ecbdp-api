//! Module with time conversion utilities for ECB Data Portal.
//! 
//! The utilities defined here convert between the formats used by the ECB and `chrono` library.
//! The timezones in `chrono` are assumed to be `FixedOffset`.


use chrono::{FixedOffset, DateTime, Datelike, TimeZone};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use crate::error::Error;
use crate::parameter::data::PeriodFormat;


const PERCENT_ENCODING_FRAGMENT: &AsciiSet = &CONTROLS.add(b':').add(b'+');


/// Converts a `DateTime` object into an ECB Data Portal formatted period.
/// 
/// # Examples
/// 
/// ```rust
/// use chrono::{DateTime, FixedOffset, TimeZone};
/// use ecbdp_api::parameter::data::PeriodFormat;
/// use ecbdp_api::time::datetime_to_ecb_period;
/// 
/// const HOUR: i32 = 3600;
/// 
/// let datetime: DateTime<FixedOffset> = FixedOffset::east_opt(1 * HOUR).unwrap()
///     .with_ymd_and_hms(2009, 05, 15, 14, 15, 0).unwrap();
/// 
/// assert_eq!(datetime_to_ecb_period(&datetime, PeriodFormat::Annual), String::from("2009"));
/// assert_eq!(datetime_to_ecb_period(&datetime, PeriodFormat::SemiAnnual), String::from("2009-S1"));
/// assert_eq!(datetime_to_ecb_period(&datetime, PeriodFormat::Quarterly), String::from("2009-Q2"));
/// assert_eq!(datetime_to_ecb_period(&datetime, PeriodFormat::Monthly), String::from("2009-05"));
/// assert_eq!(datetime_to_ecb_period(&datetime, PeriodFormat::Weekly), String::from("2009-W19"));
/// assert_eq!(datetime_to_ecb_period(&datetime, PeriodFormat::Daily), String::from("2009-05-15"));
/// ```
pub fn datetime_to_ecb_period<Tz>(datetime: &DateTime<Tz>, period: PeriodFormat) -> String
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: std::fmt::Display,
{
    let chrono_format: String = match period {
        PeriodFormat::Annual => "%Y".to_owned(),
        PeriodFormat::SemiAnnual => {
            let half_year: usize = if datetime.month() <=6 { 1 } else { 2 };
            format!("%Y-S{half_year}")
        },
        PeriodFormat::Quarterly => "%Y-Q%q".to_owned(),
        PeriodFormat::Monthly => "%Y-%m".to_owned(),
        PeriodFormat::Weekly => "%Y-W%U".to_owned(),
        PeriodFormat::Daily => "%Y-%m-%d".to_owned(),
    };
    format!("{}", datetime.format(&chrono_format))
}


/// Converts an ECB Data Portal formatted datetime into a `DateTime` object.
/// 
/// # Examples
/// 
/// ```rust
/// use chrono::{DateTime, Datelike, FixedOffset, Timelike};
/// use ecbdp_api::time::ecb_string_to_datetime;
/// 
/// const HOUR: i32 = 3600;
/// 
/// let datetime_str: String = String::from("2025-03-12T23:59:59.999+01:00");
/// let datetime: DateTime<FixedOffset> = ecb_string_to_datetime(&datetime_str).unwrap();
/// 
/// assert_eq!(datetime.year(), 2025);
/// assert_eq!(datetime.month(), 3);
/// assert_eq!(datetime.day(), 12);
/// assert_eq!(datetime.hour(), 23);
/// assert_eq!(datetime.minute(), 59);
/// assert_eq!(datetime.second(), 59);
/// assert_eq!(datetime.timezone(), FixedOffset::east_opt(1 * HOUR).unwrap())
/// ```
pub fn ecb_string_to_datetime(ecb_datetime: &str) -> Result<DateTime<FixedOffset>, Error> {
    Ok(DateTime::parse_from_rfc3339(ecb_datetime)?)
}


/// Percent econdes datetime string to be included in a URL.
/// 
/// # Examples
/// 
/// ```rust
/// use chrono::{DateTime, FixedOffset, TimeZone};
/// use ecbdp_api::time::percent_encode_datetime;
/// 
/// const HOUR: i32 = 3600;
/// 
/// // East offset
/// let datetime: DateTime<FixedOffset> = FixedOffset::east_opt(1 * HOUR).unwrap()
///     .with_ymd_and_hms(2009, 05, 15, 14, 15, 0).unwrap();
/// let encoded_datetime: String = String::from("2009-05-15T14%3A15%3A00%2B01%3A00");
/// assert_eq!(percent_encode_datetime(&datetime), encoded_datetime);
/// 
/// // West offset
/// let datetime: DateTime<FixedOffset> = FixedOffset::west_opt(1 * HOUR).unwrap()
///     .with_ymd_and_hms(2009, 05, 15, 14, 15, 0).unwrap();
/// let encoded_datetime: String = String::from("2009-05-15T14%3A15%3A00-01%3A00");
/// assert_eq!(percent_encode_datetime(&datetime), encoded_datetime);
/// ```
pub fn percent_encode_datetime<Tz: TimeZone>(datetime: &DateTime<Tz>) -> String {
    let datetime_str: String = datetime.to_rfc3339();
    utf8_percent_encode(&datetime_str, PERCENT_ENCODING_FRAGMENT).to_string()
}


#[cfg(test)]
mod tests {
    use chrono::{DateTime, Datelike, FixedOffset, TimeZone, Timelike};

    /// Number of seconds in an hour.
    const HOUR: i32 = 3600;

    #[test]
    fn unit_test_datetime_to_ecb_period() -> () {
        use crate::parameter::data::PeriodFormat;
        use crate::time::datetime_to_ecb_period;
        let datetime: DateTime<FixedOffset> = FixedOffset::east_opt(1 * HOUR).unwrap()
            .with_ymd_and_hms(2009, 05, 15, 14, 15, 0).unwrap();
        assert_eq!(datetime_to_ecb_period(&datetime, PeriodFormat::Annual), String::from("2009"));
        assert_eq!(datetime_to_ecb_period(&datetime, PeriodFormat::SemiAnnual), String::from("2009-S1"));
        assert_eq!(datetime_to_ecb_period(&datetime, PeriodFormat::Quarterly), String::from("2009-Q2"));
        assert_eq!(datetime_to_ecb_period(&datetime, PeriodFormat::Monthly), String::from("2009-05"));
        assert_eq!(datetime_to_ecb_period(&datetime, PeriodFormat::Weekly), String::from("2009-W19"));
        assert_eq!(datetime_to_ecb_period(&datetime, PeriodFormat::Daily), String::from("2009-05-15"));
    }

    #[test]
    fn unit_test_ecb_string_to_datetime() -> () {
        use crate::time::ecb_string_to_datetime;
        let datetime_str: String = String::from("2025-03-12T23:59:59.999+01:00");
        let datetime: DateTime<FixedOffset> = ecb_string_to_datetime(&datetime_str).unwrap();
        assert_eq!(datetime.year(), 2025);
        assert_eq!(datetime.month(), 3);
        assert_eq!(datetime.day(), 12);
        assert_eq!(datetime.hour(), 23);
        assert_eq!(datetime.minute(), 59);
        assert_eq!(datetime.second(), 59);
        assert_eq!(datetime.timezone(), FixedOffset::east_opt(1 * HOUR).unwrap())
    }

    #[test]
    fn unit_test_percent_encode_datetime() -> () {
        use crate::time::percent_encode_datetime;
        // East offset
        let datetime: DateTime<FixedOffset> = FixedOffset::east_opt(1 * HOUR).unwrap()
            .with_ymd_and_hms(2009, 05, 15, 14, 15, 0).unwrap();
        let encoded_datetime: String = String::from("2009-05-15T14%3A15%3A00%2B01%3A00");
        assert_eq!(percent_encode_datetime(&datetime), encoded_datetime);
        // West offset
        let datetime: DateTime<FixedOffset> = FixedOffset::west_opt(1 * HOUR).unwrap()
            .with_ymd_and_hms(2009, 05, 15, 14, 15, 0).unwrap();
        let encoded_datetime: String = String::from("2009-05-15T14%3A15%3A00-01%3A00");
        assert_eq!(percent_encode_datetime(&datetime), encoded_datetime);
    }
}