#!/bin/bash
#-------------------------------------------------------------------------------------------
#go.sh 是一个 饥荒联机版 shell脚本
#-------------------------------------------------------------------------------------------


#-------------------------------------------------------------------------------------------
# 脚本历史更新内容
#-------------------------------------------------------------------------------------------
#欲醉无由 2016.11.12  > 创建脚本 初步支持 Ubuntu 环境
#小草哥哥 2020.02.16  > 增加 CentOS 环境     
#小草哥哥 2020.12.03  > 1. 推出测试服务器版本（由于降级要重装已取消该功能） 2. 增加查看服务器版本			
#小草哥哥 2021.09.12  > 1. 面板重新排版 2. 增加 x64 x32 启动模式 3. 修复更新bug 4. 增加退出脚本操作
#			5. 调整关闭服务器无输入按回车返回脚本主目录 6. 为所有函数增加注释
#小草哥哥 2021.09.14  > 1. 修复 CentOS 系统动态库遇到的 libcurl-gnutls.so.4 问题
#小草哥哥 2021.09.19  > 1. 增加开启服务器输入‘quit’返回主菜单
#小草哥哥 2021.09.29  > 1. 增加 Arch 环境
#小草哥哥 2022.01.09  > 1. 增加启动主世界自动更新mod（v1、v2） 需配合（dedicated_server_mods_setup.lua）
#           v1 版本 mod 依旧存放在 'mods' 文件夹内，v2 版本 mod 存放在 ‘v2mods’ 文件夹内（与mods文件夹同级）
#           主世界与洞穴世界共用 ‘v2mods’ 文件夹！所以只需要主世界更新即可，第一次启动不会加载v2 mods ，需主世界
#           在正常启动一段时间后，正常加载v1 mods，服务器才会额外加在v2 版本mods ，待加载完直接重新启动服务器即可
#小草哥哥 2022.02.01  > 1. 注释忽略主服务器模组更新选项 -skip_update_server_mods  2. 模组管理增加更新选项
            3. 增加主页模组状态显示
#
#
#-------------------------------------------------------------------------------------------

# 通用变量

# 系统发行版本获取 例如：Arch linux
    os=$(awk -F = '/^NAME/{print $2}' /etc/os-release | sed 's/"//g' | sed 's/ //g' | sed 's/Linux//g' | sed 's/linux//g')
    NeedsUpdate1="初始化"
    NeedsDownload1="初始化"
# Mods v2更新

# 退出脚本
function ExitShell()
{
clear
exit

}

# 查看版本
function ShowVerSion()
{
ServerVer=$(cat ./Steam/steamapps/common/Don\'t\ Starve\ Together\ Dedicated\ Server/version.txt)
clear
echo "--------------"
echo "| 服务器版本 |"
echo "--------------"
echo "    ${ServerVer}" 
echo "--------------"
Main
}

# 白名单管理
function SetWhite()
{
echo "============================================"
echo "[1]加入白名单 [2]放出白名单 "
read white1
case $white1 in
1)
echo "============================================"
echo "请输入要加入白名单的Klei ID"
read ID2
if [ -d ./.klei/DoNotStarveTogether/$filenumber/Master/save ]
then
    cd ./.klei/DoNotStarveTogether/$filenumber/Master/save
    if [[ ! `grep "$ID2" whitelist.txt` ]]
    then 
        echo "$ID2" >> whitelist.txt
        echo "已为这个基佬预留一个位置"
    else
        echo "这个基佬已经有一个位置"
    fi
else
    echo "该存档没有地上世界"
fi
cd $HOME
;;
2)
echo "============================================"
echo "请输入要解除白名单的Klei ID"
read ID2
if [ -d ./.klei/DoNotStarveTogether/$filenumber/Master/save ]
then
    cd ./.klei/DoNotStarveTogether/$filenumber/Master/save
    if [[ `grep "$ID2" whitelist.txt` ]]
    then 
        sed -i "/$ID2/d" whitelist.txt
        cd $HOME
        echo "预留位置已开放"
    else
        echo "没有这个基佬的预留位置"
    fi
else
    echo "该存档没有地上世界"
fi
cd $HOME
;;
esac
Main
}

