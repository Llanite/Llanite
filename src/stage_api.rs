#[macro_export]
macro_rules! new_pipeline {
    ($state:ident, $path:expr) => {
        $state.pipeline_composer.new_pipeline(Some($path)).unwrap()
    };
}
