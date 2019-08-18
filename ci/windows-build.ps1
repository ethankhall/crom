invoke-restmethod -usebasicparsing 'https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe' -outfile 'rustup-init.exe'
& ./rustup-init.exe -y --default-toolchain nightly-x86_64-pc-windows-msvc --no-modify-path
%USERPROFILE%\.cargo\bin\rustup.exe install nightly-x86_64-pc-windows-msvc
remove-item rustup-init.exe
%USERPROFILE%\.cargo\bin\cargo.exe +nightly-x86_64-pc-windows-msvc build --release
.\target\release\crom.exe update-version --pre-release release
%USERPROFILE%\.cargo\bin\cargo.exe +nightly-x86_64-pc-windows-msvc build --release
%USERPROFILE%\.cargo\bin\cargo.exe +nightly-x86_64-pc-windows-msvc test --release