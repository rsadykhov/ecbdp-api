use std::{fmt::Display, ops::Add};
use crate::error::Error;


#[derive(Clone, Copy, Debug, Default)]
/// Protocol
/// 
/// As of 28 January 2021 the web service is only available over `https`.
/// `http` calls made via a browser will be automatically redirected to `https`.
pub enum Protocol {
    HTTP,
    #[default]
    HTTPS,
}

impl Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HTTP => write!(f, "http"),
            Self::HTTPS => write!(f, "https"),
        }
    }
}


#[derive(Clone, Copy, Debug, Default)]
/// wsEntryPoint
/// 
/// The web service entry point.
pub enum WSEntryPoint {
    #[default]
    /// data-api.ecb.europa.eu/service/
    Main,
}

impl Display for WSEntryPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Main => write!(f, "data-api.ecb.europa.eu/service"),
        }
    }
}


#[derive(Clone, Copy, Debug, Default, PartialEq)]
/// Resource
/// 
/// - The resource for data queries is `data`.
/// - The resource for schema queries is `schema`.
pub enum Resource {
    #[default]
    Data,
    Schema,
    MetadataDataStructure,
    MetadataMetadataStructure,
    MetadataCategoryScheme,
    MetadataConceptScheme,
    MetadataCodeList,
    MetadataHierarchicalCodeList,
    MetadataOrganisationsScheme,
    MetadataAgencyScheme,
    MetadataDataProvidersScheme,
    MetadataDataConsumerScheme,
    MetadataOrganisationUnitScheme,
    MetadataDataFlow,
    MetadataMetadataFlow,
    MetadataReportingTaxonomy,
    MetadataProvisionAgreement,
    MetadataStructureSet,
    MetadataProcess,
    MetadataCategorisation,
    MetadataContentConstraint,
    MetadataAttachmentConstraint,
    MetadataStructure,
}

impl Resource {
    /// Returns all available `data` query resources.
    pub fn all_data_resources() -> Vec<Self> {
        vec![Self::Data]
    }

    /// Returns all available `schema` query resources.
    pub fn all_schema_resources() -> Vec<Self> {
        vec![Self::Schema]
    }

    /// Returns all available `metadata` query resources.
    pub fn all_metadata_resources() -> Vec<Self> {
        vec![
            Self::MetadataDataStructure,
            Self::MetadataMetadataStructure,
            Self::MetadataCategoryScheme,
            Self::MetadataConceptScheme,
            Self::MetadataCodeList,
            Self::MetadataHierarchicalCodeList,
            Self::MetadataOrganisationsScheme,
            Self::MetadataAgencyScheme,
            Self::MetadataDataProvidersScheme,
            Self::MetadataDataConsumerScheme,
            Self::MetadataOrganisationUnitScheme,
            Self::MetadataDataFlow,
            Self::MetadataMetadataFlow,
            Self::MetadataReportingTaxonomy,
            Self::MetadataProvisionAgreement,
            Self::MetadataStructureSet,
            Self::MetadataProcess,
            Self::MetadataCategorisation,
            Self::MetadataContentConstraint,
            Self::MetadataAttachmentConstraint,
            Self::MetadataStructure, 
        ]
    }
}

impl Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Data => write!(f, "data"),
            Self::Schema => write!(f, "schema"),
            Self::MetadataDataStructure => write!(f, "datastructure"),
            Self::MetadataMetadataStructure => write!(f, "metadatastructure"),
            Self::MetadataCategoryScheme => write!(f, "categoryscheme"),
            Self::MetadataConceptScheme => write!(f, "Conceptscheme"), // Capital `C` as per ECB Data Portal resource definition
            Self::MetadataCodeList => write!(f, "codelist"),
            Self::MetadataHierarchicalCodeList => write!(f, "hierarchicalcodelist"),
            Self::MetadataOrganisationsScheme => write!(f, "organisationsscheme"),
            Self::MetadataAgencyScheme => write!(f, "agencyscheme"),
            Self::MetadataDataProvidersScheme => write!(f, "dataprovidersscheme"),
            Self::MetadataDataConsumerScheme => write!(f, "dataconsumerscheme"),
            Self::MetadataOrganisationUnitScheme => write!(f, "organisationunitscheme"),
            Self::MetadataDataFlow => write!(f, "dataflow"),
            Self::MetadataMetadataFlow => write!(f, "metadataflow"),
            Self::MetadataReportingTaxonomy => write!(f, "reportingtaxonomy"),
            Self::MetadataProvisionAgreement => write!(f, "provisionagreement"),
            Self::MetadataStructureSet => write!(f, "structureset"),
            Self::MetadataProcess => write!(f, "process"),
            Self::MetadataCategorisation => write!(f, "categorisation"),
            Self::MetadataContentConstraint => write!(f, "contentconstraint"),
            Self::MetadataAttachmentConstraint => write!(f, "attachmentconstraint"),
            Self::MetadataStructure => write!(f, "structure"),
        }
    }
}


