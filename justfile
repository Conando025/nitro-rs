# set working-directory := '~/Programming/nds'

run file="out": rom
    $EMULATOR {{file}}.nds

rom file="out": release
    $NDS_BUILDER --arm7 target/armv4t-none-eabi/release/example --arm9 target/armv5te-none-eabi/release/example -o {{file}}.nds
  
debug_rom file="out.debug": debug
    $NDS_BUILDER --arm7 target/armv4t-none-eabi/debug/example --arm9 target/armv5te-none-eabi/debug/example -o {{file}}.nds

release: arm7 arm9
    @echo release build

debug: (arm7 'debug') (arm9 'debug')
    @echo debug build

[working-directory: 'example']
arm7 debug='':
    cargo build {{if debug == 'debug' {""} else {"--release"} }} --target=armv4t-none-eabi

[working-directory: 'example']
arm9 debug='':
    cargo build {{if debug == 'debug' {""} else {"--release"} }} --target=armv5te-none-eabi
clean:
    cargo clean
