export def vita-run [] {
  lines
  | each {
    from json | get -i executable
  } | each {|elf|
    let elf-parts = ($elf | path parse);
    let velf = ($elf-parts | upsert extension { "velf" })
    let eboot-bin = ($elf-parts | upsert extension { "eboot.bin" })
    let velf = ($velf.parent | path join $"($velf.stem).($velf.extension)")
    let eboot-bin = ($eboot-bin.parent | path join $"($eboot-bin.stem).($eboot-bin.extension)")
  
    "destroy\n" | nc 192.168.1.18 1338
    ~/repos/vita-toolchain/build/src/vita-elf-create $elf $velf
    ~/repos/vita-toolchain/build/src/vita-make-fself $velf $eboot-bin
    sleep 500ms
    curl -T $eboot-bin ftp://192.168.1.18:1337/ux0:/app/ABCD99999/eboot.bin
    "launch ABCD99999\n" | nc 192.168.1.18 1338
  }
}

export def bindg [header: string, sysroot: path = "~/.vitasdk/arm-vita-eabi"] {
  let include = ($sysroot | path join "include")
  let header = ($include | path join $header)
  (bindgen --ctypes-prefix "" --no-layout-tests --no-doc-comments --no-prepend-enum-name
    $header
    --allowlist-file $header
    -- --sysroot $sysroot)
}