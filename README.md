# Description
This repository contains the implementation of a service host-worker using different languages

```
┌─────────────┐   POSIX Queue (request)    ┌─────────────┐
│             ├───────────────────────────►│             │
│  Service A  │◄───────────────────────────│  Service B  │
│   (Host)    │   POSIX Queue (response)   │  (Worker)   │
│             │                            |             │
│             │       ┌────────────┐       |             │
│             ├───────│   Shared   ├───────│             │
│             │       │   Memory   │       |             │
└─────────────┘       └────────────┘       └─────────────┘
```

# Current languages
- C++
- Python
- Rust

# Features
- IPC communication using POSIX queues

# Scripts
We have multiple scripts to build and test the different services

**Python**
- **runpython.sh** : activate python virtual environment + Start service as host
```bash
#!/bin/bash
. ../venv/bin/activate
python3.11 ../python/main.py --host
```

**C++**
- **buildcpp.sh** : Compile Cpp code
- **runcpp_host.sh** : Start service host
```bash
#!/bin/bash
../cpp/build/host/cppservice
```
- **runcpp_worker.sh** : Start service worker
```bash
#!/bin/bash
../cpp/build/worker/cppservice
```

**Rust**
- **buildrust.sh** : Compile Rust code
- **runrust.sh** : Start service as worker
```bash
#!/bin/bash
../rust/target/debug/rsservice
```

# Parallelcube tutorials:
* [Part 1: POSIX message queue](https://www.parallelcube.com/2024/06/01/services-ipc-posix-queues/)
* [Part 2: POSIX shared memory](https://www.parallelcube.com/2024/08/24/service-design-shared/)

## More UE4 Tutorials and gamedev
https://www.parallelcube.com/tutorials/

### Support
Patreon: https://www.patreon.com/BYC

Ko-fi: https://ko-fi.com/cristianbarrio

Twitter: https://twitter.com/parallelcube

Instagram: https://www.instagram.com/parallelcube

Youtube: https://www.youtube.com/channel/UC4xgSihU3i2mPe_vXSWWUXQ