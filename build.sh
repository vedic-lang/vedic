#/usr/bin/bash

# variables
PKG_NAME=`node -p "require('./package.json').name"` 
PKG_VERSION=`node -p "require('./package.json').version"`  
PKG_DESC=`node -p "require('./package.json').description"`  
NAME=${PKG_NAME}_${PKG_VERSION}

# creating dir
rm -rf vedic release
mkdir vedic release

# install pkg
if [ "$(pkg -v)" = '' ]; 
then 
    echo 'Error: pkg is not installed.'
    npm i -g pkg
fi

# build linux binary 
echo "Building Linux binary"
pkg -t node14-linux ./main.js --compress GZip --output ./vedic/vedic
tar -czvf ./release/${NAME}_linux.tar.gz ./vedic/vedic

# build deb packages
echo "Building deb packages"
mkdir -p ./vedic/${NAME}/DEBIAN
mkdir -p ./vedic/${NAME}/usr/local/bin/
cp ./vedic/vedic ./vedic/${NAME}/usr/local/bin/vedic
cat << __EOF__ > ./vedic/$NAME/DEBIAN/control
Package: $PKG_NAME
Version: $PKG_VERSION
Section: custom
Priority: optional
Architecture: all
Essential: no
Maintainer: https://github.com/vedic-lang
Description: $PKG_DESC
__EOF__
dpkg-deb --build ./vedic/${NAME}
mv ./vedic/${NAME}.deb ./release/${NAME}.deb


# build windows binary
echo "Building windows binary"
pkg -t node14-win ./main.js --compress GZip --output ./vedic/vedic.exe
zip -r -Z bzip2 ./release/${NAME}_windows.zip . -i ./vedic/vedic.exe

# build macos binary
echo "Building macos binary"
pkg -t node14-mac ./main.js --compress GZip --output ./vedic/vedic
zip -r -Z bzip2 ./release/${NAME}-macos.zip . -i ./vedic/vedic

# clear old temp
rm -rf vedic