# 黑名单管理
function SetBlack()
{
echo "============================================"
echo "[1]加入黑名单 [2]放出黑名单 "
read black1
case $black1 in
1)
echo "============================================"
echo "请输入要加入黑名单的Klei ID"
read ID2
if [ -d ./.klei/DoNotStarveTogether/$filenumber/Master/save ]
then
    cd ./.klei/DoNotStarveTogether/$filenumber/Master/save
    if [[ ! `grep "$ID2" blacklist.txt` ]]
    then 
        echo "$ID2" >> blacklist.txt
        echo "SB已被关入地上小黑屋"
    else
        echo "这个SB已经在小黑屋"
    fi
else
    echo "该存档没有地上世界"
fi
cd $HOME
if [ -d ./.klei/DoNotStarveTogether/$filenumber/Caves/save ]
then
    cd ./.klei/DoNotStarveTogether/$filenumber/Caves/save
    if [[ ! `grep "$ID2" blacklist.txt` ]]
    then 
        echo "$ID2" >> blacklist.txt
        echo "SB已被关入地下小黑屋"
    else
        echo "这个SB已经在小黑屋"
    fi
else
    echo "该存档没有地下世界"
fi
;;
2)
echo "============================================"
echo "请输入要解除黑名单的Klei ID"
read ID2
if [ -d ./.klei/DoNotStarveTogether/$filenumber/Master/save ]
then
    cd ./.klei/DoNotStarveTogether/$filenumber/Master/save
    if [[ `grep "$ID2" blacklist.txt` ]]
    then 
        sed -i "/$ID2/d" blacklist.txt
        cd $HOME
        echo "已放出地上小黑屋"
    else
        echo "这个基佬不在小黑屋"
    fi
else
    echo "该存档没有地上世界"
fi
cd $HOME
if [ -d ./.klei/DoNotStarveTogether/$filenumber/Caves/save ]
then
    cd ./.klei/DoNotStarveTogether/$filenumber/Caves/save
    if [[ `grep "$ID2" blacklist.txt` ]]
    then 
        sed -i "/$ID2/d" blacklist.txt
        cd $HOME
        echo "已放出地下小黑屋"
    else
        echo "这个基佬不在小黑屋"
    fi
else
    echo "该存档没有地下世界"
fi
;;
esac
Main
}

# 添加管理员
function SetAdmin()
{	
echo "============================================"
echo "[1]提升管理员 [2]解除管理员 "
read admin1
case $admin1 in
1)
echo "============================================"
echo "请输入要提升管理员的Klei ID"
read ID2
if [ -d ./.klei/DoNotStarveTogether/$filenumber/Master/save ]
then
    cd ./.klei/DoNotStarveTogether/$filenumber/Master/save
    if [[ ! `grep "$ID2" adminlist.txt` ]]
    then 
        echo "$ID2" >> adminlist.txt
        echo "地上管理员已设置"
    else
        echo "这个基佬已经是管理员"
    fi
else
    echo "该存档没有地上世界"
fi
cd $HOME
if [ -d ./.klei/DoNotStarveTogether/$filenumber/Caves/save ]
then
    cd ./.klei/DoNotStarveTogether/$filenumber/Caves/save
    if [[ ! `grep "$ID2" adminlist.txt` ]]
    then 
        echo "$ID2" >> adminlist.txt
        echo "地下管理员已设置"
    else
        echo "这个基佬已经是管理员"
    fi
else
    echo "该存档没有地下世界"
fi
;;
2)
echo "============================================"
echo "请输入要解除管理员的Klei ID"
read ID2
if [ -d ./.klei/DoNotStarveTogether/$filenumber/Master/save ]
then
    cd ./.klei/DoNotStarveTogether/$filenumber/Master/save
    if [[ `grep "$ID2" adminlist.txt` ]]
    then 
        sed -i "/$ID2/d" adminlist.txt
        cd $HOME
        echo "地上管理员已移除"
    else
        echo "这个基佬不是管理员"
    fi
else
    echo "该存档没有地上世界"
