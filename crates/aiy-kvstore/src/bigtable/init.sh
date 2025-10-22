# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0
INSTANCE_ID=${1:-aiy}
command=(
  cbt
  -instance
  "$INSTANCE_ID"
)
if [[ -n $BIGTABLE_EMULATOR_HOST ]]; then
  command+=(-project emulator)
fi

for table in objects transactions checkpoints checkpoints_by_digest watermark watermark_alt epochs; do
  (
    set -x
    "${command[@]}" createtable $table
    "${command[@]}" createfamily $table aiy
    "${command[@]}" setgcpolicy $table aiy maxversions=1
  )
done
"${command[@]}" setgcpolicy watermark aiy maxage=2d
