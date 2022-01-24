# yc

http 压测工具

```bash
# 用法
yc -n 100 -c 1 http://localhost:8000

# 参数解释
-n 请求次数
-c 并发数量
-t 压测时长 单位/秒
```

改进

```bash
使用tcp的方式创建大量的并发链接
```