fi
cd $HOME
if [ -d ./.klei/DoNotStarveTogether/$filenumber/Caves/save ]
then
    cd ./.klei/DoNotStarveTogether/$filenumber/Caves/save
    if [[ `grep "$ID2" adminlist.txt` ]]
    then 
        sed -i "/$ID2/d" adminlist.txt
        cd $HOME
        echo "地下管理员已移除"
    else
        echo "这个基佬不是管理员"
    fi
else
    echo "该存档没有地下世界"
fi
;;
esac
Main
}

# 存档管理员设置
function Listmanage()
{
echo "============================================"
echo "存档目录"
cd $HOME
ls -l ./.klei/DoNotStarveTogether |awk '/^d/ {print $NF}'
echo "请输入要管理Mod的存档代码"
read filenumber
while :
do
echo "============================================"
echo "[1]设置管理员 [2]管理黑名单 [3]管理白名单"
read list1
case $list1 in
    1)SetAdmin
    break;;
    2)SetBlack
    break;;
    3)SetWhite
    break;;
esac
done
}
# 模组更新提示函数
function UpNoteMod()
{
    if [ -f ~/Steam/steamapps/common/Don\'t\ Starve\ Together\ Dedicated\ Server/v2mods/appworkshop_322330.acf ]
    then
        NeedsUpdate=$(awk '/NeedsUpdate/{print $2}' ~/Steam/steamapps/common/Don\'t\ Starve\ Together\ Dedicated\ Server/v2mods/appworkshop_322330.acf | sed 's/"//g')
        NeedsDownload=$(awk '/NeedsDownload/{print $2}' ~/Steam/steamapps/common/Don\'t\ Starve\ Together\ Dedicated\ Server/v2mods/appworkshop_322330.acf | sed 's/"//g')
        if [ NeedsUpdate==0 ]
        then
        
            NeedsUpdate1="无需更新"
            
        else
        
            NeedsUpdate1="需更新"
        
        fi
        if [ NeedsDownload=0 ]
        then
        
            NeedsDownload1="无需下载"
        
        else
        
            NeedsDownload1="需下载"
        
        fi
    fi

}

# MOD自动更新管理
function UpdateMod()
{
    cd Steam/steamapps/common/Don\'t\ Starve\ Together\ Dedicated\ Server/bin
    
    ./dontstarve_dedicated_server_nullrenderer -only_update_server_mods -ugc_directory "../v2mods"
}

# 删除Mod
function RemMod()
{
while :
do
echo "============================================"
echo "请输入要移除的modID（返回主菜单输入0）"
read ID1
case $ID1 in
    0)break;;
    *)
if [ -d ./.klei/DoNotStarveTogether/$filenumber/Master ]
then
    cd ./.klei/DoNotStarveTogether/$filenumber/Master
    if [[ `grep "$ID1" modoverrides.lua` ]]
    then 
        sed -i "/$ID1/d" modoverrides.lua
        cd $HOME
        echo "地上世界Mod移除完成"
    else
        echo "地上世界该Mod不存在"
    fi
else
    echo "该存档没有地上世界"
fi
cd $HOME
if [ -d ./.klei/DoNotStarveTogether/$filenumber/Caves ]
then
    cd ./.klei/DoNotStarveTogether/$filenumber/Caves
    if [[ `grep "$ID1" modoverrides.lua` ]]
    then 
        sed -i "/$ID1/d" modoverrides.lua
        cd $HOME
        echo "地下世界Mod移除完成"
    else
        echo "地下世界该Mod不存在"
    fi
else
    echo "该存档没有地下世界"
fi
esac
done
Main
}

# 添加 Mod
function AddMod()
{
while :
do
echo "============================================"
echo "请输入要添加的modID（返回主菜单输入0）"
read ID1
case $ID1 in
    0)break;;
    *)
        if [ -d ./.klei/DoNotStarveTogether/$filenumber/Master ]
        then
            cd ./.klei/DoNotStarveTogether/$filenumber/Master
            if [[ ! `grep "$ID1" "modoverrides.lua"` ]]
            then 
                sed -i '$d' modoverrides.lua
                echo "[\"workshop-$ID1\"]={ configuration_options={  }, enabled=true }," >> modoverrides.lua
                echo "}" >> modoverrides.lua
                cd $HOME
                mod3=1
                echo "地上世界Mod添加完成"
            else
                echo "地上世界该Mod已存在"
            fi
        else
            echo "该存档没有地上世界"
            mod3=0
        fi
        cd $HOME 
        if [ -d ./.klei/DoNotStarveTogether/$filenumber/Caves ]
        then
            cd ./.klei/DoNotStarveTogether/$filenumber/Caves
            if [[ ! `grep "$ID1" modoverrides.lua` ]]
            then 
                sed -i '$d' modoverrides.lua
                echo "[\"workshop-$ID1\"]={ configuration_options={  }, enabled=true }," >> modoverrides.lua
                echo "}" >> modoverrides.lua
                cd $HOME
                mod4=$[mod3+1]
                echo "地下世界Mod添加完成"
            else
                echo "地下世界该Mod已存在"
            fi
        else
            echo "该存档没有地下世界"
            mod4=$[mod3+0]
        fi
        cd "./Steam/steamapps/common/Don't Starve Together Dedicated Server/mods"
            if [[ ! `grep "$ID1" dedicated_server_mods_setup.lua` ]]
            then        
                echo "ServerModSetup(\"$ID1\")" >> dedicated_server_mods_setup.lua
            fi
        cd "$HOME"
        case mod4 in
            0)  echo "该存档不存在"
            ;;
            *)
            ;;
        esac
        echo "$mod4"