#[derive(Clone, Debug, Default)]
/// FlowRef (Defining the dataflow reference)
///
/// A reference to the dataflow describing the data that needs to be returned.
/// The syntax is the identifier of the agency maintaining the dataflow, followed by the identifier of the dataflow,
/// followed by the dataflow version, separated by a comma (,). 
/// For example: AGENCY_ID,FLOW_ID,VERSION
///
/// If the parameter contains only one of these three elements, it is considered to be the identifier of the dataflow.
/// The value for the identifier of the agency maintaining the dataflow will default to all,
/// while the value for the dataflow version will default to latest.
///
/// If the string contains only two of these three elements, they are the identifier of the agency maintaining the dataflow
/// and the identifier of the dataflow. The value for the dataflow version will default to latest.
///
/// In order to see the dataflows available in the ECB Data Portal, a metadata query for all dataflows can be performed:
/// - <https://data-api.ecb.europa.eu/service/dataflow>
pub struct FlowRef {
    /// The identifier of the maintainer of the context
    pub agency_id: Option<String>,
    /// The identifier of the context, such as EXR for the Dataflow about exchange rates maintained by the ECB
    /// 
    /// Note: On the ECB Data Portal the series are prefixed with their flow identifier. For example,
    /// `EXR.M.USD.EUR.SP00.A` is the series key on the ECB Data Portal, but `EXR` is an `flow_id` and `M.USD.EUR.SP00.A`
    /// is the `series_key` in this implementation.
    pub flow_id: String,
    /// The version of the context to be returned. When the version number is not supplied, the latest version is returned
    pub version: Option<String>,
}

impl Display for FlowRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let agency_id: String = self.agency_id.as_ref().map_or("all".to_owned(), |v| v.clone() );
        let flow_id: String = self.flow_id.clone();
        let version: String = self.version.as_ref().map_or("latest".to_owned(), |v| v.clone() );
        write!(f, "{agency_id},{flow_id},{version}")
    }
}


#[derive(Clone, Copy, Debug, Default)]
/// The context determines the constraints that need to be taken into account when generating the schema.
pub enum Context {
    #[default]
    /// Constraints attached to the DSD will be used in the schema
    DataStructure,
    /// Constraints attached to the Dataflow and to the DSD referenced by the Dataflow will be used in the schema
    DataFlow,
    /// Constraints attached to the provision agreement, the Dataflow referenced by the agreement and to the DSD
    /// referenced by the Dataflow will be available in the schema
    ProvisionAgreement,
}

impl Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DataStructure => write!(f, "datastructure"),
            Self::DataFlow => write!(f, "dataflow"),
            Self::ProvisionAgreement => write!(f, "provisionagreement"),
        }
    }
}


#[derive(Clone, Debug, Default)]
/// Query string generator.
/// 
/// All the data stored in the ECB Data Portal can be retrieved using the following query strings:
/// - Data Queries: `protocol://wsEntryPoint/resource/flowRef/key?parameters`
/// - Schema Queries: `protocol://wsEntryPoint/resource/context/agencyID/resourceID/version`
/// - Metadata Queries: `protocol://wsEntryPoint/resource/agencyID/resourceID/version?parameters`
/// 
/// # Examples
/// 
/// 1. Data query
/// 
/// ```rust
/// use ecbdp_api::{Resource, FlowRef, Query};
/// 
/// let query: Query = Query::new()
///     .flow_ref(FlowRef { agency_id: None, flow_id: String::from("EXR"), version: None, })
///     .series_key("M.USD.EUR.SP00.A");
/// 
/// assert_eq!(query.generate_url().unwrap(), "https://data-api.ecb.europa.eu/service/data/all,EXR,latest/M.USD.EUR.SP00.A".to_owned())
/// ```
/// 
/// 2. Schema query
/// 
/// ```rust
/// use ecbdp_api::{Resource, Context, Query};
/// 
/// let query: Query = Query::new()
///     .resource(Resource::Schema)
///     .context(Context::DataStructure)
///     .agency_id("ECB")
///     .resource_id("ECB_EXR1")
///     .version("1.0");
/// 
/// assert_eq!(query.generate_url().unwrap(), "https://data-api.ecb.europa.eu/service/schema/datastructure/ECB/ECB_EXR1/1.0".to_owned());
/// ```
/// 
/// 3. Metadata query
/// 
/// ```rust
/// use ecbdp_api::{Resource, Query};
/// 
/// let query: Query = Query::new()
///     .resource(Resource::MetadataCodeList)
///     .agency_id("all")
///     .resource_id("all")
///     .version("latest");
/// 
/// assert_eq!(query.generate_url().unwrap(), "https://data-api.ecb.europa.eu/service/codelist/all/all/latest".to_owned());
/// ```
pub struct Query {
    // General query parameters
    pub protocol: Protocol,
    pub ws_entry_point: WSEntryPoint,
    pub resource: Resource,
    // Overlapping query parameters
    /// The identifier of the maintainer of the context
    pub agency_id: Option<String>,
    /// The identifier of the context, such as EXR for the Dataflow about exchange rates maintained by the ECB
    pub resource_id: Option<String>,
    /// The version of the context to be returned. When the version number is not supplied, the latest version is returned.
    pub version: Option<String>,
    // Data-specific query parameters
    pub flow_ref: Option<FlowRef>,
    /// Series key of the series.
    /// 
    /// Note: On the ECB Data Portal the series are prefixed with their flow identifier. For example,
    /// `EXR.M.USD.EUR.SP00.A` is the series key on the ECB Data Portal, but `EXR` is an `flow_id` and `M.USD.EUR.SP00.A`
    /// is the `series_key` in this implementation.
    pub series_key: Option<String>,
    // Schema=specific query parameters
    pub context: Option<Context>,
}

