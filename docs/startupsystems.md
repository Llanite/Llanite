# Startup Systems

Startup systems are used in place of raw calls to the Llanite API as there is a big issue. The issue is that calls cannot be made to the API before the launch function is called due to limitations of State not existing yet.

Startup systems will be called just after the state is created to all the API calls to be called after. Hopefully this should cause minimal disruption to the user. It may be possible that a macro could be used to automatically create a startup system with whatever code is put inside.

A startup diagram is put below, it may be a little confusion to understand though. There is a line through the arrow to run startup systems from create state to show that creating the state should be done first.

![image](https://github.com/Llanite/Llanite/assets/143108602/a719844c-0c4c-4dfa-aac3-8ec0be5f9014)
