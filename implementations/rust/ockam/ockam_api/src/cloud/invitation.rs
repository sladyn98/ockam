use minicbor::{Decode, Encode};
use std::borrow::Cow;

#[derive(serde::Deserialize, Encode, Decode, Debug)]
#[cbor(index_only)]
pub enum Scope {
    #[n(0)]
    SpaceScope,
    #[n(1)]
    ProjectScope,
}

#[derive(serde::Deserialize, Encode, Decode, Debug)]
#[cbor(index_only)]
pub enum State {
    #[n(0)]
    Pending,
    #[n(1)]
    Accepted,
    #[n(2)]
    Rejected,
}

#[derive(Decode, Encode, Debug)]
#[cbor(map)]
pub struct CreateInvitation<'a> {
    #[b(0)]
    pub identity: Cow<'a, str>,
    #[b(1)]
    pub invitee: Cow<'a, str>,
    #[b(2)]
    pub scope: Scope,
    #[b(3)]
    pub space_id: Cow<'a, str>,
    #[b(4)]
    pub project_id: Option<Cow<'a, str>>,
}

#[derive(Decode, Encode, Debug)]
#[cbor(map)]
pub struct Invitation<'a> {
    #[b(0)]
    pub id: Cow<'a, str>,
    #[b(1)]
    pub inviter: Cow<'a, str>,
    #[b(2)]
    pub invitee: Cow<'a, str>,
    #[b(3)]
    pub scope: Scope,
    #[b(4)]
    pub state: State,
    #[b(5)]
    pub space_id: Cow<'a, str>,
    #[b(6)]
    pub project_id: Option<Cow<'a, str>>,
}

impl<'a> CreateInvitation<'a> {
    pub fn new<S: Into<Cow<'a, str>>>(
        identity: S,
        invitee: S,
        space_id: S,
        project_id: Option<S>,
    ) -> Self {
        Self {
            identity: identity.into(),
            invitee: invitee.into(),
            scope: project_id
                .as_ref()
                .map_or_else(|| Scope::SpaceScope, |_| Scope::ProjectScope),
            space_id: space_id.into(),
            project_id: project_id.map(|s| s.into()),
        }
    }
}