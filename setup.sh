sudo -s
yum install -y git gcc openssl-devel kernel-devel-$(uname -r) bc numactl-devel make net-tools vim pciutils iproute pcre-devel zlib-devel elfutils-libelf-devel python3 wget unzip clang
pip3 install meson
export PATH=$PATH:/usr/local/bin
mkdir -p /data/f-stack
git clone https://github.com/F-Stack/f-stack.git /data/f-stack
wget https://github.com/ninja-build/ninja/releases/download/v1.10.2/ninja-linux.zip
sudo unzip ninja-linux.zip -d /usr/local/bin/
cd /data/f-stack/dpdk
meson -Denable_kmods=true build
ninja -C build
ninja -C build install
cd /data/
wget https://pkg-config.freedesktop.org/releases/pkg-config-0.29.2.tar.gz
tar xzvf pkg-config-0.29.2.tar.gz
cd pkg-config-0.29.2
./configure --with-internal-glib
make
make install
mv /usr/bin/pkg-config /usr/bin/pkg-config.bak
ln -s /usr/local/bin/pkg-config /usr/bin/pkg-config

export FF_PATH=/data/f-stack
export PKG_CONFIG_PATH=/usr/lib64/pkgconfig:/usr/local/lib64/pkgconfig:/usr/lib/pkgconfig
cd /data/f-stack/lib
make
make install