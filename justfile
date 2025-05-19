nds_builder := "~/Programming/nds/nds_builder/target/release/nds_builder"
linker := "ld"

# set working-directory := '~/Programming/nds'

nds file="out": (release file)
    melonDS {{file}}.nds

debug file="out": (_debug file)
    desmume {{file}}.nds

release file="out": arm7 arm9
    {{nds_builder}} --arm7 target/armv4t-none-eabi/release/example_arm7 --arm9 target/armv5te-none-eabi/release/example_arm9 -o {{file}}.nds

_debug file="out": (arm7 'debug') (arm9 'debug')
    {{nds_builder}} --arm7 target/armv4t-none-eabi/debug/example_arm7 --arm9 target/armv5te-none-eabi/debug/example_arm9 --verbose -o {{file}}.nds

[working-directory: 'example-arm7']
arm7 debug='':
    cargo build {{if debug == 'debug' {""} else {"--release"} }}

[working-directory: 'example-arm9']
arm9 debug='':
    cargo build {{if debug == 'debug' {""} else {"--release"} }}

clean:
    cargo clean
