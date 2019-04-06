use chrono::Utc;
use rusoto_core::region::Region;
use rusoto_logs::{
    CloudWatchLogs, CloudWatchLogsClient, CreateLogStreamRequest, DescribeLogStreamsRequest,
    InputLogEvent, PutLogEventsRequest,
};

pub struct Aws {
    client: CloudWatchLogsClient,
    log_group_name: String,
    log_stream_name: String,
}

impl Aws {
    pub fn new(region: Region, log_group_name: String, log_stream_name: String) -> Self {
        Self {
            client: CloudWatchLogsClient::new(region),
            log_group_name,
            log_stream_name,
        }
    }

    pub fn create_log_stream(&self) {
        self.client
            .create_log_stream(CreateLogStreamRequest {
                log_group_name: self.log_group_name.clone(),
                log_stream_name: self.log_stream_name.clone(),
            })
            .sync()
            .unwrap();
    }

    pub fn put_log(&self, message: String) {
        let input_log_event = InputLogEvent {
            message: message.to_string(),
            timestamp: Utc::now().timestamp_millis(),
        };

        let mut desc_streams_req: DescribeLogStreamsRequest = Default::default();
        desc_streams_req.log_group_name = self.log_group_name.to_string();
        let streams_resp = self.client.describe_log_streams(desc_streams_req).sync();
        let log_streams = streams_resp.unwrap().log_streams.unwrap();
        let stream = &log_streams
            .iter()
            .find(|s| s.log_stream_name == Some(self.log_stream_name.to_string()))
            .unwrap();
        let sequence_token = stream.upload_sequence_token.clone();

        let put_log_events_request = PutLogEventsRequest {
            log_events: vec![input_log_event],
            log_group_name: self.log_group_name.to_string(),
            log_stream_name: self.log_stream_name.to_string(),
            sequence_token,
        };

        self.client
            .put_log_events(put_log_events_request)
            .sync()
            .unwrap();
    }
}
