/*
 * TVDB API V4
 *
 * Documentation of [TheTVDB](https://thetvdb.com/) API V4. All related information is linked from our [Github repo](https://github.com/thetvdb/v4-api). You might also want to use our [Postman collection] (https://www.getpostman.com/collections/7a9397ce69ff246f74d0) ## Authentication 1. Use the /login endpoint and provide your API key as \"apikey\". If you have a user-supported key, also provide your subscriber PIN as \"pin\". Otherwise completely remove \"pin\" from your call. 2. Executing this call will provide you with a bearer token, which is valid for 1 month. 3. Provide your bearer token for subsequent API calls by clicking Authorize below or including in the header of all direct API calls: `Authorization: Bearer [your-token]`  ## Notes 1. \"score\" is a field across almost all entities.  We generate scores for different types of entities in various ways, so no assumptions should be made about the meaning of this value.  It is simply used to hint at relative popularity for sorting purposes.
 *
 * The version of the OpenAPI document: 4.7.10
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// SeasonBaseRecord : season genre record
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SeasonBaseRecord {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "imageType", skip_serializing_if = "Option::is_none")]
    pub image_type: Option<i32>,
    #[serde(rename = "lastUpdated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nameTranslations", skip_serializing_if = "Option::is_none")]
    pub name_translations: Option<Vec<String>>,
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    #[serde(
        rename = "overviewTranslations",
        skip_serializing_if = "Option::is_none"
    )]
    pub overview_translations: Option<Vec<String>>,
    #[serde(rename = "companies", skip_serializing_if = "Option::is_none")]
    pub companies: Option<models::Companies>,
    #[serde(rename = "seriesId", skip_serializing_if = "Option::is_none")]
    pub series_id: Option<i64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Box<models::SeasonType>>,
    #[serde(rename = "year", skip_serializing_if = "Option::is_none")]
    pub year: Option<String>,
}

impl SeasonBaseRecord {
    /// season genre record
    pub fn new() -> SeasonBaseRecord {
        SeasonBaseRecord {
            id: None,
            image: None,
            image_type: None,
            last_updated: None,
            name: None,
            name_translations: None,
            number: None,
            overview_translations: None,
            companies: None,
            series_id: None,
            r#type: None,
            year: None,
        }
    }
}
