use bson::{oid::ObjectId, Bson::Document};
use mongodb::db::ThreadedDatabase;
use mongodb::{Client, ThreadedClient};

use crate::shorty;

const BASE: u64 = 62;
const ALPHABET: [u8; BASE as usize] = [
    b'n', b'H', b'z', b'2', b'Q', b'q', b'F', b'4', b'p', b'5', b'1', b'G', b'7', b'a', b'9', b'e',
    b'f', b'6', b'm', b'X', b'd', b'g', b'l', b'B', b'o', b'8', b's', b'V', b't', b'L', b'w', b'R',
    b'j', b'k', b'P', b'x', b'y', b'S', b'A', b'C', b'r', b'N', b'h', b'M', b'D', b'E', b'U', b'i',
    b'T', b'J', b'0', b'K', b'b', b'O', b'u', b'v', b'U', b'c', b'W', b'Y', b'3', b'Z',
];

#[derive(Debug, Deserialize, Serialize)]
pub struct ShortyDocument {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub url: String,
    pub original: String,
    pub constructed: String,
    pub ga_campaign: GoogleAnalyticsCampaign,
    pub created: bson::UtcDateTime,
    pub accessed: Option<bson::UtcDateTime>,
    pub visits: i32,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct GoogleAnalyticsCampaign {
    pub utm_source: Option<String>,
    pub utm_campaign: Option<String>,
    pub utm_medium: Option<String>,
    pub utm_content: Option<String>,
    pub utm_term: Option<String>,
}

impl From<&shorty::GoogleAnalyticsCampaign> for GoogleAnalyticsCampaign {
    /// Internal conversion between the Rust struct and Protobuf
    fn from(value: &shorty::GoogleAnalyticsCampaign) -> Self {
        Self {
            utm_source: if value.utm_source.is_empty() {
                None
            } else {
                Some(value.utm_source.clone())
            },
            utm_campaign: if value.utm_campaign.is_empty() {
                None
            } else {
                Some(value.utm_campaign.clone())
            },
            utm_medium: if value.utm_medium.is_empty() {
                None
            } else {
                Some(value.utm_medium.clone())
            },
            utm_content: if value.utm_content.is_empty() {
                None
            } else {
                Some(value.utm_content.clone())
            },
            utm_term: if value.utm_term.is_empty() {
                None
            } else {
                Some(value.utm_term.clone())
            },
        }
    }
}

impl ShortyDocument {
    /// Find an URL in the database
    fn find_url(args: &shorty::ShortenRequest, client: &Client) -> Option<ShortyDocument> {
        let coll = client.db("shorty").collection("shortenedurls");

        let mut criteria: bson::Document = doc! {
            "original" => args.url.clone().to_ascii_lowercase().trim()
        };

        let campaign: shorty::GoogleAnalyticsCampaign =
            args.campaign
                .clone()
                .unwrap_or(shorty::GoogleAnalyticsCampaign {
                    ..Default::default()
                });

        if !campaign.utm_content.is_empty() {
            criteria.insert_bson("ga_campaign.utm_content".to_string(), "".to_string().into());
        }

        let result = coll.find_one(Some(criteria.clone()), None).ok();

        match result {
            Some(doc) => match doc {
                Some(d) => {
                    let document: ShortyDocument =
                        bson::from_bson(bson::Bson::Document(d)).unwrap();

                    Some(document)
                }
                None => return None,
            },
            None => return None,
        }
    }

    /// Increment counter to existing document
    pub fn add_counter(id: bson::oid::ObjectId, _counter: i32, client: &Client) -> bool {
        let coll = client.db("shorty").collection("shortenedurls");

        let update_result = coll
            .update_one(
                doc! {
                  "_id" => id
                },
                doc! {
                  "$inc" => {
                    "visits": 1
                  }
                },
                None,
            )
            .ok();

        match update_result {
            Some(r) => r.matched_count > 0,
            None => false,
        }
    }

    /// Find the original shorty
    pub fn find_original(url: &str, client: &Client) -> Option<ShortyDocument> {
        let coll = client.db("shorty").collection("shortenedurls");

        let criteria: bson::Document = doc! {
            "url" => url.trim()
        };

        let result = coll.find_one(Some(criteria.clone()), None).ok();

        println!("{:?}", result);

        match result {
            Some(doc) => match doc {
                Some(d) => {
                    let document: ShortyDocument =
                        bson::from_bson(bson::Bson::Document(d)).unwrap();

                    Some(document)
                }
                None => return None,
            },
            None => return None,
        }
    }

