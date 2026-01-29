#!/bin/env sh
echo "this script will install smu and quark"
echo "from:"
echo "\thttps://github.com/pixel2175/smu/\n\n"

echo "press to continue..."
read dummy

mkdir tools 
cd tools 

git clone --depth 1 https://github.com/pixel2175/smu  Smu
cd Smu && make
cp smu .. && cd ..

echo
echo "you're good to go"