impl Query {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn protocol(mut self, protocol: Protocol) -> Self {
        self.protocol = protocol;
        self
    }

    pub fn ws_entry_point(mut self, ws_entry_point: WSEntryPoint) -> Self {
        self.ws_entry_point = ws_entry_point;
        self
    }

    pub fn resource(mut self, resource: Resource) -> Self {
        self.resource = resource;
        self
    }

    pub fn flow_ref(mut self, flow_ref: FlowRef) -> Self {
        self.flow_ref = Some(flow_ref);
        self
    }

    pub fn series_key(mut self, series_key: &str) -> Self {
        self.series_key = Some(series_key.to_owned());
        self
    }

    pub fn context(mut self, context: Context) -> Self {
        self.context = Some(context);
        self
    }

    pub fn agency_id(mut self, agency_id: &str) -> Self {
        self.agency_id = Some(agency_id.to_owned());
        self
    }

    pub fn resource_id(mut self, resource_id: &str) -> Self {
        self.resource_id = Some(resource_id.to_owned());
        self
    }

    pub fn version(mut self, version: &str) -> Self {
        self.version = Some(version.to_owned());
        self
    }

    pub fn validate_query(&self, permitted_resources: Vec<Resource>) -> Result<(), Error> {
        if !permitted_resources.contains(&self.resource) {
            return Err(Error::WrongResourceRequested);
        }
        Ok(())
    }

    /// Generates a query URL.
    /// 
    /// Note: This url does not contain parameters.
    pub fn generate_url(&self) -> Result<String, Error> {
        // General query parameters
        let mut query: String = format!("{}://{}/{}", self.protocol.to_string(), self.ws_entry_point.to_string(), self.resource.to_string());
        query = match self.resource {
            // Data-specific query parameters
            Resource::Data => {
                query
                    .add("/").add(&self.flow_ref.as_ref().ok_or(Error::MissingQueryAttribute { attribute: "reference to dataflow".to_owned(), })?.to_string())
                    .add("/").add(&self.series_key.as_ref().ok_or(Error::MissingQueryAttribute { attribute: "series key".to_owned(), })?)
            },
            // Schema-specific query parameters
            Resource::Schema => {
                query
                    .add("/").add(&self.context.as_ref().ok_or(Error::MissingQueryAttribute { attribute: "schema context".to_owned() })?.to_string())
                    .add("/").add(&self.agency_id.as_ref().ok_or(Error::MissingQueryAttribute { attribute: "agency ID".to_owned() })?)
                    .add("/").add(&self.resource_id.as_ref().ok_or(Error::MissingQueryAttribute { attribute: "resource ID".to_owned() })?)
                    .add("/").add(&self.version.as_ref().ok_or(Error::MissingQueryAttribute { attribute: "version number".to_owned() })?)
            },
            // Metadata-specific query parameters
            _ => {
                query
                    .add("/").add(&self.agency_id.as_ref().ok_or(Error::MissingQueryAttribute { attribute: "agency ID".to_owned() })?)
                    .add("/").add(&self.resource_id.as_ref().ok_or(Error::MissingQueryAttribute { attribute: "resource ID".to_owned() })?)
                    .add("/").add(&self.version.as_ref().ok_or(Error::MissingQueryAttribute { attribute: "version number".to_owned() })?)
            },
        };
        Ok(query)
    }
}


#[cfg(test)]
mod tests {
    use crate::query::{Resource, FlowRef, Context, Query};

    #[test]
    fn unit_test_generate_url() -> () {
        // Data-specific query
        let query: Query = Query::new()
            .flow_ref(FlowRef { agency_id: None, flow_id: String::from("EXR"), version: None, })
            .series_key("M.USD.EUR.SP00.A");
        assert_eq!(query.generate_url().unwrap(), "https://data-api.ecb.europa.eu/service/data/all,EXR,latest/M.USD.EUR.SP00.A".to_owned());
        // Schema-specific query
        let query: Query = Query::new()
            .resource(Resource::Schema)
            .context(Context::DataStructure)
            .agency_id("ECB")
            .resource_id("ECB_EXR1")
            .version("1.0");
        assert_eq!(query.generate_url().unwrap(), "https://data-api.ecb.europa.eu/service/schema/datastructure/ECB/ECB_EXR1/1.0".to_owned());
        // Metadata-specific query
        let query: Query = Query::new()
            .resource(Resource::MetadataCodeList)
            .agency_id("all")
            .resource_id("all")
            .version("latest");
        assert_eq!(query.generate_url().unwrap(), "https://data-api.ecb.europa.eu/service/codelist/all/all/latest".to_owned());
    }
}