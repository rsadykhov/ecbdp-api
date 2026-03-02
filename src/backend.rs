use chrono::TimeZone;
use serde::de::DeserializeOwned;
use reqwest::{Client, Response, StatusCode};
use crate::error::Error;
use crate::query::{Resource, Query};
use crate::parameter::{data::DataParameter, metadata::MetadataParameter};


/// European Central Bank Data Portal data collection backend.
/// 
/// - For API details refrence: <https://data.ecb.europa.eu/help/api/overview>
/// - For available data sets reference: <https://data.ecb.europa.eu/data/datasets>
/// 
/// Note: On the ECB Data Portal the series are prefixed with their flow identifier. For example,
/// `EXR.M.USD.EUR.SP00.A` is the series key on the ECB Data Portal, but `EXR` is an `flow_id` and `M.USD.EUR.SP00.A`
/// is the `series_key` in this implementation.
pub struct ECBDataPortal;

impl ECBDataPortal {
    /// Checks the status code of the response for potential errors that can be converted into a `Rust` errors.
    fn process_status_code(status_code: &StatusCode) -> Result<(), Error> {
        match status_code.as_u16() {
            400 => Err(Error::SC400),
            404 => Err(Error::SC404),
            406 => Err(Error::SC406),
            500 => Err(Error::SC500),
            501 => Err(Error::SC501),
            503 => Err(Error::SC503),
            _ => Ok((),)
        }
    }

    /// Sends the request, and receive and process the response.
    async fn process_request(url: &str) -> Result<String, Error> {
        let user_agent: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.90 Safari/537.36";
        let client: Client = Client::builder().user_agent(user_agent).build()?;
        let response: Response = client.get(url).send().await?;
        // Process status code
        let status_code: StatusCode = response.status();
        Self::process_status_code(&status_code)?;
        // Return no data (Status code `304` in ECB Data Portal corresponds to no changes in the database since last request)
        if status_code.as_u16() == 304 {
            return Ok("[]".to_owned()); // Empty list
        }
        // Process response body
        let response_body: String = response.text().await?;
        Ok(response_body)
    }

    /// Sends a `data` resource request provided the constructed query and the list of parameters.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use chrono::{FixedOffset, DateTime, TimeZone};
    /// use ecbdp_api::{ECBDataPortal, Query, FlowRef, DataParameter, ECBResponse};
    /// use ecbdp_api::parameter::data::{Detail, Format};
    /// 
    /// #[tokio::main]
    /// async fn main() -> () {
    /// 
    ///     // Query
    ///     let q: Query = Query::new()
    ///         .flow_ref(FlowRef { agency_id: None, flow_id: "EXR".to_owned(), version: None, })
    ///         .series_key("M.USD.EUR.SP00.A");
    ///     
    ///     // Parameters
    ///     let hour: i32 = 3600;
    ///     let datetime: DateTime<FixedOffset> = FixedOffset::east_opt(1 * hour).unwrap()
    ///         .with_ymd_and_hms(2009, 05, 15, 14, 15, 0).unwrap();
    ///     let parameters: Option<Vec<DataParameter<FixedOffset>>> = Some(vec![
    ///         DataParameter::UpdatedAfter { datetime, },
    ///         DataParameter::Detail { detail: Detail::DataOnly, },
    ///         DataParameter::Format { format: Format::JSONData, }
    ///     ]);
    ///     
    ///     // Backend
    ///     let ecb_response: ECBResponse = ECBDataPortal::get_data(&q, parameters).await.unwrap();
    /// 
    ///     assert!(0 < ecb_response.datasets[0].series.iter().last().unwrap().1.observations.as_ref().unwrap().len());
    ///     assert_eq!(ecb_response.structure.name, "Exchange Rates".to_owned());
    /// 
    /// }
    /// ```
    pub async fn get_data<Tz, T>(q: &Query, parameters: Option<Vec<DataParameter<Tz>>>) -> Result<T, Error>
    where
        Tz: TimeZone,
        <Tz as TimeZone>::Offset: std::fmt::Display,
        T: DeserializeOwned,
    {
        q.validate_query(Resource::all_data_resources())?;
        // Generate URL of the query (Including parameters)
        let mut url: String = q.generate_url()? + "?";
        if let Some(params) = parameters {
            url += &params.iter().map(|p| p.to_string() ).collect::<Vec<String>>().join("&");
        }
        let response_body: String = Self::process_request(&url).await?;
        let ecb_response: T = serde_json::from_str(&response_body)?;
        Ok(ecb_response)
    }

    /// Sends a `schema` resource request provided the constructed query.
    /// 
    /// Note: The returned type is a `String`, which is just a text body of the response and it can be further deserialized
    /// if the appropriate deserializetion traget `struct` is provided.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use ecbdp_api::{ECBDataPortal, Query, Resource, Context};
    /// 
    /// #[tokio::main]
    /// async fn main() -> () {
    /// 
    ///     // Query
    ///     let q: Query = Query::new()
    ///         .resource(Resource::Schema)
    ///         .context(Context::DataStructure)
    ///         .agency_id("ECB")
    ///         .resource_id("ECB_EXR1")
    ///         .version("1.0");
    /// 
    ///     // Backend
    ///     let schema: String = ECBDataPortal::get_schema(&q).await.unwrap();
    /// 
    ///     assert!(!schema.is_empty())
    /// 
    /// }
    /// ```
    pub async fn get_schema(q: &Query) -> Result<String, Error> {
        q.validate_query(Resource::all_schema_resources())?;
        let url: String = q.generate_url()?;
        Self::process_request(&url).await
    }

