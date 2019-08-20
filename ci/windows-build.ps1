invoke-restmethod -usebasicparsing 'https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe' -outfile 'rustup-init.exe'
& ./rustup-init.exe -y --default-toolchain stable-x86_64-pc-windows-msvc --no-modify-path
& "$ENV:UserProfile\.cargo\bin\rustup.exe" install stable-x86_64-pc-windows-msvc
remove-item rustup-init.exe
& "$ENV:UserProfile\.cargo\bin\cargo.exe" +stable-x86_64-pc-windows-msvc build --release
.\target\release\crom.exe update-version --pre-release release
& "$ENV:UserProfile\.cargo\bin\cargo.exe" +stable-x86_64-pc-windows-msvc build --release
& "$ENV:UserProfile\.cargo\bin\cargo.exe" +stable-x86_64-pc-windows-msvc test --release