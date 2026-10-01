# sts2core — core-only

这个分支是纯模拟内核。不要把真实性验证、搜索策略、真实游戏接线或数据抓取重新塞回 core。

## 允许的职责

- `State` / RNG / 固定容量数据结构
- 游戏规则的纯状态转移
- 伤害、格挡、状态、卡牌、敌人、遗物等规则数据
- 合法动作枚举
- 纯内存、确定性的战斗初始化与推进

## 不允许的职责

以下内容必须放在外部 crate/module：

- replay / trace / observation 同步与 diff
- JSON / HTTP / MCP
- solver / planner / rollout / policy
- 构筑、路线、整幕评估
- 数据 dump / 反编译 / wiki 抓取
- 真实游戏正确性判定和基线报告

依赖方向只能是：

```text
external consumers -> sts2core
```

不能反向依赖。

## 核心不变量

1. `State` 保持固定大小、POD + `Copy`；不要引入 `Vec` / `HashMap` / `Box` / 引用。
2. 所有伤害走 `damage.rs`，不要在其他位置散写 HP 扣减规则。
3. 卡牌 / 敌人 / 能力以数据表为主；缺原语时扩 `Op` / `EOp`，不要按卡名堆特判。
4. RNG 保存在状态内；同状态 + 同动作必须可重复。
5. `step` 是全函数；非法动作返回原状态。
6. core 不做文件 I/O、网络 I/O、序列化协议或真实性验证。

## 提交前检查

```bash
cargo build --release --lib
cargo test --release --tests
```
