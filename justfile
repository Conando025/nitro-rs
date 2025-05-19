# set working-directory := '~/Programming/nds'

run file="out": (rom file)
    $EMULATOR {{file}}.nds

rom file="out": (release file)
    $NDS_BUILDER --arm7 target/armv4t-none-eabi/release/example --arm9 target/armv5te-none-eabi/release/example -o {{file}}.nds
  
debug_rom file="out": (debug file)
    $NDS_BUILDER --arm7 target/armv4t-none-eabi/debug/example --arm9 target/armv5te-none-eabi/debug/example -o {{file}}.nds

release file="out": arm7 arm9
    @echo release build

debug file="out": (arm7 'debug') (arm9 'debug')
    @echo debug build

[working-directory: 'example']
arm7 debug='':
    cargo build {{if debug == 'debug' {""} else {"--release"} }} --target=armv4t-none-eabi

[working-directory: 'example']
arm9 debug='':
    cargo build {{if debug == 'debug' {""} else {"--release"} }} --target=armv5te-none-eabi
clean:
    cargo clean
