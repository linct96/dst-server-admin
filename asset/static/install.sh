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

get_OS() {
  if [ "$(uname -s)" = "Darwin" ]; then
    OS="macos"
  elif [ -f /etc/redhat-release ]; then
    OS="centos"
  elif cat /proc/version | grep -q -E -i "centos|red hat|redhat"; then
    OS="centos"
  elif cat /proc/version | grep -q -E -i "debian"; then
    OS="debian"
  elif cat /proc/version | grep -q -E -i "ubuntu"; then
    OS="ubuntu"
  else
    OS=""
  fi
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
  chmod +x ${steam_exec}
  ./${steam_exec} +login anonymous +app_update 343050 validate +quit
  success "最新版本游戏文件安装成功"
}

main() {
  get_OS
  log "当前系统：${OS}"

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

