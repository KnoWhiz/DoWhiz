# Google Workspace CLI 对 DoWhiz 的启示与架构优化建议

- 日期: 2026-03-05
- 面向对象: DoWhiz 产品/架构团队
- 背景: 基于对“3/2 Google Workspace CLI”相关公开信息的检索，以及对当前 `DoWhiz_service` 实现的代码级对照。

## 1. 结论摘要

1. 对 DoWhiz 有明确启示: 重点不在“再做一个 CLI”，而在“动态能力层 + 事件驱动接入 + 统一工具协议”。
2. 你们当前架构已经有较好基础: 多通道 gateway、queue、worker、Google Workspace 适配器、push+poll 混合链路都已具备。
3. 主要短板在扩展效率和一致性: Docs/Sheets/Slides 仍是三套独立 CLI 与参数处理，导致新增能力成本偏高。

## 2. 外部信号（对产品方向的意义）

> 说明: 这里强调“方向启示”而非单一公告本身的市场包装。

1. 生态趋势是把 Workspace 操作做成“可编排工具能力”，而不是单点脚本。
2. 能力描述趋向动态化（基于 API 描述/Discovery），以减少手写命令和参数分叉。
3. 交互范式趋向 agent/mcp 友好（结构化输入输出、JSON-first、可组合调用）。

对 DoWhiz 的直接含义:

1. 你们应优先建设“统一能力层”，让电子员工调用稳定 schema，而不是继续按应用逐个堆 CLI。
2. 要把“接入用户原工作流”理解为“事件接入 + 能力编排 + 可观测治理”，不是只做入站转发。

## 3. 与当前 DoWhiz 架构的对照

当前已具备的优势:

1. 多通道网关和任务执行链路清晰（gateway -> ingestion queue -> worker）。
2. Google Workspace 已支持 poller，并有 push webhook 快速触发路径。
3. processed tracking + Mongo 唯一索引的幂等思路是正确的。

代码锚点:

1. `DoWhiz_service/scheduler_module/src/bin/inbound_gateway/google_workspace.rs`
2. `DoWhiz_service/scheduler_module/src/google_workspace_poller.rs`
3. `DoWhiz_service/scheduler_module/src/google_drive_changes.rs`
4. `DoWhiz_service/scheduler_module/src/bin/inbound_gateway/google_drive_webhook.rs`

主要可优化点:

1. 三套 Workspace CLI 分裂（命令体系、参数解析、输出格式复用有限），相关文件:
`DoWhiz_service/scheduler_module/src/bin/google_docs_cli.rs`、`DoWhiz_service/scheduler_module/src/bin/google_sheets_cli.rs`、`DoWhiz_service/scheduler_module/src/bin/google_slides_cli.rs`。
2. push 失败处理偏保守: 某些 watch 注册失败后当前策略是“本次进程生命周期内不重试”，恢复弹性不足。
3. route 缺失时直接跳过，自动发现/引导绑定能力还不够产品化。

## 4. 优化建议（按优先级）

## P0（建议立即做）

1. 建立统一的 `workspace_tool_router`（schema-first）。实施要点: 抽象统一命令模型 `list/read/comment/reply/edit/batch_update/search`；每个 provider（docs/sheets/slides）只实现能力映射与权限声明；对上游 agent 暴露稳定 JSON contract（输入/输出/错误码）。

2. 建立统一鉴权与 token lifecycle。实施要点: 将当前 CLI 内分散的 `get_auth()` 聚合为单点组件；增加 token 状态 telemetry（refresh 成功率、过期重试、失败原因聚类）。

3. 事件优先调度策略。实施要点: 保留 polling 兜底；push 通道失败从“永久放弃到重启”改为“指数退避 + 冷却重试 + 熔断恢复”。

## P1（建议下一阶段）

1. 路由自动化。实施要点: 对未命中文件 route 生成“待绑定事件”，支持控制台一键确认绑定；加入租户级默认路由策略与告警阈值。

2. 可观测性升级。实施要点: 增加按 channel/provider 的端到端指标（detection latency、enqueue success、dedup hit、reply SLA）；打通 trace_id 到任务执行日志，便于从 webhook 一路追踪到 outbound。

3. 能力注册中心。实施要点: 给每个工具能力记录 `capability_id/version/scope/risk_level`；支持灰度发布和按员工白名单启用。

## P2（中期）

1. 从“Workspace 适配”扩展到“Workflow 适配”。实施要点: 把 Google Workspace、Notion、Slack、GitHub 统一到同一能力层抽象；引入跨系统复合动作（例如: “读表格 -> 生成文档 -> 发邮件 -> 回写状态”）。

2. 提升策略层智能。实施要点: 基于任务类型自动选择 push/poll 组合；基于失败模式自动切换降级路径（例如仅读取、不写入）。

## 5. 建议落地节奏（4 周）

1. 第 1 周: 设计并落地 `workspace_tool_router` 最小可用版本（先覆盖 list/read/comment/reply）。
2. 第 2 周: 将 docs/sheets/slides CLI 切到统一路由层，保留旧命令兼容壳。
3. 第 3 周: 实现 watch 重试策略与指标埋点，完成基础 dashboard。
4. 第 4 周: route 自动化与运维告警联动，完成一次真实流量灰度。

## 6. 验收标准（建议）

1. 新增一个 Workspace 动作时，代码改动集中在 provider mapping，避免复制命令解析逻辑。
2. Docs/Sheets/Slides 三条链路的错误码与输出结构一致。
3. push 通道异常后可自动恢复，无需人工重启服务。
4. 未路由事件可被运营侧可视化处理，不再“静默跳过”。

## 7. 风险与注意事项

1. 统一层改造会触达现有 CLI 入口，必须保留兼容层避免影响现网任务脚本。
2. 动态能力发现会带来权限边界风险，必须做 scope allowlist 与审计日志。
3. 事件优先架构要防重复触发，幂等键设计需跨 webhook/polling 统一。

## 8. 下一步可执行动作

1. 先在 `scheduler_module` 内新增 `workspace_tool_router` 模块，不改现有外部命令名。
2. 选 `google-sheets` 作为首个迁移样板，验证统一 contract。
3. 补充针对性测试。单测覆盖 schema 校验、参数解析、错误映射、幂等键生成；E2E 覆盖 push 命中、polling 兜底、watch 失败自动重试、route 缺失告警。