esac
done
Main
}

# Mod 管理
function Modmanage()
{
echo "============================================"
echo "存档目录"
cd $HOME
ls -l ./.klei/DoNotStarveTogether |awk '/^d/ {print $NF}'
echo "请输入要管理Mod的存档代码"
read filenumber
while :
do
echo "============================================"
echo "[1]添加模组 [2]移除模组 [3]更新模组 [4]清空模组（慎用）"
read mod1
case $mod1 in
    1)AddMod
    break;;
    2)RemMod
    break;;
    3)UpdateMod
    break;;
    4)echo "请再次输入要清空Mod的存档代码"
      read filenumber
      if [ -d ./.klei/DoNotStarveTogether/$filenumber/Master ]
      then 
      echo "return {
             }" > ./.klei/DoNotStarveTogether/$filenumber/Master/modoverrides.lua
      fi
      if [ -d ./.klei/DoNotStarveTogether/$filenumber/Caves ]
      then 
      echo "return {
             }" > ./.klei/DoNotStarveTogether/$filenumber/Caves/modoverrides.lua
      fi
      echo "Mod已清空"
    break;;
esac	
done
}

# 存档删除
function Delete()
{
echo "============================================"
echo "存档目录"
ls -l ./.klei/DoNotStarveTogether |awk '/^d/ {print $NF}' 
echo "请输入要删除的存档代码"
read filenumber
    cd "./.klei/DoNotStarveTogether"
    if [ -d $filenumber/Master/save ]
    then
    rm -r $filenumber/Master/save
    fi
    if [ -d $filenumber/Caves/save ]
    then
    rm -r $filenumber/Caves/save
    fi
    echo "存档已删除"
    if [ -f "${filenumber}.tar.gz" ] 
    then
        echo "是否删除备份？[y/n]"
        read delbackup1
            case delbackup1 in
                1)rm ${filenumber}.tar.gz
                echo "备份已删除"
                ;;
                2)
                ;;
            esac
    fi
    cd $HOME
    Filemanage
}

# 存档恢复
function Recovery()
{
echo "============================================"
echo "存档目录"
ls -l ./.klei/DoNotStarveTogether |awk '/^d/ {print $NF}' 
echo "请输入要恢复的存档代码"
read filenumber 
    cd "./.klei/DoNotStarveTogether"
    if [ -f "${filenumber}.tar.gz" ]
    then
        echo "存档已找到，正在恢复"
        if [ -d "$filenumber" ]
        then
            rm -r $filenumber
        fi
        tar -zxf ${filenumber}.tar.gz $filenumber
        echo "存档已恢复"
    else
        echo "存档未找到"
    fi
    cd $HOME
    Filemanage
        
}

