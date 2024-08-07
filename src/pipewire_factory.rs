use crate::pipewire_loop::Factories;

#[allow(dead_code)]
pub struct PipewireFactory {
    factories: Factories,
}

impl PipewireFactory {
    pub async fn wait_and_new(
        factories_receiver: tokio::sync::oneshot::Receiver<Factories>,
    ) -> Self {
        let factories = factories_receiver.await.unwrap();
        PipewireFactory { factories }
    }

    pub async fn run(&self) {}
}