    /// Sends a `metadata` resource request provided the constructed query and the list of parameters.
    /// 
    /// Note: The returned type is a `String`, which is just a text body of the response and it can be further deserialized
    /// if the appropriate deserializetion traget `struct` is provided.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use ecbdp_api::{ECBDataPortal, Query, Resource};
    /// 
    /// #[tokio::main]
    /// async fn main() -> () {
    /// 
    ///     // Query
    ///     let q: Query = Query::new()
    ///         .resource(Resource::MetadataDataStructure)
    ///         .agency_id("ECB")
    ///         .resource_id("ECB_EXR1")
    ///         .version("latest");
    /// 
    ///     // Backend
    ///     let metadata: String = ECBDataPortal::get_metadata(&q, None).await.unwrap();
    /// 
    ///     assert!(!metadata.is_empty())
    /// 
    /// }
    /// ```
    pub async fn get_metadata(q: &Query, parameters: Option<Vec<MetadataParameter>>) -> Result<String, Error> {
        q.validate_query(Resource::all_metadata_resources())?;
        // Generate URL of the query (Including parameters)
        let mut url: String = q.generate_url()? + "?";
        if let Some(params) = parameters {
            url += &params.iter().map(|p| p.to_string() ).collect::<Vec<String>>().join("&");
        }
        Self::process_request(&url).await
    }
}


#[cfg(test)]
mod tests {
    use chrono::{FixedOffset, DateTime, TimeZone};
    use crate::backend::ECBDataPortal;
    use crate::query;
    use crate::parameter::data as pd;
    use crate::schemas;

    /// Functions that sets up the data for the unit tests.
    async fn unit_test_set_up(flow_id: &str, series_key: &str) -> schemas::ECBResponse {
        // Query
        let q: query::Query = query::Query::new()
            .flow_ref(query::FlowRef { agency_id: None, flow_id: flow_id.to_owned(), version: None, })
            .series_key(series_key);
        // Parameters
        let hour: i32 = 3600;
        let datetime: DateTime<FixedOffset> = FixedOffset::east_opt(1 * hour).unwrap()
            .with_ymd_and_hms(2009, 05, 15, 14, 15, 0).unwrap();
        let parameters: Option<Vec<pd::DataParameter<FixedOffset>>> = Some(vec![
            pd::DataParameter::UpdatedAfter { datetime, },
            pd::DataParameter::Detail { detail: pd::Detail::DataOnly, },
            pd::DataParameter::Format { format: pd::Format::JSONData, }
        ]);
        // Backend
        ECBDataPortal::get_data(&q, parameters).await.unwrap()
    }

    #[tokio::test]
    async fn unit_test_get_data_1() -> () {
        let ecb_response: schemas::ECBResponse = unit_test_set_up("EXR", "M.USD.EUR.SP00.A").await;
        assert!(0 < ecb_response.datasets[0].series.iter().last().unwrap().1.observations.as_ref().unwrap().len());
        assert_eq!(ecb_response.structure.name, "Exchange Rates".to_owned());
    }

    #[tokio::test]
    async fn unit_test_get_data_2() -> () {
        let ecb_response: schemas::ECBResponse = unit_test_set_up("FM", "B.U2.EUR.4F.KR.MLFR.LEV").await;
        assert!(0 < ecb_response.datasets[0].series.iter().last().unwrap().1.observations.as_ref().unwrap().len());
        assert_eq!(ecb_response.structure.name, "Financial market data".to_owned());
    }

    #[tokio::test]
    async fn unit_test_get_data_3() -> () {
        let ecb_response: schemas::ECBResponse = unit_test_set_up("CBD2", "Q.B0.W0.11._Z._Z.A.A.A0000._X.ALL.CA._Z.LE._T.EUR").await;
        assert!(0 < ecb_response.datasets[0].series.iter().last().unwrap().1.observations.as_ref().unwrap().len());
        assert_eq!(ecb_response.structure.name, "Consolidated Banking data".to_owned());
    }

    #[tokio::test]
    async fn unit_test_get_data_4() -> () {
        let ecb_response: schemas::ECBResponse = unit_test_set_up("PDD", "H.B0.W0.1._T.DDS_ALL._T._Z.N.PN").await;
        assert!(0 < ecb_response.datasets[0].series.iter().last().unwrap().1.observations.as_ref().unwrap().len());
        assert_eq!(ecb_response.structure.name, "PDD".to_owned());
    }

    #[tokio::test]
    async fn unit_test_get_data_5() -> () {
        let ecb_response: schemas::ECBResponse = unit_test_set_up("TGB", "M.U4.N.A094T.U2.EUR.A").await;
        assert!(0 < ecb_response.datasets[0].series.iter().last().unwrap().1.observations.as_ref().unwrap().len());
        assert_eq!(ecb_response.structure.name, "Target Balances".to_owned());
    }

    #[tokio::test]
    async fn unit_test_get_schema() -> () {
        // Query
        let q: query::Query = query::Query::new()
            .resource(query::Resource::Schema)
            .context(query::Context::DataStructure)
            .agency_id("ECB")
            .resource_id("ECB_EXR1")
            .version("1.0");
        // Backend
        let schema: String = ECBDataPortal::get_schema(&q).await.unwrap();
        assert!(!schema.is_empty())
    }

    #[tokio::test]
    async fn unit_test_get_metadata() -> () {
        // Query
        let q: query::Query = query::Query::new()
            .resource(query::Resource::MetadataDataStructure)
            .agency_id("ECB")
            .resource_id("ECB_EXR1")
            .version("latest");
        // Backend
        let metadata: String = ECBDataPortal::get_metadata(&q, None).await.unwrap();
        assert!(!metadata.is_empty())
    }
}