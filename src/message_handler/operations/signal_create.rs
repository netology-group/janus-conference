use anyhow::Error;
use async_trait::async_trait;
use http::StatusCode;
use svc_error::Error as SvcError;

use crate::switchboard::AgentId;

#[derive(Clone, Debug, Deserialize)]
pub struct Request {
    agent_id: AgentId,
}

#[derive(Serialize)]
struct Response {}

#[async_trait]
impl super::Operation for Request {
    async fn call(&self, request: &super::Request) -> super::OperationResult {
        verb!(
            "Calling signal.create operation";
            {"agent_id": self.agent_id, "handle_id": request.session_id()}
        );

        let internal_error = |err: Error| {
            SvcError::builder()
                .kind("signal_create_error", "Error creating a signal")
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .detail(&err.to_string())
                .build()
        };

        let app = app!().map_err(internal_error)?;
        let agent_id = self.agent_id.to_owned();
        let session_id = request.session_id().to_owned();

        app.switchboard_dispatcher
            .dispatch(move |switchboard| switchboard.associate_agent(session_id, &agent_id))
            .await
            .map_err(internal_error)?
            .map_err(internal_error)?;

        Ok(Response {}.into())
    }

    fn is_handle_jsep(&self) -> bool {
        true
    }
}
