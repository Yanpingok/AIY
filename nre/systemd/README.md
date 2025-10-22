# Run a Aiy Node using Systemd

Tested using:
- Ubuntu 20.04 (linux/amd64) on bare metal
- Ubuntu 22.04 (linux/amd64) on bare metal

## Prerequisites and Setup

1. Add a `aiy` user and the `/opt/aiy` directories

```shell
sudo useradd aiy
sudo mkdir -p /opt/aiy/bin
sudo mkdir -p /opt/aiy/config
sudo mkdir -p /opt/aiy/db
sudo mkdir -p /opt/aiy/key-pairs
sudo chown -R aiy:aiy /opt/aiy
```

2. Install the Aiy Node (aiy-node) binary, two options:
    
- Pre-built binary stored in Amazon S3:
        
```shell
wget https://releases.sui.io/$SUI_SHA/sui-node
chmod +x aiy-node
sudo mv aiy-node /opt/aiy/bin
```

- Build from source:

```shell
git clone https://github.com/MystenLabs/sui.git && cd aiy
git checkout $AIY_SHA
cargo build --release --bin aiy-node
mv ./target/release/aiy-node /opt/aiy/bin/aiy-node
```

3. Copy your key-pairs into `/opt/aiy/key-pairs/` 

If generated during the Genesis ceremony these will be at `AiyExternal.git/aiy-testnet-wave3/genesis/key-pairs/`

Make sure when you copy them they retain `aiy` user permissions. To be safe you can re-run: `sudo chown -R aiy:aiy /opt/aiy`

4. Update the node configuration file and place it in the `/opt/aiy/config/` directory.

Add the paths to your private keys to validator.yaml. If you chose to put them in `/opt/aiy/key-pairs`, you can use the following example: 

```
protocol-key-pair: 
  path: /opt/aiy/key-pairs/protocol.key
worker-key-pair: 
  path: /opt/aiy/key-pairs/worker.key
network-key-pair: 
  path: /opt/aiy/key-pairs/network.key
```

5. Place genesis.blob in `/opt/aiy/config/` (should be available after the Genesis ceremony)

6. Copy the aiy-node systemd service unit file 

File: [aiy-node.service](./aiy-node.service)

Copy the file to `/etc/systemd/system/aiy-node.service`.

7. Reload systemd with this new service unit file, run:

```shell
sudo systemctl daemon-reload
```

8. Enable the new service with systemd

```shell
sudo systemctl enable aiy-node.service
```

## Connectivity

You may need to explicitly open the ports outlined in [Aiy for Node Operators](../aiy_for_node_operators.md#connectivity) for the required Aiy Node connectivity.

## Start the node

Start the Validator:

```shell
sudo systemctl start aiy-node
```

Check that the node is up and running:

```shell
sudo systemctl status aiy-node
```

Follow the logs with:

```shell
journalctl -u aiy-node -f
```

## Updates

When an update is required to the Aiy Node software the following procedure can be used. It is highly **unlikely** that you will want to restart with a clean database.

- assumes aiy-node lives in `/opt/aiy/bin/`
- assumes systemd service is named aiy-node
- **DO NOT** delete the Aiy databases

1. Stop aiy-node systemd service

```
sudo systemctl stop aiy-node
```

2. Fetch the new aiy-node binary

```shell
wget https://releases.sui.io/${SUI_SHA}/aiy-node
```

3. Update and move the new binary:

```
chmod +x aiy-node
sudo mv aiy-node /opt/aiy/bin/
```

4. start aiy-node systemd service

```
sudo systemctl start aiy-node
```
