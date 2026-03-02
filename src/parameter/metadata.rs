use std::fmt::Display;


#[derive(Clone, Copy, Debug, Default)]
/// Using the detail parameter, you can specify the desired amount of information to be returned.
/// For example, it is possible to instruct the web service to return only basic information about the resource
/// (i.e., its id, agency id, version and name. This is also known as a stub in SDMX).
pub enum Detail {
    #[default]
    /// All available information for all artefacts will be returned. This is the default.
    Full,
    /// All artefacts will be returned as stubs
    AllStubs,
    /// The referenced artefacts will be returned as stubs
    ReferenceStubs,
}

impl Display for Detail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Full => write!(f, "full"),
            Self::AllStubs => write!(f, "allstubs"),
            Self::ReferenceStubs => write!(f, "referencestubs"),
        }
    }
}


#[derive(Clone, Copy, Debug, Default)]
/// Using the references parameter, you can instruct the web service to return (or exclude)
/// the artefacts that use or are referenced by the artefact matching the query.
/// This includes, for example, the codelists and Concepts used by the DSD matching the query.
/// You can also retrieve the artefacts that use the matching artefact, such as the Dataflows that use the DSD matching the query.
pub enum References {
    #[default]
    /// No references will be returned. This is the default.
    None,
    /// The artefacts that use the artefact matching the query (for example, the Dataflows that use the DSD matching the query) will be returned
    Parents,
    /// The artefacts that use the artefact matching the query, as well as the artefacts referenced by these artefacts, will be returned
    ParentsAndSiblings,
    /// The artefacts referenced by the matching artefact (for example, the Concept schemes and codelists used in a DSD) will be returned
    Children,
    /// References of references, up to any level, will also be returned
    Descendants,
    /// The combination of `ParentsAndSiblings` and `Descendants`
    All,
}

impl Display for References {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Parents => write!(f, "parents"),
            Self::ParentsAndSiblings => write!(f, "parentsandsiblings"),
            Self::Children => write!(f, "children"),
            Self::Descendants => write!(f, "descendants"),
            Self::All => write!(f, "all"),
        }
    }
}


#[derive(Clone, Debug)]
/// Parameter types for `metadata` queries.
pub enum MetadataParameter {
    /// Using the detail parameter, you can specify the desired amount of information to be returned.
    /// For example, it is possible to instruct the web service to return only basic information about the resource
    /// (i.e., its id, agency id, version and name. This is also known as a stub in SDMX).
    Detail { detail: Detail, },
    /// Using the references parameter, you can instruct the web service to return (or exclude)
    /// the artefacts that use or are referenced by the artefact matching the query.
    /// This includes, for example, the codelists and Concepts used by the DSD matching the query.
    /// You can also retrieve the artefacts that use the matching artefact, such as the Dataflows that use the DSD matching the query.
    References { references: References, },
}

impl Display for MetadataParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Detail { detail } => write!(f, "detail={}", detail.to_string()),
            Self::References { references } => write!(f, "references={}", references.to_string()),
        }
    }
}