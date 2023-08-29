/// Booster. The file where everything kicks off in the engine loop.

pub mod booster {
    use crate::errors::BoosterError;
    use crate::config::Config;

    /// Call the asynchronous launch function.
    pub fn initiate(config: Config) -> Result<(), BoosterError> {
        pollster::block_on(self::launch(config))?;

        Ok(())
    }

    /// Create the event loop and start up.
    pub async fn launch(config: Config) -> Result<(), BoosterError> {
        todo!("Launch!")
    }
}