# 存档备份
function Backup()
{
echo "============================================"
echo "存档目录"
ls -l ./.klei/DoNotStarveTogether |awk '/^d/ {print $NF}'
echo "请输入要备份的存档代码"
read filenumber
    cd "./.klei/DoNotStarveTogether"
    if [ -d "$filenumber" ]
    then
        if [ ! -f "${filenumber}.tar.gz" ]
        then
            tar -zcf ${filenumber}.tar.gz $filenumber
            echo  "备份成功"
        else
        echo "备份已存在，是否覆盖？[y/n]"
            read backup2
            case $backup2 in
                y)tar -zcf ${filenumber}.tar.gz $filenumber
                  echo  "备份成功"
                  cd $HOME
                  Filemanage
                ;;
                n)cd $HOME
                  Filemanage
            esac
        fi
    else 
        echo "该存档不存在"
    fi
    cd $HOME
    Filemanage
}

# 存档管理
function Filemanage()
{
echo "============================================"
echo "[1]备份存档 [2]恢复存档 [3]删除存档"
read filemanage1
    case $filemanage1 in
        1)Backup
        ;;
        2)Recovery
        ;;
        3)Delete
        ;;
        *)echo "指令无效";Filemanage
    esac

}

# 启动地下世界
function StartCaves()
{
echo "#!/bin/bash

gamesPath=\"Steam/steamapps/common/Don't Starve Together Dedicated Server/bin$ModebitPath\"
cd \"\$HOME\"
cd \"\$gamesPath\"

run_shared=(./dontstarve_dedicated_server_nullrenderer$Modebit)

# start caves
run_shared+=(-console)
run_shared+=(-skip_update_server_mods)
run_shared+=(-cluster "$filenumber")
run_shared+=(-ugc_directory \"../v2mods\")
run_shared+=(-region sing)
run_shared+=(-monitor_parent_process $)

\"\${run_shared[@]}\" -shard Caves" > ./.klei/DoNotStarveTogether/$filenumber/startcaves.sh
cd ./.klei/DoNotStarveTogether/$filenumber
chmod u+x ./startcaves.sh
cd $HOME
screen -S "Caves Server$Modebit $filenumber" "./.klei/DoNotStarveTogether/$filenumber/startcaves.sh"
}

# 启动主世界
function StartMaster()
{
echo "#!/bin/bash

gamesPath=\"Steam/steamapps/common/Don't Starve Together Dedicated Server/bin$ModebitPath\"
cd \"\$HOME\"
cd \"\$gamesPath\"

run_shared=(./dontstarve_dedicated_server_nullrenderer$Modebit)

# start Master
run_shared+=(-console)

# 忽略更新服务器模组（弃用）
# run_shared+=(-skip_update_server_mods)

run_shared+=(-cluster "$filenumber")
run_shared+=(-ugc_directory \"../v2mods\")
run_shared+=(-region sing)
run_shared+=(-monitor_parent_process $)

\"\${run_shared[@]}\" -shard Master" > ./.klei/DoNotStarveTogether/$filenumber/startmaster.sh
cd ./.klei/DoNotStarveTogether/$filenumber
chmod u+x ./startmaster.sh
cd $HOME
screen -S "Master Server$Modebit $filenumber" "./.klei/DoNotStarveTogether/$filenumber/startmaster.sh"
}

# 创建服务器世界配置文件2
function Serversetting2()
{

while :
do
echo "请选择模式：1.无尽 2.生存 3.荒野"
read gamemode1
case $gamemode1 in
    1)
    gamemode1="endless"
    break;;
    2)
    gamemode1="survival"
    break;;
    3)
    gamemode1="wilderness"
    break;;
esac
done

echo "请输入最大玩家数量："
read players

while :
do
echo "是否开启pvp？[y/n]"
read ifpvp
case $ifpvp in
    y)
    ifpvp="true"
    break;;
    n)
    ifpvp="false"
    break;;
esac
done

while :
do
echo "是否开启暂停？[y/n]"
read ifpause
case $ifpause in
    y)
    ifpause1="true"
    break;;
    n)
    ifpause1="false"
    break;;
esac
done

while :
do
echo "请选择游戏难度：1.休闲 2.合作 3.竞赛 4.疯狂"
read intention
case $intention in
    1)
    intention1=social
    break;;
    2)
    intention1=cooperative
    break;;
    3)
    intention1=competitive
    break;;
    4)
    intention1=madness
    break;;
