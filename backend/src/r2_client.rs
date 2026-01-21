use aws_sdk_s3::Client;
use aws_sdk_s3::config::BehaviorVersion;

#[derive(Clone)]
pub struct R2Client {
    pub client: Client,
    pub bucket_name: String,
}

impl R2Client {
    pub async fn new(account_id: &str, access_key_id: &str, secret_access_key: &str, bucket_name: &str) -> Self {
        let endpoint_url = format!("https://{}.r2.cloudflarestorage.com", account_id);
        
        let credentials = aws_sdk_s3::config::Credentials::new(
            access_key_id,
            secret_access_key,
            None,
            None,
            "r2",
        );

        let config = aws_sdk_s3::Config::builder()
            .behavior_version(BehaviorVersion::latest())
            .endpoint_url(&endpoint_url)
            .credentials_provider(credentials)
            .region(aws_sdk_s3::config::Region::new("auto"))
            .force_path_style(true)
            .build();

        let client = Client::from_conf(config);

        Self {
            client,
            bucket_name: bucket_name.to_string(),
        }
    }
}
