#!/bin/bash


# linux 发行版本
OS=""
# 饥荒游戏版本
DST_Version=""
# 存档个数
DST_SaveCount=""

DST_MOD_NEED_UPDATE_COUNT="0"
DST_MOD_NEED_DOWNLOAD_COUNT="0"

# 根目录
HomePath="${HOME}"
# steamCMD根目录
SteamCMDPath="${HomePath}/steamcmd"
# 饥荒服务端根目录
DSTServerPath="${HomePath}/Steam/steamapps/common/Don't Starve Together Dedicated Server"
DSTServerModSetupPath="$DSTServerPath/mods/dedicated_server_mods_setup.lua"
# 存档根目录
DSTSavesPath="${HomePath}/.klei/DoNotStarveTogether"

UgcDirectoryPath="$DSTServerPath/ugc_mods"
# 全局变量-当前操作的存档
CurOperateSavePath=""
RunningServer=""
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

# 获取文件夹下的文件个数
countDir(){
  local count="$(ls $1 -l |grep '^d'|wc -l)"
  echo $count
}

get_status(){
  get_DST_version
  get_OS
  get_saves_count
  get_running_server
  get_mod_update
}

get_DST_version(){
  local versionFile="${DSTServerPath}/version.txt"
  if [ -f "$versionFile" ]; then
    DST_Version=$(cat "$versionFile")
  else
    DST_Version=""
  fi
}

get_OS(){
  if [ -f /etc/redhat-release ]; then
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

get_saves_count(){
  if [ -d "$DSTSavesPath" ]; then
    DST_SaveCount=$(countDir "$DSTSavesPath")
  else
    DST_SaveCount=0
  fi
}

get_running_server(){
  local runningServer=""
  for str in $(screen -ls);
  do
    if [[ $str =~ ^[0-9]+. ]]; then
      local OLD_IFS=$IFS
      IFS="."
      local arr=($str)
      local pId=${arr[0]}
      local worldName=${arr[1]}
      local saveName=${arr[2]}
      IFS=$OLD_IFS
      if [ -z "$runningServer" ]; then
        runningServer="$saveName"
      else
        local hasRepeat="false"
        for i in $runningServer
        do
          if [[ $i = $saveName ]]; then
            hasRepeat="true"
            break
          fi
        done
        if [ "$hasRepeat" = "false" ]; then
          runningServer="$runningServer $saveName"
        fi
      fi
    fi
  done
  RunningServer=$runningServer
}

get_mod_update(){
  local workshopFile="${DSTServerPath}/appworkshop_322330.acf"
  if [ -f "$workshopFile" ]; then
    DST_MOD_NEED_UPDATE_COUNT=$(awk '/NeedsUpdate/{print $2}' $workshopFile | sed 's/"//g')
    DST_MOD_NEED_DOWNLOAD_COUNT=$(awk '/NeedsDownload/{print $2}' $workshopFile | sed 's/"//g')
  fi
}



check_env(){
  log "开始确认环境"

  if [ -z ${OS} ]; then
    error "很遗憾！本脚本暂时只支持Debian7+和Ubuntu12+和CentOS7+的系统！" && exit 1
  fi

  if [ ! -f ${SteamCMDPath}/steamcmd.sh ]; then
    log "steamCMD 未安装"
    install_dependency
    install_steamCMD
  else
    log "steamCMD 已安装"
  fi

  if [ -z ${DST_Version} ]; then
    log "游戏文件未安装"
    install_game
  else
    log "游戏文件已安装"
  fi

}

install_dependency(){
  if [ "$OS" == "ubuntu" ]; then
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
  else
    error "该系统未被本脚本支持！"
    exit 1
  fi
}

install_steamCMD(){
  log "开始安装 steamCMD"
  mkdir -p ${SteamCMDPath}
  cd ${SteamCMDPath}
  wget --no-check-certificate https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz
  tar -xvzf steamcmd_linux.tar.gz
  rm -f steamcmd_linux.tar.gz
  success "steamCMD 安装成功"
}


install_game(){
  if [ -f ${SteamCMDPath}/steamcmd.sh ]; then
    log "开始获取最新版本游戏文件"
    cd ${SteamCMDPath}
    chmod +x steamcmd.sh
    ./steamcmd.sh +login anonymous +app_update 343050 validate +quit
    
    mkdir -p "$UgcDirectoryPath"

    success "最新版本游戏文件安装成功"
  else
    error "steamCMD 未安装"
    exit 1
  fi
}

menu(){
  while :
  do
  echo "============================================"
  echo " 存档个数：$DST_SaveCount"
  echo " 模组状态：$DST_MOD_NEED_UPDATE_COUNT 个需要更新，$DST_MOD_NEED_DOWNLOAD_COUNT 个需要下载"
  echo " 游戏版本：正式版(Version: $DST_Version)"
  echo " 系统信息：$OS linux 环境"
  if [ ! -z "$RunningServer" ]; then
    echo " 正在运行：$RunningServer"
  fi
  echo "============================================"
  echo " [1]启动服务器 [2]关闭服务器"
  echo " [3]存档管理   [4]更新游戏"
  echo " [5]Mod管理    [6]Mod更新"
  echo " [0]退出"
  echo "============================================="
  echo 
  txt_orange " 请输入功能选项: "
  read feat
  case $feat in
    1)
      clear
      if [ $DST_SaveCount -ne 0 ]; then
        show_saves
      fi
      start_server
      
      # if [ $DST_SaveCount -ne 0 ]; then
      #   show_saves
      #   start_server
      # else
      #   log "当前无存档"
      #   txt_orange " 是否新建存档(默认新建)? (y/n): "
      #   read willCreateSave
      #   if [[ "${willCreateSave}" == [Nn] ]]; then
      #     clear
      #     menu
      #   else
      #     create_save
      #   fi
      # fi
      break;;
    2)
      close_server
      break;;
    3)
      clear
      manage_saves
      break;;
    4)
      clear
      install_game
      break;;
    5)
      clear
      if [ $DST_SaveCount -ne 0 ]; then
        select_save
        manage_mod $CurOperateSavePath
      else
        log "当前无可用存档，请先创建存档"
        menu
      fi
      break;;
    6)
      clear
      update_mod
      break;;
    0)
      exit_shell
      break;;
    *)
      clear
      error "输入错误 请重新输入"
    esac
  done
}