esac
done

while :
do
echo "控制台是否打开？[y/n]"
read console
case $console in
    y)
    console1="true"
    break;;
    n)
    console1="false"
    break;;
esac
done
echo "请输入你的饥荒令牌："
read token

echo "$token" > ./.klei/DoNotStarveTogether/$filenumber/cluster_token.txt

echo "请输入服务器名字："
read servername

echo "请输入服务器介绍：PS：若无请按Enter键"
read description

echo "请输入服务器密码：PS：若无请按Enter键"
read password

echo "请输入地上世界服务器外网IP:"
read masterip

echo "[GAMEPLAY]
game_mode = $gamemode1
max_players = $players
pvp = $ifpvp
pause_when_empty = $ifpause1


[NETWORK]
cluster_description = $description
cluster_name = $servername
cluster_intention = $intention1
cluster_password = $password


[MISC]
console_enabled = $console1


[SHARD]
shard_enabled = true
bind_ip = 0.0.0.0
master_ip = $masterip
master_port = 10889
cluster_key = supersecretkey" > ./.klei/DoNotStarveTogether/$filenumber/cluster.ini
clear
echo "服务器配置完成！"
}

# 创建服务器世界配置文件
function Serversetting1()
{
while :
do
echo "请选择模式：1.无尽 2.生存 3.荒野"
read gamemode
case $gamemode in
    1)
    gamemode1="endless"
    break;;
    2)
    gamemode1="survival"
    break;;
    3)
    gamemode1="wilderness"
    break;;
esac
done

echo "请输入最大玩家数量："
read players

while :
do
echo "是否开启pvp？[y/n]"
read ifpvp
case $ifpvp in
    y)
    ifpvp="true"
    break;;
    n)
    ifpvp="false"
    break;;
esac
done

while :
do
echo "是否开启暂停？[y/n]"
read ifpause
case $ifpause in
    y)
    ifpause1="true"
    break;;
    n)
    ifpause1="false"
    break;;
esac
done

while :
do
echo "请选择游戏难度：1.休闲 2.合作 3.竞赛 4.疯狂"
read intention
case $intention in
    1)
    intention1=social
    break;;
    2)
    intention1=cooperative
    break;;
    3)
    intention1=competitive
    break;;
    4)
    intention1=madness
    break;;
esac
done

while :
do
echo "控制台是否打开？[y/n]"
read console
case $console in
    y)
    console1="true"
    break;;
    n)
    console1="false"
    break;;
esac
done
echo "请输入你的饥荒令牌："
read token

echo "$token" > ./.klei/DoNotStarveTogether/$filenumber/cluster_token.txt

echo "请输入服务器名字："
read servername

echo "请输入服务器介绍：PS：若无请按Enter键"
read description

echo "请输入服务器密码：PS：若无请按Enter键"
read password

echo "[STEAM]
steam_group_admins = true
steam_group_id = 30690547
steam_group_only = false

[GAMEPLAY]
game_mode = $gamemode1
max_players = $players
pvp = $ifpvp
pause_when_empty = $ifpause1


[NETWORK]
cluster_description = $description
cluster_name = $servername
cluster_intention = $intention1
cluster_password = $password


[MISC]
console_enabled = $console1
max_snapshots = 40


[SHARD]
shard_enabled = true
bind_ip = 0.0.0.0
master_ip = 127.0.0.1
master_port = 11111
cluster_key = supersecretkey" > ./.klei/DoNotStarveTogether/$filenumber/cluster.ini
clear
echo "服务器配置完成！"
}

# 创建地下世界配置文件
function CreatCavesini()
{
echo "[NETWORK]
server_port = 11101


[SHARD]
is_master = false
name = Caves


[STEAM]
master_server_port = 27019
authentication_port = 8769" > ./.klei/DoNotStarveTogether/$filenumber/Caves/server.ini
echo "return {
override_enabled = true,
preset = \"DST_CAVE\",
}" > ./.klei/DoNotStarveTogether/$filenumber/Caves/worldgenoverride.lua
echo "return {
}" >> "./.klei/DoNotStarveTogether/$filenumber/Caves/modoverrides.lua"
}

