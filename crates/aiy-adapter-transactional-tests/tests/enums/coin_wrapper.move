// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//# init --addresses Test=0x0 --accounts A

//# publish --upgradeable --sender A
module Test::f {
    use aiy::coin::Coin;
    use aiy::aiy::AIY;

    public struct Other { }

    public enum CoinWrapper has store {
        Aiy(Coin<AIY>),
        Other(Coin<Other>),
    }

    public struct CoinObject has key, store {
        id: UID,
        coin: CoinWrapper,
    }

    public fun split_off(coin: &mut CoinObject, amount: u64, ctx: &mut TxContext): CoinObject {
        match (&mut coin.coin) {
            CoinWrapper::Aiy(c) => {
                let new_coin = CoinObject {
                    id: object::new(ctx),
                    coin: CoinWrapper::Aiy(c.split(amount, ctx)),
                };
                new_coin
            },
            CoinWrapper::Other(c) => {
                let new_coin = CoinObject {
                    id: object::new(ctx),
                    coin: CoinWrapper::Other(c.split(amount, ctx)),
                };
                new_coin
            },
        }
    }

    public fun create_aiy(coin: &mut Coin<AIY>, amount: u64, ctx: &mut TxContext): CoinObject {
        CoinObject {
            id: object::new(ctx),
            coin: CoinWrapper::Aiy(coin.split(amount, ctx)),
        }
    }
}

//# programmable --sender A --inputs 10 @A
//> 0: Test::f::create_aiy(Gas, Input(0));
//> 1: TransferObjects([Result(0)], Input(1))
