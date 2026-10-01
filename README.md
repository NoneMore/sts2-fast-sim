# sts2core

这是从原 `sts2-fast-sim` 收缩出来的**纯快速模拟核心**。

这个分支只负责：

```text
State + Action -> State
```

真实性校验、真实游戏观测、trace 回放、搜索/规划、构筑评估和 MCP 接线不属于本 crate，
应由外部模块依赖本 crate 后完成。

## 核心 API

```rust
use sts2core::{begin_combat, legal_actions, step, Action, State};
```

主要入口：

- `step(State, Action) -> State`
- `legal_actions(&State)`
- `begin_combat(State) -> State`
- `end_turn_with_incoming(...)`
- `end_turn_with_live_incoming(...)`

## 模块边界

```text
src/
├── state.rs    固定大小、Copy 的战斗状态与 RNG
├── damage.rs   统一伤害管线
├── ops.rs      规则操作与内容定义结构
├── content.rs  卡牌 / 敌人 / 能力 / 遗物静态表
├── asc.rs      进阶数值快照
└── step.rs     状态转移与合法动作
```

本仓库核心不包含：

- trace / replay / 真实游戏 observation 适配
- JSON 输入输出协议
- solver / planner / rollout
- 构筑与整幕评估
- MCP / STS2MCP 接线
- 数据抓取、反编译、校验报告
- 真实对局语料

推荐依赖方向：

```text
search / rl / validator / advisor
              |
              v
          sts2core
```

`sts2core` 不反向依赖任何验证器、搜索器或游戏进程。

## 设计约束

- `State` 保持固定大小、POD + `Copy`，快照和回滚接近一次 memcpy。
- 所有伤害统一走 `damage.rs`。
- 卡牌、敌人和能力优先数据化；缺原语时扩 `Op` / `EOp`。
- RNG 属于 `State`；同状态、同动作必须得到同结果。
- `step` 是全函数：非法动作返回原状态，不 panic。
- core 不做文件 I/O、HTTP、JSON、录制或真实性判定。

## 构建

```bash
cargo build --release --lib
cargo test --release --tests
```

CI 只检查 core 边界，不再运行真实游戏验收。

## 数据版本

`content.rs` 和 `asc.rs` 是从原项目继承的规则/数据快照。原项目最后声明适配 STS2
v0.107.1；本 core **不负责证明这些数据仍与真实游戏一致**。后续版本同步应由外部
validator/generator 完成，再把确认后的静态表更新进 core。

## License

MIT。保留原项目的版权与许可声明，见 [LICENSE](LICENSE)。
