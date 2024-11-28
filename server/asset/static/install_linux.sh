#!/bin/bash


# 系统
OS=$(awk -F = '/^NAME/{print $2}' /etc/os-release | sed 's/"//g' | sed 's/ //g' | sed 's/Linux//g' | sed 's/linux//g')
# 根目录
HomePath="${HOME}"
# steamCMD根目录
SteamCMDPath="${HomePath}/steamCMD"
# 饥荒服务端根目录
DSTServerPath="${HomePath}/Steam/steamapps/common/Don't Starve Together Dedicated Server"

error(){
  echo -e "\033[0;31m【错误】\033[0m$1"
}

success(){
  echo -e "\033[0;32m【成功】\033[0m$1"
}

warning(){
  echo -e "\033[0;33m【警告】\033[0m$1"
}

log(){
  echo -e "\033[0;34m【信息】\033[0m$1"
}

txt_bold(){
  echo -e "\033[1m$1\033[0m"
}

txt_green(){
  echo -e "\e[32m$1\e[0m\c"
}

txt_orange(){
  echo -e "\e[33m$1\e[0m\c"
}

txt_red(){
  echo -e "\e[31m$1\e[0m\c"
}


install_lib() {
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
  fi
}

install_steamCMD(){
  log "开始安装 steamCMD"
  mkdir -p ${SteamCMDPath}
  cd ${SteamCMDPath}
  curl -sqkL https://media.st.dl.bscstorage.net/client/installer/steamcmd_linux.tar.gz | tar zxf - -C ${SteamCMDPath}
  success "steamCMD 安装成功"
}

update_dst_server(){
  if [ -f ${SteamCMDPath}/steamcmd.sh ]; then
    log "开始获取最新版本游戏文件"
    cd ${SteamCMDPath}
    chmod +x ./steamcmd.sh
    ./steamcmd.sh +login anonymous +app_update 343050 validate +quit
    success "最新版本游戏文件安装成功"
  else
    error "steamCMD 未安装"
    exit 1
  fi
}

main(){
  log "开始确认环境"
  if [ ${OS} != "Ubuntu" -a ${OS} != "Centos" -a ${OS} != "Arch" ]; then
    error "很遗憾！本脚本暂时只支持Debian7+和Ubuntu12+和CentOS7+的系统！" && exit 1
  fi

  install_lib

  if [ ! -f "${SteamCMDPath}/steamcmd.sh" ]; then
    log "steamCMD 未安装"
    install_steamCMD
  else
    log "steamCMD 已安装"
  fi

  if [ ! -f "${DSTServerPath}/version.txt" ]; then
    log "游戏文件未安装"
    update_dst_server
  else
    log "游戏文件已安装"
  fi
}

# install_lib
main