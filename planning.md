# Scripting Model

## Callback based:
Scripts have a tick() function that rust calls at the start of each tick.
  - If tick() overruns into the next tick, rust does not call that future tick function.

## Continuous, async:
Scripts have a main() function that runs. If main ever returns, then it will be re-called by rust at the start of the next tick.
  - Main runs in an async task parallel to bevys update rate.

Operations:



## The mlua approach.
Main function that has limited clock cycles to not overwhelm. 


# Tools:
## Task pools:
https://docs.rs/bevy/latest/bevy/tasks/struct.TaskPool.html

## External Control
https://rhai.rs/book/patterns/singleton.html
https://rhai.rs/book/patterns/control.html

## Big parallelism
https://rhai.rs/book/patterns/parallel.html

## Multithreaded Sync
https://rhai.rs/book/patterns/multi-threading.html

## Script Execution Control!
https://github.com/schungx/rhai/blob/master/examples/pause_and_resume.rs

# Script Registration
-> Have a "script" component that stores the string

Script Initialisation:
  // QUERIES ON script component and no engine component
  creates an EngineRegistration component

Script Registration:
  // QUERIES ON script component WITH<EngineRegistration> for ScriptAPI trait
  for each thing in the scriptapis:
    thing.register(engine) // lets the api register any communication channels
  

-> ScriptChanged system that checks for changes to the script
