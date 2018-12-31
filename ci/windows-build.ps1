invoke-restmethod -usebasicparsing 'https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe' -outfile 'rustup-init.exe'
./rustup-init.exe -y --default-toolchain nightly-x86_64-pc-windows-msvc --no-modify-path
& "$env:USERPROFILE/.cargo/bin/rustup.exe" install nightly-x86_64-pc-windows-msvc
remove-item rustup-init.exe
invoke-restmethod -usebasicparsing 'https://github.com/ethankhall/crom/releases/download/v0.1.1/crom-windows.exe' -outfile 'crom.exe'
./crom.exe update-version --no-snapshot
& "$env:USERPROFILE/.cargo/bin/cargo.exe" +nightly-x86_64-pc-windows-msvc build --release
& "$env:USERPROFILE/.cargo/bin/cargo.exe" +nightly-x86_64-pc-windows-msvc test --release