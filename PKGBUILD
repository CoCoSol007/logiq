# Maintainer: CoCoSol <solois.corentin@gmail.com>
pkgname='logiq'
pkgver=1
pkgrel=1
pkgdesc="A logic SAT solver"
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
url="https://github.com/CoCoSol007/logiq"
license=('GPL')
makedepends=('cargo')
validpgpkeys=()

prepare() {
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}


build() {
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}


check() {
    export RUSTUP_TOOLCHAIN=stable
    cargo test --frozen --all-features
}


package() {
    install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/$pkgname"
}