show_saves(){
  echo "存档目录"
  echo "============================================"
  ls -l $DSTSavesPath |awk '/^d/ {print $NF}'
  echo "============================================"
}

get_saves(){
  if [ "$(ls -A $DSTSavesPath)" ]; then
    for file in $DSTSavesPath/*
    do
      if test -d $file; then
        local arr=(${arr[@]} $file)
      fi
    done
    echo ${arr[@]}
  fi
}

start_server(){
  local savePath=$1
  if [ -z "$savePath" ]; then
    txt_orange " 请输入启动的存档: "
    read boot_save
    savePath=$DSTSavesPath/$boot_save
  fi
  if [ -d $savePath ]; then
    if [[ $RunningServer == $boot_save ]]; then
      error "该存档已经在运行"
      start_server
    else
      log "开始启动服务"
      run_server $DSTSavesPath/$boot_save
      local count=0
      while (($count < 10))
      do
        sleep 1
        get_running_server
        if [ ! -z "$RunningServer" ]; then
          break
        fi
        count=$(($count+1))
        if [ $count -eq 20 ]; then
          error "服务器启动失败"
          exit 1
        fi
      done
      success "服务器已启动"
      sleep 1
      clear
      menu
    fi
    
  else
    txt_orange " 存档[$boot_save]不存在,是否新建存档(默认是)? (y/n):"
    read willCreateSave
    if [[ "${willCreateSave}" == [Nn] ]]; then
      start_server
    else
      create_save $boot_save
      start_server $savePath
      # txt_orange "请选择接下来的操作(默认为1)?  1.启动服务器   2.添加mod  :"
      # read input

    fi
    echo ""
  fi
}

run_server(){
  local savePath=$1
  for file in `find $savePath -name "*.sh"`
  do
    local shellName=$(basename $file .sh)
    local saveName=$(basename $savePath)
    chmod u+x $file
    screen -dmS "${shellName//start/}.$saveName" $file
  done
}

close_server(){
  if [ -z "$RunningServer" ]; then
    log "当前无运行的服务器"
  else
    for str in $(screen -ls);
    do
      if [[ $str =~ ^[0-9]+. ]]; then
        local OLD_IFS=$IFS
        IFS="."
        local arr=($str)
        local pId=${arr[0]}
        local worldName=${arr[1]}
        local saveName=${arr[2]}
        # echo "$pId $worldName $saveName"
        IFS=$OLD_IFS
        screen -r $pId -X stuff "c_shutdown()\n"
      fi
    done
    log "正在关闭服务器$RunningServer"
    local count=0
    while (($count < 20))
    do
      sleep 1
      get_running_server
      if [ -z $RunningServer ]; then
        break
      fi
      count=$(($count+1))
      if [ $count -eq 20 ]; then
        error "服务器关闭失败"
        exit 1
      fi
    done
    log "关闭服务器成功"
  fi
  
  sleep 1
  clear
  menu
}

create_save(){
  local saveName=$1

  if [ -z $saveName ]; then
    txt_orange " 请输入新建存档名称(默认名称为default): "
    read inputName
    if [ -z ${inputName} ]; then
      saveName="default"
    else
      saveName=$inputName
    fi
  fi

  if [[ ! $saveName =~ ^[A-Za-z0-9]+$ ]]; then
    error "存档名称不能包含中文、符号和空格，请重新输入"
    create_save
  fi
  
  log 存档名称：${saveName}
  local savePath="${DSTSavesPath}/${saveName}"

  if [ -d $savePath ]; then
    txt_orange " 该存档已存在，\e[31m是否覆盖该存档\e[33m(默认否)? (y/n): "
    read willDeleteSave
    if [[ "${willDeleteSave}" == [Yy] ]]; then
      rm -rf $savePath
      success "成功覆盖存档 ${saveName}"
    else
      create_save
    fi
  fi

  init_cluster $savePath

  txt_orange " 是否添加洞穴(默认是)? (y/n): "
  read willAddCave
    if [[ "${willAddCave}" == [Nn] ]]; then
      :
    else
      add_Cave $savePath
    fi
  success "创建成功"

  if [ -z $1 ]; then
    clear
    menu
  fi
}

init_cluster(){
  local savePath=$1
  local tokenPath="${savePath}/cluster_token.txt"
  local adminConfigPath="${savePath}/adminlist.txt"
  local blockConfigPath="${savePath}/blocklist.txt"
  local whiteConfigPath="${savePath}/whitelist.txt"
  local configPath="${savePath}/cluster.ini"
  local saveName=$(basename $savePath)
  mkdir -p $savePath
  touch $tokenPath
  touch $adminConfigPath
  touch $blockConfigPath
  touch $whiteConfigPath
  touch $configPath
  
  txt_orange " 请输入私有令牌(默认使用脚本提供的令牌): "
  read tokenInput
  if [ -z $tokenInput ]; then
    tokenInput="pds-g^KU_xnhMAWcB^RN/1y/K7lu3VFtczaFFxwdLFUi9ajqjC56kQdSoXGvU="
  fi

  echo $tokenInput >> $tokenPath

  echo -e "[STEAM]" > $configPath
  echo -e "steam_group_id = 0" >> $configPath
  echo -e "steam_group_only = false" >> $configPath
  echo -e "steam_group_admins = false" >> $configPath
  echo -e "" >> $configPath
  
  echo -e "[GAMEPLAY]" >> $configPath
  echo -e "; survival|endless|wilderness" >> $configPath
  echo -e "game_mode = endless" >> $configPath
  echo -e "; 1..64" >> $configPath
  echo -e "max_players = 4" >> $configPath
  echo -e "; true|false" >> $configPath
  echo -e "pause_when_empty = true" >> $configPath
  echo -e "; true|false" >> $configPath
  echo -e "vote_enabled = true" >> $configPath
  echo -e "" >> $configPath
  
  echo -e "[NETWORK]" >> $configPath
  
  echo -e "; cooperative|competitive|social|madness" >> $configPath
  echo -e "cluster_intention = cooperative" >> $configPath
  
  echo -e "cluster_name = ${saveName}" >> $configPath
  echo -e "cluster_description = 一起享受饥荒吧！" >> $configPath
  echo -e "cluster_password = password" >> $configPath
  echo -e "cluster_language = zh" >> $configPath
  
  echo -e "; true|false" >> $configPath
  echo -e "autosaver_enabled = true" >> $configPath
  
  echo -e "; true|false" >> $configPath
  echo -e "enable_vote_kick = true" >> $configPath
  
  echo -e "; 10|15|30|60" >> $configPath
  echo -e "tick_rate = 15" >> $configPath
  
  echo -e "; milliseconds before unresponsive clients gets kicked out" >> $configPath
  echo -e "connection_timeout = 5000" >> $configPath
  echo -e "" >> $configPath
    
  echo -e "[MISC]" >> $configPath
  echo -e "; Maximum number of snapshots to retain." >> $configPath
  echo -e "max_snapshots = 20" >> $configPath
  
  echo -e "; true|false" >> $configPath
  echo -e "console_enabled = true" >> $configPath
  echo -e "" >> $configPath
  
  echo -e "[SHARD]" >> $configPath
  echo -e "; Optimally a randomly generated key" >> $configPath
  echo -e "cluster_key = randomsecretkey" >> $configPath
  
  echo -e "shard_enabled = true" >> $configPath
  echo -e "bind_ip = 0.0.0.0" >> $configPath
  echo -e "master_ip = 127.0.0.1" >> $configPath
  echo -e "master_port = 11111" >> $configPath
  echo -e "" >> $configPath
  
  add_Forest $savePath
  success "主世界初始化完成"
}

add_Forest(){
  local savePath=$1
  local saveName=$(basename $savePath)
  local index=$(($(countDir $savePath)+1))
  local worldName="Forest${index}"
  local worldPath="$savePath/$worldName"
  local configPath="${worldPath}/server.ini"
  local leveldataoverridePath="${worldPath}/leveldataoverride.lua"
  local modoverridesPath="${worldPath}/modoverrides.lua"
  local isMaster="false"
  if [ $index = "1" ]; then
    isMaster="true"
  fi

  mkdir -p $worldPath
  
  echo -e "[NETWORK]" > $configPath
  echo -e "server_port = 10999" >> $configPath
  echo -e "" >> $configPath
  
  echo -e "[SHARD]" >> $configPath
  echo -e "name = $worldName" >> $configPath
  echo -e "is_master = $isMaster" >> $configPath
  echo -e "" >> $configPath
  
  echo -e "[ACCOUNT]" >> $configPath
  echo -e "encode_user_path = true" >> $configPath
  echo -e "" >> $configPath

  # TODO https://github.com/mathielo/dst-dedicated-server/blob/master/DSTClusterConfig/Master/leveldataoverride.lua
  echo -e "return {" > $leveldataoverridePath
  echo -e "  id=\"SURVIVAL_TOGETHER\"," >> $leveldataoverridePath
  echo -e "  name=\"标准森林\"," >> $leveldataoverridePath
  echo -e "  desc=\"标准《饥荒》体验。\"," >> $leveldataoverridePath
  echo -e "  hideminimap=false," >> $leveldataoverridePath
  echo -e "  location=\"forest\"," >> $leveldataoverridePath
  echo -e "  max_playlist_position=999," >> $leveldataoverridePath
  echo -e "  min_playlist_position=0," >> $leveldataoverridePath
  echo -e "  numrandom_set_pieces=4," >> $leveldataoverridePath
  echo -e "  override_level_string=false," >> $leveldataoverridePath
  echo -e "  overrides={" >> $leveldataoverridePath
  echo -e "    world_size=\"default\"," >> $leveldataoverridePath
  echo -e "  }," >> $leveldataoverridePath
  echo -e "  random_set_pieces={," >> $leveldataoverridePath
  echo -e "  }," >> $leveldataoverridePath
  echo -e "  required_prefabs={ \"multiplayer_portal\" }," >> $leveldataoverridePath
  echo -e "  required_setpieces={ \"Sculptures_1\", \"Maxwell5\" }," >> $leveldataoverridePath
  echo -e "  settings_desc=\"标准《饥荒》体验。\"," >> $leveldataoverridePath
  echo -e "  settings_id=\"SURVIVAL_TOGETHER",\" >> $leveldataoverridePath
  echo -e "  settings_name=\"标准森林\"," >> $leveldataoverridePath
  echo -e "  substitutes={  }," >> $leveldataoverridePath
  echo -e "  version=4," >> $leveldataoverridePath
  echo -e "  worldgen_desc=\"标准《饥荒》体验。\"," >> $leveldataoverridePath
  echo -e "  worldgen_id=\"SURVIVAL_TOGETHER\"," >> $leveldataoverridePath
  echo -e "  worldgen_name=\"标准森林\" " >> $leveldataoverridePath
  echo -e "}" >> $leveldataoverridePath

  echo -e "return {" > $modoverridesPath
  echo -e "}" >> $modoverridesPath
  
  add_server_shell $savePath $worldName '32'
}


add_Cave(){
  local savePath=$1
  local saveName=$(basename $savePath)
  local index=$(($(countDir $savePath)+1))
  local worldName="Caves${index}"
  local worldPath="$savePath/$worldName"
  local configPath="${worldPath}/server.ini"
  local leveldataoverridePath="${worldPath}/leveldataoverride.lua"
  local modoverridesPath="${worldPath}/modoverrides.lua"

  mkdir -p $worldPath

  echo -e "[STEAM]" > $configPath
  echo -e "master_server_port = 27017" >> $configPath
  echo -e "authentication_port = 8767" >> $configPath
  echo -e "" >> $configPath
  
  echo -e "[NETWORK]" >> $configPath
  echo -e "server_port = 10998" >> $configPath
  echo -e "" >> $configPath
  
  echo -e "[SHARD]" >> $configPath
  echo -e "id = 778967778" >> $configPath
  echo -e "name = $worldName" >> $configPath
  echo -e "is_master = false" >> $configPath
  echo -e "" >> $configPath
  
  echo -e "[ACCOUNT]" >> $configPath
  echo -e "encode_user_path = true" >> $configPath
  echo -e "" >> $configPath

  # TODO https://github.com/mathielo/dst-dedicated-server/blob/master/DSTClusterConfig/Caves/leveldataoverride.lua
  echo -e "return {" > $leveldataoverridePath
  echo -e "  background_node_range={ 0, 1 }," >> $leveldataoverridePath
  echo -e "  id=\"DST_CAVE\"," >> $leveldataoverridePath
  echo -e "  name=\"洞穴\"," >> $leveldataoverridePath
  echo -e "  desc=\"探查洞穴…… 一起！\"," >> $leveldataoverridePath
  echo -e "  hideminimap=false," >> $leveldataoverridePath
  echo -e "  location=\"cave\"," >> $leveldataoverridePath
  echo -e "  max_playlist_position=999," >> $leveldataoverridePath
  echo -e "  min_playlist_position=0," >> $leveldataoverridePath
  echo -e "  numrandom_set_pieces=0," >> $leveldataoverridePath
  echo -e "  override_level_string=false," >> $leveldataoverridePath
  echo -e "  overrides={" >> $leveldataoverridePath
  echo -e "    branching=\"default\"," >> $leveldataoverridePath
  echo -e "    touchstone=\"default\"," >> $leveldataoverridePath
  echo -e "    world_size=\"default\"," >> $leveldataoverridePath
  echo -e "    prefabswaps_start=\"default\"," >> $leveldataoverridePath
  echo -e "    loop=\"default\"," >> $leveldataoverridePath
  echo -e "    boons=\"default\"," >> $leveldataoverridePath
  echo -e "    cavelight=\"default\"," >> $leveldataoverridePath
  echo -e "    start_location=\"default\"," >> $leveldataoverridePath
  echo -e "    task_set=\"default\"," >> $leveldataoverridePath
  echo -e "    season_start=\"default\"," >> $leveldataoverridePath
  echo -e "    slurper=\"default\"," >> $leveldataoverridePath
  echo -e "    monkey=\"default\"," >> $leveldataoverridePath
  echo -e "    rocky=\"default\"," >> $leveldataoverridePath
  echo -e "    bunnymen=\"default\"," >> $leveldataoverridePath
  echo -e "    slurtles=\"default\"," >> $leveldataoverridePath
  echo -e "    fissure=\"default\"," >> $leveldataoverridePath
  echo -e "    spiders=\"default\"," >> $leveldataoverridePath
  echo -e "    tentacles=\"default\"," >> $leveldataoverridePath
  echo -e "    chess=\"default\"," >> $leveldataoverridePath
  echo -e "    worms=\"default\"," >> $leveldataoverridePath
  echo -e "    cave_spiders=\"default\"," >> $leveldataoverridePath
  echo -e "    bats=\"default\"," >> $leveldataoverridePath
  echo -e "    grass=\"default\"," >> $leveldataoverridePath
  echo -e "    rock=\"default\"," >> $leveldataoverridePath
  echo -e "    mushroom=\"default\"," >> $leveldataoverridePath
  echo -e "    cave_ponds=\"default\"," >> $leveldataoverridePath
  echo -e "    sapling=\"default\"," >> $leveldataoverridePath
  echo -e "    berrybush=\"default\"," >> $leveldataoverridePath
  echo -e "    trees=\"default\"," >> $leveldataoverridePath
  echo -e "    reeds=\"default\"," >> $leveldataoverridePath
  echo -e "    flint=\"default\"," >> $leveldataoverridePath
  echo -e "    fern=\"default\"," >> $leveldataoverridePath
  echo -e "    flower_cave=\"default\"," >> $leveldataoverridePath
  echo -e "    mushtree=\"default\"," >> $leveldataoverridePath
  echo -e "    wormlights=\"default\"," >> $leveldataoverridePath
  echo -e "    marshbush=\"default\"," >> $leveldataoverridePath
  echo -e "    lichen=\"default\"," >> $leveldataoverridePath
  echo -e "    banana=\"default\"," >> $leveldataoverridePath
  echo -e "    spiderqueen=\"default\"," >> $leveldataoverridePath
  echo -e "    liefs=\"default\"," >> $leveldataoverridePath
  echo -e "    toadstool=\"default\"," >> $leveldataoverridePath
  echo -e "    fruitfly=\"default\"," >> $leveldataoverridePath
  echo -e "    krampus=\"default\"," >> $leveldataoverridePath
  echo -e "    autumn=\"default\"," >> $leveldataoverridePath
  echo -e "    spring=\"default\"," >> $leveldataoverridePath
  echo -e "    specialevent=\"default\"," >> $leveldataoverridePath
  echo -e "    beefaloheat=\"default\"," >> $leveldataoverridePath
  echo -e "    winter=\"default\"," >> $leveldataoverridePath
  echo -e "    day=\"default\"," >> $leveldataoverridePath
  echo -e "    summer=\"default\"," >> $leveldataoverridePath
  echo -e "    rocky_setting=\"default\"," >> $leveldataoverridePath
  echo -e "    monkey_setting=\"default\"," >> $leveldataoverridePath
  echo -e "    moles_setting=\"default\"," >> $leveldataoverridePath
  echo -e "    dustmoths=\"default\"," >> $leveldataoverridePath
  echo -e "    snurtles=\"default\"," >> $leveldataoverridePath
  echo -e "    slurtles_setting=\"default\"," >> $leveldataoverridePath
  echo -e "    grassgekkos=\"default\"," >> $leveldataoverridePath
  echo -e "    lightfliers=\"default\"," >> $leveldataoverridePath
  echo -e "    bunnymen_setting=\"default\"," >> $leveldataoverridePath
  echo -e "    pigs_setting=\"default\"," >> $leveldataoverridePath
  echo -e "    mushgnome=\"default\"," >> $leveldataoverridePath
  echo -e "    spiders_setting=\"default\"," >> $leveldataoverridePath
  echo -e "    bats_setting=\"default\"," >> $leveldataoverridePath
  echo -e "    spider_dropper=\"default\"," >> $leveldataoverridePath
  echo -e "    spider_warriors=\"default\"," >> $leveldataoverridePath
  echo -e "    merms=\"default\"," >> $leveldataoverridePath
  echo -e "    spider_spitter=\"default\"," >> $leveldataoverridePath
  echo -e "    spider_hider=\"default\"," >> $leveldataoverridePath
  echo -e "    nightmarecreatures=\"default\"," >> $leveldataoverridePath
  echo -e "    molebats=\"default\"," >> $leveldataoverridePath
  echo -e "    flower_cave_regrowth=\"default\"," >> $leveldataoverridePath
  echo -e "    regrowth=\"default\"," >> $leveldataoverridePath
  echo -e "    mushtree_moon_regrowth=\"default\"," >> $leveldataoverridePath
  echo -e "    mushtree_regrowth=\"default\"," >> $leveldataoverridePath
  echo -e "    lightflier_flower_regrowth=\"default\"," >> $leveldataoverridePath
  echo -e "    spawnprotection=\"default\"," >> $leveldataoverridePath
  echo -e "    seasonalstartingitems=\"default\"," >> $leveldataoverridePath
  echo -e "    extrastartingitems=\"default\"," >> $leveldataoverridePath
  echo -e "    shadowcreatures=\"default\"," >> $leveldataoverridePath
  echo -e "    brightmarecreatures=\"default\"," >> $leveldataoverridePath
  echo -e "    dropeverythingondespawn=\"default\"," >> $leveldataoverridePath
  echo -e "    atriumgate=\"default\"," >> $leveldataoverridePath
  echo -e "    earthquakes=\"default\"," >> $leveldataoverridePath
  echo -e "    weather=\"default\"," >> $leveldataoverridePath
  echo -e "    wormattacks=\"default\"," >> $leveldataoverridePath
  echo -e "    layout_mode=\"RestrictNodesByKey\"," >> $leveldataoverridePath
  echo -e "    start_location=\"caves\"," >> $leveldataoverridePath
  echo -e "    task_set=\"cave_default\"," >> $leveldataoverridePath
  echo -e "    wormhole_prefab=\"tentacle_pillar\"," >> $leveldataoverridePath
  echo -e "  }," >> $leveldataoverridePath
  echo -e "  required_prefabs={ \"multiplayer_portal\" }," >> $leveldataoverridePath
  echo -e "  settings_desc=\"探查洞穴…… 一起！\"," >> $leveldataoverridePath
  echo -e "  settings_id=\"DST_CAVE\"," >> $leveldataoverridePath
  echo -e "  settings_name=\"洞穴\"," >> $leveldataoverridePath
  echo -e "  substitutes={  }," >> $leveldataoverridePath
  echo -e "  version=4," >> $leveldataoverridePath
  echo -e "  worldgen_desc=\"探查洞穴…… 一起！\"," >> $leveldataoverridePath
  echo -e "  worldgen_id=\"DST_CAVE\"," >> $leveldataoverridePath
  echo -e "  worldgen_name=\"洞穴\" " >> $leveldataoverridePath

  echo -e "}" >> $leveldataoverridePath
  
  echo -e "return {" > $modoverridesPath
  echo -e "}" >> $modoverridesPath

  add_server_shell $savePath $worldName '32'
}


function add_server_shell(){
  local savePath=$1
  local worldName=$2
  local bit=$3
  local saveName=$(basename $savePath)
  local shellPath="${savePath}/start${worldName}.sh"
  local binPath="${DSTServerPath}/bin"
  local extutorName="dontstarve_dedicated_server_nullrenderer"
  
  if [ $bit = "64" ]; then
    binPath="${DSTServerPath}/bin64"
    extutorName="dontstarve_dedicated_server_nullrenderer_x64"
  fi
  
  echo -e "#!/bin/bash" > $shellPath
  echo -e "cd \"${binPath}\"" >> $shellPath
  echo -e "run_shared=(./${extutorName})" >> $shellPath
  echo -e "run_shared+=(-console_enabled)" >> $shellPath
  # echo -e "run_shared+=(-skip_update_server_mods)" >> $shellPath
  echo -e "run_shared+=(-cluster \"$saveName\")" >> $shellPath
  echo -e "run_shared+=(-ugc_directory \"$UgcDirectoryPath\")" >> $shellPath
  echo -e "run_shared+=(-region sing)" >> $shellPath
  echo -e "run_shared+=(-monitor_parent_process $)" >> $shellPath
  echo -e "run_shared+=(-shard \"$worldName\")" >> $shellPath
  echo -e "\"\${run_shared[@]}\"" >> $shellPath

  chmod +x $shellPath
}

manage_saves(){
  echo "============================================================"
  echo "[1]新建存档 [2] 恢复存档 [3]备份存档 [4]删除存档 [0]返回菜单"
  echo "============================================================"
  read -p "请输入选项：" num
  case $num in
    1)
      clear
      create_save
      break;;
    2)
      recovery_save
      break;;
    3)
      if [ $DST_SaveCount -eq 0 ]; then
        clear
        log "当前无可备份存档，请先创建存档"
        manage_saves
      else
        backup_save
      fi
      break;;
    4)
      clear
      if [ $DST_SaveCount -eq 0 ]; then
        log "当前无可删除存档，请先创建存档"
        manage_saves
      else
        show_saves
        delete_save
      fi
      break;;
    0)
      clear
      menu
      break;;
    *)
      clear
      error "输入错误 请重新输入"
      manage_saves
  esac
}

select_save(){
  show_saves
  local savePath=""
  while true :
  do
    txt_orange "请选择存档："
    read input
    if [[ $input == "0" ]]; then
      break
    fi
    savePath="$DSTSavesPath/$input"
    if [ ! -d $savePath ]; then
      error "存档($input)不存在,请重新输入"
      continue
    fi
    break
  done
  CurOperateSavePath=$savePath
}

manage_mod(){
  local savePath=$1
  clear
  log "操作存档：$(basename $savePath)"
  echo "==============================================================="
  echo "[1]新增mod [2]新增mod合集 [3]删除mod [4]清空mod [0]返回上级菜单"
  echo "==============================================================="
  txt_orange "请选择操作："
  read input
  case $input in
    1)
      add_mod $savePath
      break;;
    2)
      add_mod_collection $savePath
      break;;
    3)
      remove_mod $savePath
      break;;
    4)
      clear_mod $savePath
      break;;
    0)
      clear
      select_save
      manage_mod $CurOperateSavePath
      break;;
  esac
}

add_mod_collection(){
  echo 1
}

add_mod(){
  local savePath=$1
  while :
  do
    txt_orange "请输入待新增的mod编号："
    read modId
    if [[ $modId == "0" ]]; then
      break
    fi
    if [ -z $modId ]; then
      error "输入不可为空值！！！"
      continue
    fi

    local breakFlag="false"
    for file in `find $savePath -maxdepth 2 -name "modoverrides.lua"`
    do
      if [[ `grep "$modId" $file` ]]; then
        error "mod($modId)已存在！！！"
        breakFlag="true"
        break
      fi
      sed -i '$d' $file
      echo "  [\"workshop-$modId\"]={ enabled=true, configuration_options={  } }," >> $file
      echo "}" >> $file
    done
    if [ $breakFlag = "false" ]; then
      log "mod($modId)添加成功"
    fi

    if [[ ! `grep "$modId" "${DSTServerModSetupPath}"` ]]; then        
      echo "ServerModSetup(\"$modId\")" >> "${DSTServerModSetupPath}"
    fi
  done

  manage_mod $CurOperateSavePath
}

update_mod(){
  local tempSaveName="temp_save"
  cd "${DSTServerPath}/bin"
  ./dontstarve_dedicated_server_nullrenderer -cluster "${tempSaveName}" -shard Master -only_update_server_mods -ugc_directory "${UgcDirectoryPath}"
  rm -rf ${DSTSavesPath}/${tempSaveName}
  cd "$HomePath"
}

remove_mod(){
  local savePath=$1
  local saveName=$(basename $savePath)
  txt_orange "请输入待删除的mod编号："
  read input
  if [[ $input == "0" ]]; then
    manage_mod $savePath
  fi

  local breakFlag="false"
  for file in `find $savePath -maxdepth 2 -name "modoverrides.lua"`
  do
    if [[ `grep "$input" $file` ]]; then
      deletedFlag="true"
      sed -i "/workshop-$input/d" $file
    else
      breakFlag="true"
      break
    fi
  done
  if [ $breakFlag = "true" ]; then
    log "mod($input)不存在！！！"
  else
    success "mod($input)删除成功"
  fi
  remove_mod $savePath
}

clear_mod(){
  local savePath=$1
  local saveName=$(basename $savePath)
  txt_orange "请再次输入存档名字以确认："
  read input
  if [ ! $input = $saveName ]; then
    error "输入错误！！！"
    clear_mod $savePath
  fi
  for file in `find $savePath -maxdepth 2 -name "modoverrides.lua"`
  do
    echo -e "return {" > $file
    echo -e "}" >> $file
  done
  success "清空mod成功"
  sleep 1
  manage_mod $savePath
}

backup_save(){
  show_saves
  read -p "请输入需要备份的存档编号：" num
  echo $num
}

recovery_save(){
  show_saves
  read -p "请输入需要恢复的存档编号：" num
  echo $num
}

delete_save(){
  txt_orange " 请输入需要删除的存档名称："
  read filename
  if [ -z ${filename} ]; then
    :
  elif [ -d "${DSTSavesPath}/${filename}" ]; then
    rm -rf ${DSTSavesPath}/${filename}
    success "删除成功"
    sleep 1
    clear
  else
    error "该存档不存在，请重新输入"
  fi
  get_saves_count
  if [ $DST_SaveCount -eq 0 ]; then
    clear
    menu
  else
    show_saves
    delete_save
  fi
}

function exit_shell(){
  clear
  exit 1
}

clear
get_status
check_env
clear
menu