# 创建主世界配置文件
function CreatMasterini()
{
echo "[NETWORK]
server_port = 11100

[SHARD]
is_master = true

[STEAM]
master_server_port = 27018
authentication_port = 8768" > ./.klei/DoNotStarveTogether/$filenumber/Master/server.ini
echo "return {
}" >> "./.klei/DoNotStarveTogether/$filenumber/Master/modoverrides.lua"
}

# 创建地下世界文件夹
function Cavesfile()
{
echo "============================================"
echo "请输入存档代码"
read filenumber
if [ ! -d "./klei/DoNotStarveTogether/$filenumber" ]
then 
    mkdir -p ./.klei/DoNotStarveTogether/$filenumber/Caves
fi
CreatCavesini
Serversetting2
StartCaves
}

# 创建主世界文件夹
function Masterfile()
{
echo "============================================"
echo "请输入存档代码"
read filenumber
if [ ! -d "./klei/DoNotStarveTogether/$filenumber" ]
then 
    mkdir -p ./.klei/DoNotStarveTogether/$filenumber/Master
fi
CreatMasterini
Serversetting2
StartMaster
}

# 创建世界
function MasterCaves()
{
echo "============================================"
while :
do
echo "[1]地上 [2]地下"
read masterCaves1
case $masterCaves1 in
    1)Masterfile
    break;;
    2)Cavesfile
    break;;
esac
done
}

# 判断存档文件
function Filechose()
{ 
if [  -d "./.klei/DoNotStarveTogether/$filenumber/Master" ]; then
    zzz=1
else
    zzz=0
fi
if [ -d "./.klei/DoNotStarveTogether/$filenumber/Caves" ] ; then
    zz=$[zzz+2]
else
    zz=$[zzz+4]
fi
case $zz in
        2)StartCaves
        ;;
        3)StartMaster;StartCaves;
        ;;
        4)echo "存档没有内容，是否新建？[y/n]"
          read newfile1
          case $newfile1 in
            y)Modechose
            ;;
            n)Main
          esac
        ;;
        5)StartMaster
        ;;
esac
}

# 创建世界目录
function Newfile()
{ 
mkdir -p ./.klei/DoNotStarveTogether/$filenumber/Master
mkdir -p ./.klei/DoNotStarveTogether/$filenumber/Caves
Serversetting1
CreatMasterini
CreatCavesini
StartMaster
StartCaves
}

# 启动 64 bit 服务器
function Startserver64()
{

ModebitPath=64
Modebit=_x64

echo "============================================"
echo "存档目录"
ls -l ./.klei/DoNotStarveTogether |awk '/^d/ {print $NF}'
echo "============================================"
echo "请输入存档代码"
read filenumber
if [ -d "./.klei/DoNotStarveTogether/$filenumber" ];then
    Filechose
elif [ $filenumber == quit ];then
    clear
    Main
else
    echo "存档不存在，是否新建？[y/n]" 
    read newfile2
    while :
    do
        case $newfile2 in
            y)Modechose
            break;;
            n)Main
            break;;
        esac
    done
fi
}

# 启动 32 bit 服务器
function Startserver32()
{
ModebitPath=""
Modebit=""
echo "============================================"
echo "存档目录"
ls -l ./.klei/DoNotStarveTogether |awk '/^d/ {print $NF}'
echo "============================================"
echo "请输入存档代码"
read filenumber
if [ -d "./.klei/DoNotStarveTogether/$filenumber" ];then
    Filechose
elif [ $filenumber == quit ];then
    clear
    Main
else
    echo "存档不存在，是否新建？[y/n]" 
    read newfile2
    while :
    do
        case $newfile2 in
            y)Modechose
            break;;
            n)Main
            break;;
        esac
    done
fi
}

# 关闭服务器
function CloseServer()
{
echo "============================================"
screen -ls
echo "============================================"
echo "直接回车返回主目录"
echo "输入要切换的PID"
echo "PS:回车后会进入地上或地下的运行界面"
echo "   手动输入c_shutdown(true)回车保存退出"
echo "   进入后不想关闭请按ctrl+a+d"

read pid1

if [ -z "$pid1" ];then
clear
Main
else
screen -r $pid1
fi

}

