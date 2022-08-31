---
id: forge cli
title: Forge CLI
custom_edit_url: https://github.com/aptos-labs/aptos-core/edit/main/testsuite/forge-cli/README.md
---

# Forge CLI

This crate contains the Forge command line interface (CLI) tool. This enables users to
run local and remote forge swarms (i.e., networks of validator nodes). For example, to
deploy a local validator swarm, run:

```
cargo run -p forge-cli -- --suite "run_forever" --num-validators 4 test local-swarm
```

This will start a local swarm of 4 validators, each running in their own process. The
swarm will run forever, unless manually killed. The output will display the locations
of the swarm files (e.g., the genesis files, logs, node configurations, etc.) and the
commands that were run to start each node. The process id (PID) of each node and
server addresses are also displayed when it starts. For example:

```
...
2022-08-31T22:22:22.753238Z [main] INFO testsuite/forge/src/backend/local/node.rs:129 Started node "3" (PID: 22103) with command: ".../aptos-core/target/debug/aptos-node" "-f" ".../.tmpPKwNFt/3/node.yaml"
2022-08-31T22:22:22.753300Z [main] INFO testsuite/forge/src/backend/local/node.rs:137 Node "3" REST API is listening at: 127.0.0.1:52972/v1
2022-08-31T22:22:22.753314Z [main] INFO testsuite/forge/src/backend/local/node.rs:142 Node "3" Inspection service is listening at 127.0.0.1:52974
...
```

Using the information from the above output, you could stop a single node and restart
it, e.g., stop and restart node `3`:

```
kill -9 22103
cargo run -p aptos-node -- -f ".../.tmpPKwNFt/3/node.yaml"
```

In addition, the

To also run fullnodes inside the swarm, use the `--num-validator-fullnodes` flag, e.g.,:
```
cargo run -p forge-cli -- --suite "run_forever" --num-validators 3 --num-validator-fullnodes 1 test local-swarm
```

To see all tool usage options, run:
```
cargo run -p forge-cli --help
```



// TODO: add more detailed usage information. There's a lot more that users can do!
