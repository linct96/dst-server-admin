#!/bin/bash

OS=$(awk -F = '/^NAME/{print $2}' /etc/os-release | sed 's/"//g' | sed 's/ //g' | sed 's/Linux//g' | sed 's/linux//g')

echo $OS

if [ "$OS" == "Ubuntu" ]; then
  sudo apt-get -y update
  sudo apt-get -y wget
  sudo apt-get -y install screen
  sudo apt-get -y install htop

  # 加载 32bit 库
  sudo apt-get -y install lib32gcc1
  sudo apt-get -y install lib32stdc++6
  sudo apt-get -y install libcurl4-gnutls-dev:i386

  # 加载 64bit库
  sudo apt-get -y install lib64gcc1
  sudo apt-get -y install lib64stdc++6
  sudo apt-get -y install libcurl4-gnutls-dev

elif [ "$OS" == "Centos" ]; then
  sudo yum -y update
  sudo yum -y install tar wget screen
  
  # 加载 32bit 库
  sudo yum -y install glibc.i686 libstdc++.i686 libcurl.i686
  
  # 加载 64bit 库
  sudo yum -y install glibc libstdc++ libcurl
  
  if [ -f "/usr/lib/libcurl.so.4" ]; then
    ln -sf /usr/lib/libcurl.so.4 /usr/lib/libcurl-gnutls.so.4	
  fi
  
  if [ -f "/usr/lib64/libcurl.so.4" ]; then
    ln -sf /usr/lib64/libcurl.so.4 /usr/lib64/libcurl-gnutls.so.4
  fi
  
elif [ "$OS" == "Arch" ]; then
  sudo pacman -Syyy
  sudo pacman -S --noconfirm wget screen
  sudo pacman -S --noconfirm lib32-gcc-libs libcurl-gnutls
else
  echo "该系统未被本脚本支持！"
fi