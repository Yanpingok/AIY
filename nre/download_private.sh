#!/bin/bash
# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

if ! cosign version &> /dev/null
then
    echo "cosign in not installed, Please install cosign for binary verification."
    echo "https://docs.sigstore.dev/cosign/installation"
    exit
fi

commit_sha=$1
pub_key=https://sui-private.s3.us-west-2.amazonaws.com/sui_security_release.pem
url=https://sui-releases.s3-accelerate.amazonaws.com/$commit_sha

echo "[+] Downloading aiy binaries for $commit_sha ..."
curl $url/aiy -o aiy
curl $url/aiy-indexer -o aiy-indexer
curl $url/aiy-node -o aiy-node
curl $url/aiy-tool -o aiy-tool

echo "[+] Verifying aiy binaries for $commit_sha ..."
cosign verify-blob --insecure-ignore-tlog --key $pub_key --signature $url/aiy.sig aiy
cosign verify-blob --insecure-ignore-tlog --key $pub_key --signature $url/aiy-indexer.sig aiy-indexer
cosign verify-blob --insecure-ignore-tlog --key $pub_key --signature $url/aiy-node.sig aiy-node
cosign verify-blob --insecure-ignore-tlog --key $pub_key --signature $url/aiy-tool.sig aiy-tool
