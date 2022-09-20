# Paste

最近又敲代码上头了，写了个软件...勉强自己能用，后面再优化下吧。

Paste是一个快捷输入工具：

<img src="./assets/ani.gif" alt="ani" style="zoom: 67%;" />

功能有点像espanso，可以快速输入长字符串。运行后按下 `Ctrl` + `Space` 打开快速输入栏。用文本框输入来搜索快捷语句，按 `↑`  和 `↓` 选择语句，按下 `Enter` 输入语句。

在 `C:\Users\<USER_NAME>\AppData\Roaming\paste\search.json` 路径下设置需要的快捷语句。设置完需要手动重启。参考语法：

```json
[
    {"name": "语句名", "content": "语句具体内容"},
    {"name": "phone", "content": "130xxxxxxxx"}
]
```

用户体验可能要骂娘...但最近不想改了...