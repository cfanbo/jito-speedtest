实现测试 `jito Block Engine` 节点网络速度，找出当前当前网络最快的节点。

## 测试网速

测试 Mainnet 网络

```
 $ ./jito-speedtest
🌐 测试 Mainnet 网络节点...
开始测试网络连接速度,请稍候...

🚀 网络速度测试结果
============================================================
#1 🟢 🇺🇸 Salt Lake City - 1447ms
    URL: https://slc.mainnet.block-engine.jito.wtf

#2 🟢 🇺🇸 New York - 1593ms
    URL: https://ny.mainnet.block-engine.jito.wtf

#3 🟢 🌐 Mainnet - 1609ms
    URL: https://mainnet.block-engine.jito.wtf

#4 🟢 🇬🇧 London - 1763ms
    URL: https://london.mainnet.block-engine.jito.wtf

#5 🟢 🇩🇪 Frankfurt - 1913ms
    URL: https://frankfurt.mainnet.block-engine.jito.wtf

#6 🟢 🇳🇱 Amsterdam - 1925ms
    URL: https://amsterdam.mainnet.block-engine.jito.wtf

#7 🟢 🇯🇵 Tokyo - 1945ms
    URL: https://tokyo.mainnet.block-engine.jito.wtf

#8 🟢 🇸🇬 Singapore - 2267ms
    URL: https://singapore.mainnet.block-engine.jito.wtf
```

测试 Testnet 网络

```
$ ./jito-speedtest run -t
🧪 测试 Testnet 网络节点...
开始测试网络连接速度,请稍候...

🚀 网络速度测试结果
============================================================
#1 🟢 🇺🇸 Dallas (Testnet) - 1548ms
    URL: https://dallas.testnet.block-engine.jito.wtf

#2 🟢 🇺🇸 New York (Testnet) - 1817ms
    URL: https://ny.testnet.block-engine.jito.wtf

#3 🔴 🌍 Testnet - 失败
    URL: https://testnet.block-engine.jito.wtf
    错误: error sending request
```

## 软件升级

```
./jito-speedtest update
Checking target-arch... aarch64-apple-darwin
Checking current version... v0.0.1
Checking latest released version... 
✅ 已是最新版本: v0.0.1
```
