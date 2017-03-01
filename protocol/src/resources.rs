use super::*;
use std::collections::HashMap;

pub trait Identified {
    type I : Id;
    fn id(&self) -> &Self::I;
}

pub trait Id {
    fn stringify(&self) -> String;
}

macro_rules! uuid_id {
    ( #[$doc:meta] $name:ident ) => {
        #[$doc]
        #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $name(pub Uuid);

        impl Default for $name {
            fn default() -> $name {
                $name(Uuid::new(::uuid::UuidVersion::Random).unwrap())
            }
        }

        impl Id for $name {
            fn stringify(&self) -> String {
                self.0.to_string()
            }
        }
    }
}

/// Basic description of an agent, e.g. participants, clerks, and admins.
///
/// Primary use is identification, including allowing services to perform access control and logging.
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Agent {
    pub id: AgentId,
    // /// Key used for verifying signatures from agent, if any.
    pub verification_key: Option<VerificationKey>,
}

uuid_id!{ #[doc="Unique agent identifier."] AgentId }

// FIXME should we macro_rule this ?
impl Identified for Agent {
    type I = AgentId;
    fn id(&self) -> &AgentId {
        &self.id
    }
}

/// Extended profile of an agent, providing information intended for increasing trust such as name and social handles.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub owner: AgentId,
    pub name: Option<String>,
    pub twitter_id: Option<String>,
    pub keybase_id: Option<String>,
    pub website: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Signed<M>
where M: Clone + ::std::fmt::Debug + PartialEq + ::serde::Serialize + ::serde::Deserialize
{
    pub signature: Signature,
    pub signer: AgentId,
    pub body: M
}

impl<M> std::ops::Deref for Signed<M>
where M: Clone + ::std::fmt::Debug + PartialEq + ::serde::Serialize + ::serde::Deserialize
{
    type Target = M;
    fn deref(&self) -> &M {
        &self.body
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Labeled<ID,M>
where M: Clone + ::std::fmt::Debug + PartialEq + ::serde::Serialize + ::serde::Deserialize,
      ID: Id + Clone + ::std::fmt::Debug + PartialEq + ::serde::Serialize + ::serde::Deserialize,
{
    pub id: ID,
    pub body: M
}

impl<ID,M> Identified for Labeled<ID,M>
where M: Clone + ::std::fmt::Debug + PartialEq + ::serde::Serialize + ::serde::Deserialize,
      ID: Id + Clone + ::std::fmt::Debug + PartialEq + ::serde::Serialize + ::serde::Deserialize
{
    type I = ID;
    fn id(&self) -> &ID {
        &self.id
    }
}


pub struct LabelledVerificationKeypairId(pub Uuid);

uuid_id!{ #[doc="Unique encryption key identifier."] EncryptionKeyId }

pub type SignedEncryptionKey = Signed<Labeled<EncryptionKeyId, EncryptionKey>>;


/// Description of an aggregation.
pub struct Aggregation {
    pub id: AggregationId,
    pub title: String,
    /// Fixed dimension of input and output vectors.
    pub vector_dimension: usize,
    // pub modulus: i64,  // TODO move here instead of in the primitives?
    /// Recipient of output vector.
    pub recipient: AgentId,
    /// Encryption key to be used for encryptions to the recipient.
    pub recipient_key: EncryptionKeyId,
    /// Masking scheme and parameters to be used between the recipient and the committee.
    pub masking_scheme: LinearMaskingScheme,
    /// Scheme and parameters to be used for secret sharing between the clerks in the committee.
    pub committee_sharing_scheme: LinearSecretSharingScheme,
    /// Scheme and parameters to be used for encrypting masks for the recipient.
    pub recipient_encryption_scheme: AdditiveEncryptionScheme,
    /// Scheme and parameters to be used for encryption masked shares for the committee.
    pub committee_encryption_scheme: AdditiveEncryptionScheme,
}

/// Unique aggregation identifier.
#[derive(Clone, Debug)] // TODO could we use Copy instead?
pub struct AggregationId(pub Uuid);

/// Description of committee elected for an aggregation.
pub struct Committee {
    pub aggregation: AggregationId,
    /// Order of the clerks in the committee.
    pub clerk_order: Vec<AgentId>,
    /// Encryption keys to be used.
    ///
    /// Note that while this could simply be a vector, it's easier to work with a map.
    pub clerk_keys: HashMap<AgentId, EncryptionKeyId>,
}

/// Description of a participant's input to an aggregation.
#[derive(Debug)]
pub struct Participation {
    /// Unique identifier of participation.
    ///
    /// This allows a service to keep track, and possible discard, multiple participations from each participant.
    pub id: ParticipationId,
    pub participant: AgentId,
    pub aggregation: AggregationId,
    pub encryptions: HashMap<AgentId, Encryption>,
}

/// Unique participatin identifer.
#[derive(Debug)]
pub struct ParticipationId(pub Uuid);

/// Partial aggregation job to be performed by a clerk.
///
/// Includes all inputs needed.
pub struct ClerkingJob {
    pub id: ClerkingJobId,
    pub clerk: AgentId,
    pub aggregation: AggregationId,
    pub encryptions: Vec<Encryption>,
}

/// Result of a partial aggregation job performed by a clerk.
pub struct ClerkingResult {
    pub job: ClerkingJobId,
    pub aggregation: AggregationId,
    pub encryption: Encryption,
}

#[derive(Clone)]
pub struct ClerkingJobId(pub Uuid);

/// Current status of an aggregation.
pub struct AggregationStatus {
    pub aggregation: AggregationId,
    /// Current number of participations received from the users.
    pub number_of_participations: usize,
    /// Current number of clerking results received from the clerks.
    pub number_of_clerking_results: usize,
    /// Indication of whether a result of the aggregation can be produced from the current clerking results.
    pub result_ready: bool,
}

/// Result of an aggregation, including output.
pub struct AggregationResult {
    pub aggregation: AggregationId,
    /// Number of participation used in this result.
    pub number_of_participations: usize,
    /// Number of clerking results used in this result.
    pub number_of_clerking_results: usize,
    /// Result of the aggregation.
    pub encryptions: Vec<Encryption>,
}
