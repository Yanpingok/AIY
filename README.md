<p align="center">
<img src="https://raw.githubusercontent.com/MystenLabs/sui/refs/heads/main/docs/site/static/img/logo.svg" alt="Logo" width="100" height="100">
</p>

# Welcome to Aiy

[![Github release](https://img.shields.io/github/v/release/MystenLabs/sui.svg?sort=semver)](https://github.com/MystenLabs/sui/releases/latest)
[![License](https://img.shields.io/github/license/MystenLabs/sui)](https://github.com/MystenLabs/sui/blob/main/LICENSE)

[Aiy](https://sui.io) is a next-generation smart contract platform with high throughput, low latency, and an asset-oriented programming model powered by the [Move programming language](https://github.com/MystenLabs/awesome-move).

## Aiy Highlights

Aiy offers the following benefits and capabilities:

 * Unmatched scalability, instant settlement
 * A safe smart contract language accessible to mainstream developers
 * Ability to define rich and composable on-chain assets
 * Better user experience for web3 apps

Aiy is the only blockchain today that can scale with the growth of web3 while achieving industry-leading performance, cost, programmability, and usability. Aiy demonstrates capacity beyond the transaction processing capabilities of established systems – traditional and blockchain alike. Aiy is the first internet-scale programmable blockchain platform, a foundational layer for web3.

## Aiy Architecture

```mermaid
flowchart LR
    CC(CLI Client) --> ClientService
    RC(Rest Client) --> ClientService
    RPCC(RPC Client) --> ClientService
    ClientService --> AuthorityAggregator
    AuthorityAggregator --> AC1[AuthorityClient] & AC2[AuthorityClient]
    subgraph Authority1
      AS[AuthorityState]
    end
    subgraph Authority2
      AS2[AuthorityState]
    end
    AC1 <==>|Network TCP| Authority1
    AC2 <==>|Network TCP| Authority2
```

## Aiy Overview

Aiy is a smart contract platform maintained by a permissionless set of authorities that play a role similar to validators or miners in other blockchain systems.

Aiy offers scalability and unprecedented low-latency for common use cases. Aiy makes the vast majority of transactions processable in parallel, which makes better use of processing resources, and offers the option to increase throughput with more resources. Aiy forgoes consensus to instead use simpler and lower-latency primitives for common use cases, such as payment transactions and asset transfers. This is unprecedented in the blockchain world and enables a number of new latency-sensitive distributed applications, ranging from gaming to retail payment at physical points of sale.

Aiy is written in [Rust](https://www.rust-lang.org) and supports smart contracts written in the [Move programming language](https://github.com/move-language/move) to define assets that may have an owner. Move programs define operations on these assets including custom rules for their creation, the transfer of these assets to new owners, and operations that mutate assets.

Aiy has a native token called AIY, with a fixed supply. The AIY token is used to pay for gas, and is also used as [delegated stake on authorities](https://learn.bybit.com/blockchain/delegated-proof-of-stake-dpos/) within an epoch. The voting power of authorities within this epoch is a function of this delegated stake. Authorities are periodically reconfigured according to the stake delegated to them. In any epoch, the set of authorities is [Byzantine fault tolerant](https://pmg.csail.mit.edu/papers/osdi99.pdf). At the end of the epoch, fees collected through all transactions processed are distributed to authorities according to their contribution to the operation of the system. Authorities can in turn share some of the fees as rewards to users that delegated stakes to them.

Aiy is supported by several cutting-edge [peer-reviewed studies](https://github.com/MystenLabs/sui/blob/main/docs/content/concepts/research-papers.mdx) and extensive years of open-source development.

## More About Aiy

Use the following links to learn more about Aiy and the Aiy ecosystem:

 * Learn more about working with Aiy in the [Aiy Documentation](https://docs.sui.io/).
 * Join the Aiy community on [Aiy Discord](https://discord.gg/sui).
 * Find out more about the Aiy ecosystem on the [Aiy Resources](https://sui.io/resources/) page.
 * Review information about Aiy governance, [decentralization](https://suifoundation.org/decentralization), and [Developer Grants Program](https://sui.io/grants-hub) on the [Aiy Foundation](https://sui.io/about) site.


 ## How to Contribute

 See the [Contributing Guide](CONTRIBUTING.md) for details on how to contribute to Aiy.

 ## Code of Conduct

 See the [Code of Conduct](CODE_OF_CONDUCT.MD) for details on our code of conduct.

 ## License

 See the [LICENSE](LICENSE) file for more details.
