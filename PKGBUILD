pkgname=catsh
pkgver=0.1.0_beta.0
pkgrel=1
arch=('x86_64')
url='https://github.com/jheysonsaav/catsh'
license=('MIT')
depends=('git')
makedepends=('rust')
provides=('catsh')
source=('${pkgname}-${pkgver}.zip::${url}/archive/v${pkgver}.zip')

build() {
  cd "${srcdir}/${pkgname}-${pkgver}"
  export CARGO_HOME="${srcdir}/${pkgname}/CARGO"
  export CFLAGS="-fcommon -fPIE"
  export RUSTUP_TOOLCHAIN=stable
  cargo build --release
}

package() {
  install -Dm644 "${srcdir}/${pkgname}-${pkgver}/LICENSE" "${pkgdir}/usr/share/licenses/catsh/LICENSE"
  install -Dm755 "${srcdir}/${pkgname}-${pkgver}/target/release/catsh" "${pkgdir}/usr/bin/catsh"
}
