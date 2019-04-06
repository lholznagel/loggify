use crate::aws::Aws;
use chrono::Utc;
use rusoto_core::region::Region;

/// Creates a new log stream everytime it is initialized
/// If no log stream name is given, the current timestamp in milliseconds is used
/// else the following template is used: `{log_Stream_name}_{current_timestamp_in_millis}`
pub struct AwsBuilder {
    pub region: Region,
    pub log_group_name: String,
    pub log_stream_name: String,
}

impl AwsBuilder {
    pub fn set_region(mut self, region: Region) -> Self {
        self.region = region;
        self
    }

    pub fn set_log_group_name(mut self, log_group_name: String) -> Self {
        self.log_group_name = log_group_name;
        self
    }

    pub fn set_log_stream_name(mut self, log_stream_name: String) -> Self {
        self.log_stream_name = format!("{}_{}", log_stream_name, Utc::now().timestamp_millis());
        self
    }

    pub fn build(self) -> Aws {
        let aws = Aws::new(self.region, self.log_group_name, self.log_stream_name);
        aws.create_log_stream();
        aws
    }
}

impl Default for AwsBuilder {
    fn default() -> Self {
        Self {
            region: Region::EuCentral1,
            //region: Region::default(),
            log_group_name: String::from("loggify"),
            log_stream_name: Utc::now().timestamp_millis().to_string(),
        }
    }
}
