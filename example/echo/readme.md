# requirements

### install boost
### install cmake


# build

```
mkdir build
cd build
cmake .. -DCMAKE_BUILD_TYPE=debug
make
```

# Deterministic Testing


## Run Model Checking, and output action database
    
1. Open echo.tla

    File -> Open Spec -> Add New Spec -> Browse.. -> choose echo.tla -> Finish

2. Setting the model, New Model ->
    Model Overview page,

    In `What is the behaviour spec?`, Select `Temporal formula`, filling with Spec
    
    In `What is the model?`:

    - SERVER_NODE_ID <- filling with "{A_s1, A_s2}" , set of model values, check Type: A
    - CLIENT_NODE_ID <- filling with "{A_c1, A_c2}" , set of model values, check Type: A
    - MESSAGE <- filling with "{A_m1, A_m2}" , set of model values, check Type: A
    - ENABLE_ACTION <- TRUE
    - OUTPUT_ACTION_PATH <- filling with path "/tmp/echo/state.db"

    Uncheck deadlock

    In `Invariants', filling with "SaveAction"

    TLC Option page, in 'Checking Mode', model checking mode, filling 'vars_view' in View

    Run TLC model

## Generate traces from the action database previously built
    
1. Specify a const mapping json file, map the model value to program type
    
    ```shell
    cat /tmp/echo/map_const.json
   ```
  
   ```json
    {
        "A_s1": 1,
        "A_s2": 2,
        "A_c1": 100,
        "A_c2": 200,
        "A_m1": "message 1",
        "A_m2": "message 2",
        "INVALID_NODE_ID": null
    }
    ```

2. Generate trace data

    ```shell
    sedeve_trace_gen --state-db-path /tmp/echo/state.db --out-trace-db-path /tmp/echo/trace.db --map-const-path /tmp/echo/map_const.json
    ```   
       
   this command would generate the trace database `trace.db`
   

## Launch player and the tested system, and run determinist testing

1. Specify the config of the player
    ```shell
    cat /tmp/echo/player_conf.toml
    ```

    ```toml
    trace_db_path = "/tmp/echo/trace.db"
    
    [player_addr]
    nid = 1
    addr = "127.0.0.1:24001"
    
    [[peer_addr]]
    nid = 2
    addr = "127.0.0.1:24002"
    
    [[peer_addr]]
    nid = 3
    addr = "127.0.0.1:24003"

2. Run the player
   ```shell
   sedeve_player --conf-path /tmp/echo/player_conf.toml
   ```

3. Run the deterministic testing of echo
   ```shell
   python dtm_echo.py
   ```
