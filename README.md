# ECB Data Portal API Wrapper

`ecbdp-api` is a wrapper for European Central Bank (ECB) Data Portal. The data can be accessed using
series keys, which can be found at the ECB Data Portal website:
- <https://data.ecb.europa.eu/data/datasets>

For more information on the ECB Data Portal API use the API documentation:
- <https://data.ecb.europa.eu/help/api/overview>

Note: On the ECB Data Portal, the series are prefixed with their flow identifier. For example,
`EXR.M.USD.EUR.SP00.A` is the series key on the ECB Data Portal, but for the purpose of this crate `EXR` is a `flow_id`
and `M.USD.EUR.SP00.A` is a `series_key`.

**Disclaimer:** This crate is an unofficial ECB Data Portal wrapper, the maintainers of the crate are independent developers.
The developers of the crate do not accept any responsibility or liability for the accuracy, security, or completeness of the code,
or the information provided within the crate.

# General information
If you would like to add a commit or an issue, please do so using the GitHub link to the project:
- <https://github.com/rsadykhov/ecbdp-api>

# Examples

1. Query ECB Data Portal for data
 
```rust
use chrono::{FixedOffset, DateTime, TimeZone};
use ecbdp_api::{ECBDataPortal, Query, FlowRef, DataParameter, ECBResponse};
use ecbdp_api::parameter::data::{Detail, Format};

#[tokio::main]
async fn main() -> () {

    // Query
    let q: Query = Query::new()
        .flow_ref(FlowRef { agency_id: None, flow_id: "EXR".to_owned(), version: None, })
        .series_key("M.USD.EUR.SP00.A");
    
    // Parameters
    let hour: i32 = 3600;
    let datetime: DateTime<FixedOffset> = FixedOffset::east_opt(1 * hour).unwrap()
        .with_ymd_and_hms(2009, 05, 15, 14, 15, 0).unwrap();
    let parameters: Option<Vec<DataParameter<FixedOffset>>> = Some(vec![
        DataParameter::UpdatedAfter { datetime, },
        DataParameter::Detail { detail: Detail::DataOnly, },
        DataParameter::Format { format: Format::JSONData, }
    ]);
    
    // Backend
    let ecb_response: ECBResponse = ECBDataPortal::get_data(&q, parameters).await.unwrap();

    assert!(0 < ecb_response.datasets[0].series.iter().last().unwrap().1.observations.as_ref().unwrap().len());
    assert_eq!(ecb_response.structure.name, "Exchange Rates".to_owned());

}
```

2. Query ECB Data Portal for a schema

```rust
use ecbdp_api::{ECBDataPortal, Query, Resource, Context};

#[tokio::main]
async fn main() -> () {

    // Query
    let q: Query = Query::new()
        .resource(Resource::Schema)
        .context(Context::DataStructure)
        .agency_id("ECB")
        .resource_id("ECB_EXR1")
        .version("1.0");

    // Backend
    let schema: String = ECBDataPortal::get_schema(&q).await.unwrap();

    assert!(!schema.is_empty())

}
```

3. Query ECB Data Portal for metadata

```rust
use ecbdp_api::{ECBDataPortal, Query, Resource};

#[tokio::main]
async fn main() -> () {

    // Query
    let q: Query = Query::new()
        .resource(Resource::MetadataDataStructure)
        .agency_id("ECB")
        .resource_id("ECB_EXR1")
        .version("latest");

    // Backend
    let metadata: String = ECBDataPortal::get_metadata(&q, None).await.unwrap();

    assert!(!metadata.is_empty())

}
```