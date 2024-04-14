use crate::{IrisClient, ServerResponse};

pub struct PipeBuilder<'a> {
    pub command: String,
    pub client: &'a mut IrisClient
}

impl<'a> PipeBuilder<'a> {
    pub fn set(self: &'a mut Self, id: &str, data: &str) -> Self {
        self.command.push_str(format!("SET {id} {data} ~>").as_str());

        Self {
            command: self.command.clone(),
            client: self.client
        }
    }

    pub fn get(self: &'a mut Self, id: &str) -> Self {
        self.command.push_str(format!(" GET {id}").as_str());

        Self {
            command: self.command.clone(),
            client: self.client
        }
    }

    pub async fn execute(self: &mut Self) -> Result<ServerResponse, String> {
        let server_resp = self.client.raw(self.command.trim()).await?;
        Ok(server_resp)
    }
}
