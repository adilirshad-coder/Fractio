use crate::ports::ComplianceProvider;
use async_trait::async_trait;

#[derive(Default)]
pub struct MockComplianceProvider;

#[async_trait]
impl ComplianceProvider for MockComplianceProvider {
    async fn start_check(&self, _subject_ref: &str, _kind: &str) -> anyhow::Result<(String, String)> {
        Ok((format!("mock-{}", uuid::Uuid::new_v4()), "approved".to_owned()))
    }

    async fn status(&self, _external_reference: &str) -> anyhow::Result<String> {
        Ok("approved".to_owned())
    }
}
