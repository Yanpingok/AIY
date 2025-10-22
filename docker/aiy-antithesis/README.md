# Aiy Network Docker Compose

This was tested using MacOS 14.3.1, Docker Compose: v2.13.0.

This compose brings up 3 validators, 1 fullnode, and 1 stress (load gen) client

Steps for running:

1. build local stress image 

```
cd docker/stress
docker build -t stress:testing --build-arg AIY_TOOLS_IMAGE_TAG=mainnet-v1.19.1 .
```

2. run compose

```
(optional) `rm -r /tmp/aiy`
docker compose up
```


**additional info**
The version of `aiy` that generates  the genesis outputs must be on the same protocol version as the fullnode/validators (for example, `mysten/aiy-node:mainnet-v1.19.1`)
Here's an example of how to build a `aiy` binary that creates a genesis which is compatible with the release: `v1.19.1`
```
git checkout releases/aiy-v1.19.0-release
cargo build --bin aiy
```
you can also use `aiy-network/Dockerfile` for building genesis
