# Maintainer: Jheyson Saavedra <jheysonsaav@gmail.com>

pkgname=catsh
pkgver=0.1.0
pkgdesc="A cross-platform shell."
pkgrel=1
arch=("x86_64")
url="https://github.com/jheysonsaav/catsh"
license=("MIT")
depends=("git")
makedepends=("rust" "git")
provides=("catsh")
source=("${pkgname}-${pkgver}.zip::${url}/archive/v${pkgver}.zip")
sha512sums=('SKIP')

check() {
  cd "${srcdir}/${pkgname}-${pkgver}"
  cargo test --release
  cargo check --release
}

build() {
  cd "${srcdir}/${pkgname}-${pkgver}"
  cargo build --release
}

package() {
  install -Dm644 "${srcdir}/${pkgname}-${pkgver}/LICENSE" "${pkgdir}/usr/share/licenses/catsh/LICENSE"
  install -Dm755 "${srcdir}/${pkgname}-${pkgver}/target/release/catsh" "${pkgdir}/usr/bin/catsh"
  install -Dm755 "${srcdir}/${pkgname}-${pkgver}/assets/linux/catsh.desktop" "${pkgdir}/usr/share/applications/catsh.desktop"
}
