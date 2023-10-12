# Startup Systems

Startup systems are used in place of raw calls to the Llanite API as there is a big issue. The issue is that calls cannot be made to the API before the launch function is called due to limitations of State not existing yet.

Startup systems will be called just after the state is created to all the API calls to be called after. Hopefully this should cause minimal disruption to the user. It may be possible that a macro could be used to automatically create a startup system with whatever code is put inside.

A startup diagram is put below, it may be a little confusion to understand though. There is a line through the arrow to run startup systems from create state to show that creating the state should be done first.

![image](https://github.com/Llanite/Llanite/assets/143108602/a719844c-0c4c-4dfa-aac3-8ec0be5f9014)

## Current implementation

```rust
use llanite::prelude::*;

fn main() {
    let mut controller = Controller::default();
    let mut llanite = Llanite::default();

    controller.add_stage(|state| {
        state
            .pipeline_composer
            .new_pipeline("./shaders/custom.wgsl".into())
            .unwrap();
    });

    llanite.start(Config::default(), controller);
}
```

The `Controller` type holds startup systems, they are referred to as stages. You can add a stage as a closure for example and simply call functions that are on state from the new stage.
