#!/bin/bash

# 检查参数数量是否正确
if [ "$#" -ne 1 ]; then
  echo "Usage: $0 <steamCMD_path>"
  exit 1
fi

# 获取参数
steam_cmd_path=$1
dst_server_path=$2
OS=""
steam_exec="steamcmd.sh"

# 根目录
HomePath="${HOME}"
# steamCMD根目录
SteamCMDPath="${HomePath}/steamCMD"
# 饥荒服务端根目录
DSTServerPath="${HomePath}/Library/Application Support/Steam/steamapps/common/Don't Starve Together Dedicated Server"

error() {
  echo -e "\033[0;31m【错误】\033[0m$1"
}

success() {
  echo -e "\033[0;32m【成功】\033[0m$1"
}

warning() {
  echo -e "\033[0;33m【警告】\033[0m$1"
}

log() {
  echo -e "\033[0;34m【信息】\033[0m$1"
}

set -e
handle_error() {
  local exit_code=$?
  local command=$BASH_COMMAND
  error "Error: Command '$command' failed with exit code $exit_code" >&2
  exit $exit_code
}
trap 'handle_error' ERR

get_OS() {
  if [ "$(uname -s)" = "Darwin" ]; then
    OS="macos"
  elif [ -f /etc/redhat-release ]; then
    OS="centos"
  elif lsb_release -a | grep -q -E -i "centos|red hat|redhat"; then
    OS="centos"
  elif lsb_release -a | grep -q -E -i "debian"; then
    OS="debian"
  elif lsb_release -a | grep -q -E -i "ubuntu"; then
    OS="ubuntu"
  else
    OS=""
  fi
}

install_lib() {
  log "开始安装依赖库"
  if [ "$OS" == "ubuntu" ]; then
    sudo dpkg --add-architecture i386
    sudo apt-get -y update
    sudo apt-get -y dist-upgrade
    sudo apt-get -y install wget
    sudo apt-get -y install screen
    sudo apt-get -y install htop
    sudo apt-get -y install libstdc++6
    sudo apt-get -y install libstdc++6:i386
    sudo apt-get -y install glibc || true

    # 加载 32bit 库
    sudo apt-get -y install lib32stdc++6
    sudo apt-get -y install libcurl4-gnutls-dev:i386
    sudo apt-get -y install lib32gcc-s1
    sudo apt-get -y install lib32gcc1 || true

    # 加载 64bit库
    sudo apt-get -y install lib64gcc-s1:i386 || true
    sudo apt-get -y install lib64gcc1 || true
    sudo apt-get -y install lib64stdc++6 || true
    sudo apt-get -y install libcurl4-gnutls-dev || true

  elif [ "$OS" == "centos" ]; then
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
  log "依赖库安装完成"
}

install_steamCMD() {
  log "开始安装 steamCMD"
  mkdir -p ${steam_cmd_path}
  cd ${steam_cmd_path}
  donwload_steamCMD_url="https://media.st.dl.bscstorage.net/client/installer/steamcmd_linux.tar.gz"

  if [ "${OS}" = "macos" ]; then
    donwload_steamCMD_url="https://steamcdn-a.akamaihd.net/client/installer/steamcmd_osx.tar.gz"
  fi

  curl -sqkL ${donwload_steamCMD_url} | tar zxf - -C ${steam_cmd_path}
  success "steamCMD 安装成功"
}

update_dst_server() {
  log "开始获取最新版本游戏文件"
  cd ${steam_cmd_path}
  chmod +x ./steamcmd.sh
  ./steamcmd.sh +login anonymous +app_update 343050 validate +quit
  success "最新版本游戏文件安装成功"
}

main() {
  get_OS
  log "当前系统：${OS}"

  if [ "${OS}" != "macos" ]; then
    install_lib
  fi

  if [ "${OS}" = "macos" ]; then
    steam_exec="steamcmd.sh"
  fi

  if [ -f "${steam_cmd_path}/${steam_exec}" ]; then
    log "steamCMD 已安装"
  else
    log "steamCMD 未安装"
    install_steamCMD
  fi

  update_dst_server
}

main