# naiveQA Medium
Medium为可选模块，使用Medium可以实现文件互传和更好的命令控制功能
## 架构
Medium分为守护进程`Medium`和脚手架进程`mcli`，守护进程常驻，脚手架进程则供外部程序调用

`Medium`的默认通讯端口为localhost:14514，由naiveQA Core配置NAT转发，通讯协议使用WebSocket

在运行时需要区分Host和Guest两个角色，因此守护进程的启动命令可能为`Medium -r Host`或`Medium -r Guest`
## 技术
为考虑Release体积，Medium使用Rust编写，考虑使用[rust-websocket](https://crates.io/crates/websocket)实现WebSocket通讯，自己实现协议控制与文件分段传输
## 通讯
在WebSocket端口上允许进行的基础通讯类型包括`file` `command`两种
### file
#### 用途
用于在宿主机和目标机之间传递文件，任意一方可以发起`put`或`get`请求以读写对方的文件
#### 流程
##### 宿主机->目标机
假设宿主机需要向目标机发送文件`D:\pecmd.ini`存放到`X:\Windows\System32\pecmd.ini`，则在控制流文件中填写以下步骤：
```yaml
-   name: Copy pecmd.ini
    call: medium_file
    args: 
        opt: put
        src: D:\pecmd.ini
        dst: X:\Windows\System32\pecmd.ini
```
##### 目标机->宿主机
假设目标机需要请求宿主机上的`D:\test.txt`文件存放到`/home/cno/1.txt`，则目标机执行命令`mcli file get "D:\test.txt" "/home/cno/1.txt"`，然后等待`mcli`进程退出即可；使用`mcli`进程退出码可判断传输是否成功
### command
#### 用途
用于在宿主机和目标机之间互相执行远程命令（类似于ssh），为了确保安全来自目标机的执行请求会被默认忽略
#### 流程
##### 宿主机->目标机
假设宿主机需要在目标机上执行命令`zypper install git`并判断是否成功，则在控制流文件中填写以下步骤：

```yaml
-   name: Install git
    call: medium_command
    args: 
        run: zypper install git
    rets:
        code: ${{ZYPPER_RET_CODE}}
        string: ${{ZYPPER_RET_STR}}

-   name: Judge install git
    call: logic_if
    args:
        exp: ${{ZYPPER_RET_CODE}}!=0
        steps: 
            -   name: Print console
                call: system_print
                args: 
                    text: ${{ZYPPER_RET_STR}}
            
            -   name: Jump to exit
                call: flow_jump
                args:
                    name: Exit

-   name: Exit
    call: flow_exit
        
```
##### 目标机->宿主机
假设目标机需要在宿主机执行两条命令：`zypper install neofetch`和`neofetch`，首先宿主机的控制文件中必须允许来自目标机的命令接收
```yaml
config:
    allow_target_command: true
```
然后目标机通过执行命令`mcli command "zypper install neofetch;neofetch"`，所有命令运行完成后`mcli`进程退出

>使用转义符`\`可以取消对`;`的转译，例如"mcli command "echo too young\\;>1.txt;echo too simple>>1.txt"

当存在多条命令时，不同命令的回显会使用`===COMMAND_NAME START===` `===COMMAND_NAME END===`包裹，例如：

执行`echo sometimes;echo naive`，得到的回显为：
```
===echo sometimes START===
sometimes
===echo sometimes END===

===echo naive START===
naive
===echo naive END===
```