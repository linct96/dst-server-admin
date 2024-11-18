#!/bin/bash


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


install_steamCMD(){
  log "开始安装 steamCMD"
  mkdir -p ${SteamCMDPath}
  cd ${SteamCMDPath}
  curl -sqkL https://steamcdn-a.akamaihd.net/client/installer/steamcmd_osx.tar.gz | tar zxvf -
  success "steamCMD 安装成功"
}

update_dst_server(){
  if [ -f ${SteamCMDPath}/steamcmd.sh ]; then
    log "开始获取最新版本游戏文件"
    cd ${SteamCMDPath}
    chmod +x ./steamcmd.sh
    ./steamcmd.sh +force_install_dir ${HomePath}/dst +login anonymous +app_update 343050 validate +quit
    success "最新版本游戏文件安装成功"
  else
    error "steamCMD 未安装"
    exit 1
  fi
}

main(){
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

log "steamCMD 未安装"
# install_steamCMD
log "steamCMD 已安装"