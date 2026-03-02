use std::fmt::Display;
use chrono::{DateTime, TimeZone};
use crate::time;


#[derive(Clone, Copy, Debug, Default)]
pub enum PeriodFormat {
    /// YYYY for annual data (e.g., 2013)
    Annual,
    /// YYYY-S[1-2] for semi-annual data (e.g., 2013-S1)
    SemiAnnual,
    /// YYYY-Q[1-4] for quarterly data (e.g., 2013-Q1)
    Quarterly,
    /// YYYY-MM for monthly data (e.g., 2013-01)
    Monthly,
    /// YYYY-W[01-53] for weekly data (e.g., 2013-W01)
    Weekly,
    #[default]
    /// YYYY-MM-DD for daily data (e.g., 2013-01-01)
    Daily,
}


#[derive(Clone, Copy, Debug, Default)]
/// Using the detail parameter, it is possible to specify the desired amount of information to be returned by the web service.
pub enum Detail {
    #[default]
    /// The data (Time series and Observations) and the Attributes will be returned. This is the default.
    Full,
    /// The Attributes will be excluded from the returned message.
    DataOnly,
    /// Only the Time series will be returned, excluding the Attributes and the Observations.
    /// This can be used to list Time series that match a certain query without returning the actual data.
    SeriesKeysOnly,
    /// The Time series will be returned, including the Attributes, but the Observations will not.
    NoData,
}

impl Display for Detail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Full => write!(f, "full"),
            Self::DataOnly => write!(f, "dataonly"),
            Self::SeriesKeysOnly => write!(f, "serieskeysonly"),
            Self::NoData => write!(f, "nodata"),
        }
    }
}


#[derive(Clone, Copy, Debug, Default)]
pub enum Format {
    #[default]
    JSONData,
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JSONData => write!(f, "jsondata"),
        }
    }
}


#[derive(Clone, Debug)]
/// Parameter types for `data` queries.
pub enum DataParameter<Tz>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: std::fmt::Display,
{
    /// It is possible to define a date range for which Observations are to be returned by using the startPeriod and/or endPeriod parameters.
    StartPeriod { datetime: DateTime<Tz>, period_format: PeriodFormat, },
    /// It is possible to define a date range for which Observations are to be returned by using the startPeriod and/or endPeriod parameters.
    EndPeriod { datetime: DateTime<Tz>, period_format: PeriodFormat, },
    /// By supplying a percent-encoded ISO 8601 timestamp for the updatedAfter parameter, it is possible to retrieve the
    /// latest version of changed values in the database after a certain point in time (i.e., updates and revisions).
    /// This will include:
    /// - The Observations that have been added since the supplied timestamp
    /// - TYhe Observations that have been revised since the supplied timestamp
    /// - The Observations that have been deleted since the supplied timestamp
    UpdatedAfter { datetime: DateTime<Tz>, },
    /// Using the detail parameter, it is possible to specify the desired amount of information to be returned by the web service.
    Detail { detail: Detail, },
    /// Using the firstNObservations parameter, it is possible to specify the maximum number of Observations to be returned for
    /// each of the matching Time series, starting from the first Observation (firstNObservations).
    FirstNObservations { n: usize, },
    /// Using the lastNObservations parameter, it is possible to specify the maximum number of Observations to be returned for
    /// each of the matching Time series, counting back from the most recent Observation (lastNObservations).
    LastNObservations { n: usize, },
    /// Using the includeHistory parameter, you can instruct the web service to return previous versions of the matching data.
    /// This allows you to see how the data have evolved over time (i.e., see when new data were released, revised or deleted).
    /// Possible options are:
    /// - `false`: Only the version currently in production will be returned. This is the default.
    /// - `true`: The version currently in production and all previous versions will be returned.
    IncludeHistory { yes: bool, },
    /// Using the format parameter, you can instruct the web service to return data in different formats.
    Format { format: Format, },
}

impl<Tz> Display for DataParameter<Tz>
where
    Tz: TimeZone,
    <Tz as TimeZone>::Offset: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StartPeriod { datetime, period_format } => {
                write!(f, "startPeriod={}", time::datetime_to_ecb_period(datetime, *period_format))
            },
            Self::EndPeriod { datetime, period_format } => {
                write!(f, "endPeriod={}", time::datetime_to_ecb_period(datetime, *period_format))
            },
            Self::UpdatedAfter { datetime } => write!(f, "updatedAfter={}", time::percent_encode_datetime(datetime)),
            Self::Detail { detail } => write!(f, "detail={}", detail.to_string()),
            Self::FirstNObservations { n } => write!(f, "firstNObservations={n}"),
            Self::LastNObservations { n } => write!(f, "lastNObservations={n}"),
            Self::IncludeHistory { yes } => write!(f, "includeHistory={}", yes),
            Self::Format { format } => write!(f, "format={}", format.to_string()),
        }
    }
}