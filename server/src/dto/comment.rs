use crate::entity::{self, comment::CommentStatus, UserRole};
use chrono::{DateTime, Utc};
use serde::{ser::SerializeStruct, Serialize};
pub struct Comment {
    pub id: i64,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub status: CommentStatus,
    pub nickname: String,
    pub email: String,
    pub site: Option<String>,
    pub parent: Option<i64>,
    pub children: Vec<Comment>,
    pub role: UserRole,
    pub is_authed: bool,
}

impl From<entity::comment::Model> for Comment {
    fn from(item: entity::comment::Model) -> Self {
        let children = match item.children {
            Some(v) => v,
            None => vec![],
        };
        Self {
            id: item.id,
            created: item.created.unwrap_or(Utc::now()),
            modified: item.modified.unwrap_or(Utc::now()),
            status: item.status.unwrap_or(CommentStatus::Hidden),
            nickname: item.nickname.unwrap_or("".to_owned()),
            email: item.email.unwrap_or("".to_owned()),
            site: item.site,
            parent: item.parent,
            role: item.role,
            children: children
                .into_iter()
                .map(|child| Comment::from(child))
                .collect(),
            is_authed: false,
        }
    }
}

impl Serialize for Comment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Comment", 10)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("created", &self.created)?;
        state.serialize_field("nickname", &self.nickname)?;
        state.serialize_field("email", &self.email)?;
        state.serialize_field("site", &self.site)?;
        state.serialize_field("parent", &self.parent)?;
        state.serialize_field("children", &self.children)?;
        state.serialize_field("role", &self.role)?;
        if self.is_authed {
            state.serialize_field("modified", &self.modified)?;
        }
        state.end()
    }
}
