use crate::models::openapi::member::Member;
use easy_dynamodb::Document;
use dto::CryptoStance;

// NOTE: This is a real model and recommended to be moved to shared_models
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Default)]
pub struct AssemblyMember {
    id: String,
    r#type: String,
    code: String, // code could be duplicated by language

    created_at: u64,
    updated_at: u64,
    deleted_at: Option<u64>,

    name: Option<String>,
    party: Option<String>,
    district: Option<String>,
    // stance: CryptoStance, // consider update logic
    image_url: Option<String>,

    // Indexes, if deleted_at is set, all values of indexes must be empty.
    gsi1: String, // language
    gsi2: String,
}

impl Document for AssemblyMember {
    fn document_type() -> String {
        "assembly_member".to_string()
    }
}

impl AssemblyMember {
    pub fn new(
        id: String,
        code: String,
        name: String,
        party: String,
        district: String,
        image_url: String,
    ) -> Self {
        let now = chrono::Utc::now().timestamp_nanos_opt().unwrap_or_else(|| { log::error!("Failed to get timestamp"); 0 }) as u64;

        Self {
            id,
            code,
            r#type: AssemblyMember::document_type(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
            name: Some(name),
            party: Some(party),
            district: Some(district),
            image_url: Some(image_url),
            gsi1: "".to_string(),
            gsi2: "".to_string(),
        }
    }
}

impl TryFrom<(String, String, &str, &Member)> for AssemblyMember {
    type Error = String;
    fn try_from(
        (code, image_url, lang, member): (String, String, &str, &Member),
    ) -> Result<Self, Self::Error> {
        let now = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64;

        Ok(Self {
            id: format!("{}-{}", lang, code),
            code: member.code.clone(),
            r#type: AssemblyMember::document_type(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
            name: Some(member.name.clone()),
            party: Some(member.party.clone()),
            district: Some(member.district.clone()),
            image_url: Some(image_url),
            gsi1: lang.to_string(),
            gsi2: "".to_string(),
        })
    }
}