# 单双服务器模式选择
function Modechose()
{
echo "============================================"
while :
do
echo "[1]单服务器搭建 [2]双服务器搭建 "
read servermode
case $servermode in
    1)Newfile
    break;;
    2)MasterCaves
    break;;
esac
done	
}

# 服务器更新
function Gameupdate()
{
cd ./steamcmd
> steamcmd.log
./steamcmd.sh +login anonymous +app_update 343050 validate +quit | tee steamcmd.log
grep "343050" steamcmd.log
if [ $? -eq 0 ];then
    cd "$HOME"
    clear
    echo "更新完毕"
    Main
else
    echo "更新失败，请重新尝试再次更新！"
    Main
fi

}

# 配置 系统环境库
function Library()
{

if [ "$os" == "Ubuntu" ];then

     echo ""
     echo "##########################"
     echo "# 加载 Ubuntu Linux 环境 #"
     echo "##########################"
     echo ""
     
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

elif [ "$os" == "CentOS" ];then

     echo ""
     echo "##########################"
     echo "# 加载 CentOS Linux 环境 #"
     echo "##########################"
     echo ""
     
     sudo yum -y update
     sudo yum -y install tar wget screen

     # 加载 32bit 库
     sudo yum -y install glibc.i686 libstdc++.i686 libcurl.i686
     
     # 加载 64bit 库
     sudo yum -y install glibc libstdc++ libcurl
     if [ -f "/usr/lib/libcurl.so.4" ];then
        ln -sf /usr/lib/libcurl.so.4 /usr/lib/libcurl-gnutls.so.4	
     fi
     if [ -f "/usr/lib64/libcurl.so.4" ];then
        ln -sf /usr/lib64/libcurl.so.4 /usr/lib64/libcurl-gnutls.so.4
     fi
elif [ "$os" == "Arch" ];then

     echo ""
     echo "########################"
     echo "# 加载 Arch Linux 环境 #"
     echo "########################"
     echo ""


     sudo pacman -Syyy
     sudo pacman -S --noconfirm wget screen
     sudo pacman -S --noconfirm lib32-gcc-libs libcurl-gnutls
     
     
     
else

     echo "该系统未被本脚本支持！"
     
fi


}

# 安装 SteamCMD
function Prepare()
{
if [ ! -d "./steamcmd" ];then
    mkdir $HOME/.klei/DoNotStarveTogether -p
    Library
    mkdir ./steamcmd
    cd ./steamcmd
    wget https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz
    tar -xvzf steamcmd_linux.tar.gz
    rm -f steamcmd_linux.tar.gz

    ./steamcmd.sh +login anonymous +app_update 343050 validate +quit
fi

# MOD更新需要的文件夹
mkdir -p "$HOME/Steam/steamapps/common/Don't Starve Together Dedicated Server/v2mods"
cd "$HOME"
}

# 主程序
function Main()
{
if [ "$os" == "Arch" ];then
    start32="(暂未启用)"
else
    start32=""
fi

while :
do
echo "============================================"
echo "模组状态：$NeedsUpdate1 $NeedsDownload1"
echo "============================================"
echo "$os Linux 环境 | 作者：小草哥哥 | Ver: 4.2"
echo "--------------------------------------------"
echo "饥荒联机群：246489642"
echo "============================================"
echo "[1]更新服务器 	   [2]关闭服务器 "
echo "[3]启动服务器x64   [4]启动服务器x32 ${start32}"
echo "[5]查看版本   	   [6]管理模组"
echo "[7]管理特殊名单	   [8]管理存档"
echo "[0]退出脚本"
echo "============================================"
echo 
read main1
    case $main1 in
        1)Gameupdate
        break;;
        2)CloseServer
        break;;
        3)Startserver64
        break;;
        4)Startserver32
        break;;
        5)ShowVerSion
        break;;
        6)Modmanage
        break;;
        7)Listmanage
        break;;
        8)Filemanage
        break;;
        0)ExitShell
        break;;
        esac
        clear
    done
}


# 程序入口
echo "欢迎使用"
Prepare
echo "准备完毕"
UpNoteMod
Main
