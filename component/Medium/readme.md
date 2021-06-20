# naiveQA Medium
Medium为可选模块，使用Medium可以实现文件互传和更好的命令控制功能
## 架构
Medium分为守护进程`Medium`和脚手架进程`mcli`，守护进程常驻，脚手架进程则供外部程序调用

`Medium`的默认通讯端口为localhost:14514，由naiveQA Core配置NAT转发，通讯协议使用WebSocket

在运行时需要区分Host和Guest两个角色，因此守护进程的启动命令可能为`Medium -r Host`或`Medium -r Guest`
## 技术
为考虑Release体积，Medium使用Rust编写，考虑使用[rust-websocket](https://crates.io/crates/websocket)实现WebSocket通讯，自己实现协议控制与文件分段传输
## 能力
Medium提供的能力包括`file` `command`两种
### file
#### 用途
用于在宿主机和目标机之间传递文件，任意一方可以发起`put`或`get`请求以读写对方的文件
#### 流程
##### 宿主机->目标机
假设宿主机需要向目标机发送文件`D:\pecmd.ini`存放到`X:\Windows\System32\pecmd.ini`，则在控制流文件中填写以下步骤：
```typescript
QA_Medium.put("D:\\pecmd.ini","X:\\Windows\\System32\\pecmd.ini","Copy pecmd.ini")
```
##### 目标机->宿主机
假设目标机需要请求宿主机上的`D:\test.txt`文件存放到`/home/cno/1.txt`，则目标机执行命令`mcli file get "D:\test.txt" "/home/cno/1.txt"`，然后等待`mcli`进程退出即可；使用`mcli`进程退出码可判断传输是否成功
### command
#### 用途
用于在宿主机和目标机之间互相执行远程命令（类似于ssh），为了确保安全来自目标机的执行请求会被默认忽略
#### 流程
##### 宿主机->目标机
假设宿主机需要在目标机上执行命令`zypper install git`并判断是否成功，则在控制流文件中填写以下步骤：

```typescript
let result =await QA_Medium.command("zypper install git","Install git")
if(result.code!=0){
    QA_Report.print(result.output,"Print console")
    QA_Flow.exit("Exit with git not installed")
}
```
##### 目标机->宿主机
假设目标机需要在宿主机执行两条命令：`zypper install neofetch`和`neofetch`，首先宿主机的控制文件中必须允许来自目标机的命令接收
```yaml
config:
    allow_target_command: true
```
然后目标机通过执行命令`mcli command "zypper install neofetch;neofetch"`，所有命令运行完成后`mcli`进程退出

>使用转义符`\`可以取消对`;`的转译，例如"mcli command "echo too young\\;>1.txt;echo too simple>>1.txt"

当存在多条命令时，不同命令的回显会使用`===START COMMAND_NAME===` `===END COMMAND_NAME===`包裹，例如：

执行`echo sometimes;echo naive`，得到的回显为：
```
===START echo sometimes===
sometimes
===END echo sometimes===

===START echo naive===
naive
===END echo naive===
```

## 通讯
Medium Host与Guest之间的通讯统一使用以下格式的Json：
```jsonc
{
    //一个唯一的任务标识，由发起方生成
    "id":"114514",
    //任务类型
    "task":"command",
    //是请求还是回应 request 0/reply 1
    "direction":0
    //负载，包含任务需要的数据信息
    "payload":{}
}
```
对payload的定义：
### file
```jsonc
{
    //源路径和目标路径
    "src":"",
    "dst":"",
    //分页信息，通常取每页200KB；后续版本分页前会对数据进行压缩
    "current_page":0,
    "total_page":10,
    //页面内容
    "page":""
}
```
### command
```jsonc
{
    //command也可为string数组
    "command":"",
    "options":{
        "pwd":"",
        "env":"",
        "encoding":"",
        "shell":"",
        "timeout":""
    }
}
```