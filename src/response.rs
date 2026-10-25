use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AllResponse {
    pub moph_id_token_response: Option<MophIdTokenesponse>,
    pub provider_id_token_response: Option<ProviderIdTokenResponse>,
    pub provider_id_profile_response: Option<ProviderIdProfileResponse>,
}

impl AllResponse {

    pub(crate) fn set_moph_id_token_response(&mut self, moph_id_token_response: MophIdTokenesponse) {
        self.moph_id_token_response = Some(moph_id_token_response);
    }

    pub(crate) fn set_provider_id_token_response(&mut self, provider_id_token_response: ProviderIdTokenResponse) {
        self.provider_id_token_response = Some(provider_id_token_response);
    }

    pub(crate) fn set_provider_id_profile_response(&mut self, provider_id_profile_response: ProviderIdProfileResponse) {
        self.provider_id_profile_response = Some(provider_id_profile_response);
    }

    pub fn get_hash_cid_uppercase(&self) -> Option<String> {
        self.provider_id_profile_response.as_ref().map(|res| res.data.hash_cid.to_ascii_uppercase())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OauthRedirectParams {
    pub code: Option<String>,
    pub state: Option<String>,
    pub as_path: Option<String>,
    pub service_name: Option<String>,
    pub provider_logo: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MophIdTokenesponse {
    pub data: MophIdTokenResponseData,
    pub message: String,
    pub status: String,
    pub status_code: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MophIdTokenResponseData {
    pub access_token: String,
    pub account_id: String,
    pub expires_in: u64,
    pub token_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProviderIdTokenResponse {
    pub data: ProviderIdTokenResponseData,
    pub message: String,
    pub status: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProviderIdTokenResponseData {
    pub access_token: String,
    pub account_id: String,
    pub expiration_date: String,
    pub expires_in: u64,
    pub login_by: String,
    pub result: String,
    pub token_type: String,
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProviderIdProfileResponse {
    pub data: ProviderIdProfileResponseData,
    pub message: String,
    pub status: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProviderIdProfileResponseData {
    pub account_id: String,
    pub created_at: String,
    pub date_of_birth: String,
    pub email: String,
    pub firstname_en: String,
    pub firstname_th: String,
    /// lowercase hex
    pub hash_cid: String,
    pub lastname_en: String,
    pub lastname_th: String,
    pub name_eng: String,
    pub name_th: String,
    pub organization: Vec<Organization>,
    pub provider_id: String,
    pub special_title_en: Option<String>,
    pub special_title_th: Option<String>,
    pub title_en: Option<String>,
    pub title_th: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Organization {
    pub address: Address,
    pub affiliation: String,
    pub business_id: String,
    pub code9: String,
    pub department: String,
    // pub employee_job_lines: Vec<??>, TODO: add object structure
    pub expertise: String,
    pub expertise_id: String,
    pub hcode: String,
    pub hcode9: String,
    pub hname_eng: String,
    pub hname_th: String,
    pub hospital_main: bool,
    pub is_private_provider: bool,
    pub level: String,
    pub license_expired_date: Option<String>,
    pub license_id: String,
    pub license_id_verify: bool,
    pub ministry: String,
    pub moph_station_ref_code: Option<String>,
    pub position: String,
    pub position_id: String,
    pub special_expertise: Vec<SpecialExpertise>,
    pub tax_id: String,
    pub type_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Address {
    pub address: Option<String>,
    pub building: Option<String>,
    pub district: String,
    pub moo: Option<String>,
    pub province: String,
    pub soi: Option<String>,
    pub street: Option<String>,
    pub sub_district: String,
    pub zip_code: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpecialExpertise {
    pub id: String,
    pub name: String,
}
