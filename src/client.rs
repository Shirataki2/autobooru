use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BooruClient {
    client: reqwest::Client,
    endpoint: String,
    pub username: String,
    pub api_key: String,
}

impl BooruClient {
    pub fn new(username: String, api_key: String) -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.93 Safari/537.36")
                .gzip(true)
                .build()
                .unwrap(),
            endpoint: "https://danbooru.donmai.us".to_string(),
            username,
            api_key,
        }
    }

    pub async fn get(
        &self,
        path: &str,
        query: Option<HashMap<String, String>>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let mut query = match query {
            Some(query) => query,
            None => HashMap::new(),
        };
        query.insert("login".into(), self.username.clone());
        query.insert("api_key".into(), self.api_key.clone());
        let mut url = format!("{}/{}", self.endpoint, path);
        if !query.is_empty() {
            url.push('?');
            let q = query
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");
            url.push_str(&q);
        }
        self.client
            .get(url)
            .header("ContentType", "application/x-www-form-urlencoded")
            .send()
            .await
    }

    pub async fn get_first_post(&self, tag: &str) -> Result<Post, reqwest::Error> {
        let query = hashmap! {
            "limit".into() => "1".into(),
            "tags".into() => tag.into(),
        };
        let res = self.get("posts.json", Some(query)).await?;
        info!("res: {:?}", res);
        let posts: Vec<Post> = res.json().await?;
        Ok(posts[0].clone())
    }

    pub async fn get_random_post(&self, tag: &str) -> Result<Post, reqwest::Error> {
        let query = hashmap! {
            "tags".into() => tag.into(),
        };
        let res = self.get("posts/random.json", Some(query)).await?;
        info!("res: {:?}", res);
        let post: Post = res.json().await?;
        Ok(post)
    }
}

pub type Timestamp = chrono::DateTime<chrono::Utc>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub created_at: Timestamp,
    pub uploader_id: i64,
    pub score: i32,
    pub source: Option<String>,
    pub md5: String,
    pub last_comment_bumped_at: Option<Timestamp>,
    pub rating: String,
    pub image_width: i64,
    pub image_height: i64,
    pub tag_string: String,
    pub fav_count: i64,
    pub file_ext: String,
    pub last_noted_at: Option<Timestamp>,
    pub parent_id: Option<i64>,
    pub has_children: bool,
    pub approver_id: Option<i64>,
    pub tag_count_general: i64,
    pub tag_count_artist: i64,
    pub tag_count_character: i64,
    pub tag_count_copyright: i64,
    pub file_size: u64,
    pub up_score: i64,
    pub down_score: i64,
    pub is_pending: bool,
    pub is_flagged: bool,
    pub is_deleted: bool,
    pub tag_count: i64,
    pub updated_at: Option<Timestamp>,
    pub is_banned: bool,
    pub pixiv_id: Option<u64>,
    pub last_commented_at: Option<Timestamp>,
    pub has_active_children: bool,
    pub bit_flags: i64,
    pub tag_count_meta: i64,
    pub has_large: bool,
    pub has_visible_children: bool,
    pub tag_string_general: String,
    pub tag_string_character: String,
    pub tag_string_copyright: String,
    pub tag_string_artist: String,
    pub tag_string_meta: String,
    pub file_url: String,
    pub large_file_url: Option<String>,
    pub preview_file_url: Option<String>,
}