    /// Save shorty to database
    fn save(&self, client: &Client) -> bool {
        let coll: mongodb::coll::Collection = client.db("shorty").collection("shortenedurls");
        let ser = bson::to_bson(self).unwrap(); // TODO handle errors correctly
        if let Document(document) = ser {
            let result = coll.insert_one(document, None);
            match result {
                Ok(_) => true,
                Err(e) => {
                    error!("Error saving shortened document: {:?}", e);
                    false
                }
            }
        } else {
            false
        }
    }
}

/// Shorten URL
pub fn shorten(req: &shorty::ShortenRequest, client: &Client) -> Option<shorty::Shorty> {
    let url = join_url(req);

    let found = ShortyDocument::find_url(&req, &client);

    match found {
        Some(f) => Some(shorty::Shorty {
            id: f.id.to_hex(),
            url: f.url,
        }),
        None => {
            let id = ObjectId::new().unwrap();
            let id_ref = create_short_segment(&id.to_hex());
            // create a new document
            let newdoc = ShortyDocument {
                // create shortened segment
                id,
                original: req.url.to_owned(),
                accessed: None,
                constructed: url,
                created: chrono::Utc::now().into(),
                ga_campaign: req.campaign.as_ref().unwrap().into(),
                url: format!("{}/{}", std::env::var("SHORTY_PREFIX").unwrap(), id_ref),
                visits: 0,
            };
            let saved = newdoc.save(client);
            match saved {
                false => None,
                true => Some(shorty::Shorty {
                    id: newdoc.id.to_hex(),
                    url: newdoc.url,
                }),
            }
        }
    }
}

fn num_to_base62(mut num: u64) -> String {
    let mut bytes = Vec::new();

    if num == 0 {
        // return a single character
        return "n".to_owned();
    }

    while num > 0 {
        bytes.push(ALPHABET[(num % BASE) as usize]);
        num /= BASE
    }

    bytes.reverse();

    String::from_utf8(bytes).unwrap()
}

/// Extract the first 8 and last 4 characters of the ID
/// 
/// Note: if rewriting to support uuid, please change the logic to extract last 4 characters instead
fn create_short_segment(id: &str) -> String {
    let obj = &id[0..8];
    let last = &id[20..24];
    let last = format!("{}{}", last, obj);
    let hex = u64::from_str_radix(&last, 16).unwrap();
    let share = num_to_base62(hex);
    share
}

/// Join the original URL with Google Analytics stuff
fn join_url(request: &shorty::ShortenRequest) -> String {
    let mut string = String::new();
    string.push_str(&request.url.to_ascii_lowercase().trim());
    let campaign = request
        .campaign
        .clone()
        .unwrap_or(shorty::GoogleAnalyticsCampaign::default());

    string.push('?');

    if !campaign.utm_medium.is_empty() {
        string.push_str(&format!("utm_medium={}&", &campaign.utm_medium.trim()));
    }
    if !campaign.utm_campaign.is_empty() {
        string.push_str(&format!("utm_campaign={}&", &campaign.utm_campaign.trim()));
    }
    if !campaign.utm_content.is_empty() {
        string.push_str(&format!("utm_content={}&", &campaign.utm_content.trim()));
    }
    if !campaign.utm_source.is_empty() {
        string.push_str(&format!("utm_source={}&", &campaign.utm_source.trim()));
    }
    if !campaign.utm_term.is_empty() {
        string.push_str(&format!("utm_term={}&", &campaign.utm_term.trim()));
    }

    string
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shorty_mongo;

    #[test]
    fn test_encode() {
        assert_eq!(shorty_mongo::num_to_base62(1254654), "qfol");
    }

    #[test]
    fn test_create_short_segment() {
        let shortened = create_short_segment("5916ac09028d015664bd4f95".into());
        assert_eq!("obkm5d78", shortened);
        let shortened = create_short_segment("5d291a7845cd7b75c40124c3".into());
        assert_eq!("GLCuBLsT", shortened);
    }
}
