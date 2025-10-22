# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

aiy client --client.config config.yaml switch --env base

aiy client --client.config config.yaml envs
aiy client --client.config config.yaml --client.env one envs
aiy client --client.config config.yaml --client.env two envs

aiy client --client.config config.yaml active-env
aiy client --client.config config.yaml --client.env one active-env
aiy client --client.config config.yaml --client.env two active-env

# Unknown name -- Should give you None and nothing active
aiy client --client.config config.yaml --client.env not_an_env envs
aiy client --client.config config.yaml --client.env not_an_env active-env
