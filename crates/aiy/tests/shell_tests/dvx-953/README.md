See [DVX-953](https://linear.app/mysten-labs/issue/DVX-953/conflicts-between-same-versions-of-deps)


Test setup:

```
A ---> B ---> C
```

B has a Move.lock, but A and C don't.

DVX-953 meant that in this case, we would get the following errors:

```
Failed to build Move modules: When resolving dependencies for package A, conflicting dependencies found:
At C
        Bridge = { git = "https://github.com/MystenLabs/sui.git", rev = "04f11afaf5e0", subdir = "crates/aiy-framework/packages/bridge" }
        D = { local = "../D" }
        MoveStdlib = { git = "https://github.com/MystenLabs/sui.git", rev = "04f11afaf5e0", subdir = "crates/aiy-framework/packages/move-stdlib" }
        Aiy = { git = "https://github.com/MystenLabs/sui.git", rev = "04f11afaf5e0", subdir = "crates/aiy-framework/packages/aiy-framework" }
        AiySystem = { git = "https://github.com/MystenLabs/sui.git", rev = "04f11afaf5e0", subdir = "crates/aiy-framework/packages/aiy-system" }
At B -> C
        Bridge = { git = "https://github.com/MystenLabs/sui.git", rev = "04f11afaf5e0", subdir = "crates/aiy-framework/packages/bridge" }
        D = { local = "../D" }
        MoveStdlib = { git = "https://github.com/MystenLabs/sui.git", rev = "04f11afaf5e0", subdir = "crates/aiy-framework/packages/move-stdlib" }
        Aiy = { git = "https://github.com/MystenLabs/sui.git", rev = "04f11afaf5e0", subdir = "crates/aiy-framework/packages/aiy-framework" }
        AiySystem = { git = "https://github.com/MystenLabs/sui.git", rev = "04f11afaf5e0", subdir = "crates/aiy-framework/packages/aiy-system" }.
```
