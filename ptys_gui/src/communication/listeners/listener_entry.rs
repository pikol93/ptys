use tokio_util::sync::CancellationToken;

pub struct ListenerEntry {
    pub id: u32,
    token: CancellationToken,
}

impl ListenerEntry {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            token: CancellationToken::new(),
        }
    }

    pub fn clone_token(&self) -> CancellationToken {
        self.token.clone()
    }

    pub fn cancel(&self) {
        self.token.cancel();
    }
}
