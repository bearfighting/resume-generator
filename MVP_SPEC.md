# Local-first Resume Studio — MVP 开发规范

> 版本：0.1（2026-11-02）  
> 状态：供 AI 编码代理执行的初始规范  
> 目标：在不上传任何用户职业信息的前提下，构建可离线使用的桌面简历生成器。

## 1. 项目目标与原则

### 1.1 产品定位

Local-first Resume Studio 是一款无需账户、无需联网、数据仅保存在用户设备上的桌面简历应用。用户维护一份完整的职业档案（Career Profile），从中生成多份面向不同职位的简历草稿（Resume Draft），使用内置模板预览并导出 PDF、可编辑 DOCX 或静态 HTML+CSS。

### 1.2 MVP 完成定义

用户首次启动应用后，无需联网即可：

1. 创建、编辑和保存完整职业档案；
2. 从档案创建多份简历草稿，选择、排序和隐藏内容；
3. 在九个内置模板之间切换，并调整少量样式参数；
4. 预览最终页面，导出可选择文字的 PDF、可编辑 DOCX 和静态 HTML+CSS；
5. 导出和导入完整 JSON 备份；
6. 关闭并重新打开应用后恢复全部数据。

### 1.3 不可妥协的约束

- 默认不建立任何网络连接；不包含用户账户、远程数据库、遥测、崩溃上报、远程字体、CDN、在线渲染。
- 任何职业信息、简历草稿和导出文件不得上传到服务器。
- 核心业务规则与 UI、文件系统、PDF 生成实现隔离。
- 事实层与简历草稿分离：修改草稿不得静默修改职业档案。
- 数据导入、模板解析、渲染和文件写入失败必须返回可处理的错误；不以 `unwrap`/`expect` 处理可预期失败。
- 所有重要模块通过公开接口交互；禁止 UI 直接访问数据库或拼装 PDF。
- 同样的数据、模板、配置和渲染环境应产生语义与布局一致的输出（不要求 PDF 文件字节完全相同）。

## 2. 范围与非目标

### 2.1 MVP 功能

| 功能 | MVP 范围 | 验收依据 |
|---|---|---|
| 职业档案 | 联系方式、职业简介、工作经历、教育、技能、项目、证书、语言、奖项、志愿经历、出版/演讲、专业组织、执照、推荐人、兴趣 | 增删改查、持久化；资料条目可被多个草稿引用 |
| 简历草稿 | 多草稿、章节和条目选择、排序、局部文本覆盖；职业简介可按草稿改写 | 草稿互不影响，源数据不被改写；未选择的章节不留空标题或空白栏 |
| 模板 | 9 个经过视觉打磨的内置模板：Modern Professional、Creative / Design、Academic / Research、Executive / Leadership、Classic / Traditional、Two-Column / Modern、Two-Column / Left Rail、Creative Minimal、Modern Business | 同一草稿切换模板，内容不丢失；层级、留白、对齐和分页经过真实样例检查；至少一个 ATS 友好单栏模板 |
| 样式 | 字体族（受控列表）、字号比例、强调色、紧凑度 | 预览与各导出格式保持对应模板的视觉特征 |
| 预览 | 本地分页预览 | 无内容裁切、明显重叠 |
| 导出 | PDF（文字可选中）、DOCX（可编辑）、静态 HTML+CSS | 本地保存、文本/顺序/链接完整；各格式在目标阅读器中可用 |
| 备份 | JSON 导入/导出，Schema 版本 | 往返恢复数据 |
| 离线 | 所有核心流程无需网络 | 断网端到端测试 |

### 2.2 明确不做

- 模板在线商店、模板下载和更新；
- AI 内容生成、ATS 打分、岗位抓取；
- 云同步、账号、协作、分享链接；
- 通用可视化模板编辑器、任意用户自定义 HTML/CSS 或脚本模板；
- 移动端、插件系统、任意格式导入（除本应用 JSON 备份）。

这些功能只能作为后续扩展点，不得为了“未来兼容”提前实现复杂基础设施。

## 3. 技术选型与工程约束

### 3.1 推荐实现

- 桌面：Tauri 2 + TypeScript + React + Vite（UI 与本地 HTML 预览）。
- 核心：Rust workspace，业务模型、校验、草稿生成、模板编译为独立 crate。
- 持久化：SQLite；通过 Rust Repository 接口访问。
- PDF：优先使用与预览一致的本地 Web 排版/打印管线；在实现前做一项可重复的 PDF 导出可行性 Spike。**不要假设 Tauri WebView 在所有目标系统上都具有相同的无头 PDF 打印 API。**
- 导出格式：由共享的语义/视觉文档驱动 PDF、静态 HTML+CSS 和 DOCX 渲染器。HTML 导出不加载外部资源；DOCX 优先保证结构、可编辑性和内容完整，不承诺与像素排版完全一致。
- MVP 首个支持平台：选择一个桌面平台作为正式验收目标（建议 Linux）；其他平台仅在核心功能稳定后验证。

> 技术栈是初始建议，不是绝对要求。若 AI 发现 PDF 管线在目标平台无法可靠实现，必须先提交简短 ADR，说明替代方案、依赖、离线能力、预览一致性和测试策略，再修改实现。

模板视觉方向以专业阅读体验为准：建立清晰的信息层级、统一的对齐和间距，强调色克制；至少保留一个 ATS 友好单栏样式。参考公开的职业发展指南和样例以提炼原则，不照搬受版权保护的模板。可从 [MIT Career Advising 的简历格式建议](https://capd.mit.edu/resources/career-toolkit-crafting-an-effective-resume/) 和 [Harvard 的简历指南](https://careerservices.fas.harvard.edu/resources/create-a-strong-resume/) 开始调研。

### 3.2 PDF 技术决策门槛（必须先做）

Spike 应用固定的样例简历测试：A4/Letter、字体回退、列表、标题不孤立、跨页、长 URL、Unicode、两页以上内容。比较本地预览与 PDF 的文字顺序、分页、边距和溢出。选用可离线部署、可测试、许可合适的方案；不满足要求则不得宣称 PDF 完成。

## 4. 领域模型

### 4.1 核心概念

- **CareerProfile**：真实职业事实的唯一来源，包含带稳定 ID 的条目。
- **ResumeDraft**：针对一个目标的内容选择、顺序和文本覆盖，引用事实 ID。
- **ResumeDocument**：由草稿编译出的语义文档快照；不包含 UI 控件或 PDF 指令。
- **TemplateDefinition**：版本化的设计 Token、组件规则、区域组合和布局提示。
- **VisualDocument**：模板编译后、与输出格式无关的视觉节点树。
- **RenderResult**：导出结果及诊断信息。

### 4.2 参考数据契约（可细化，不可随意打破语义）

```ts
interface CareerProfile {
  id: string;
  schemaVersion: number;
  personal: PersonalInfo;
  experiences: Experience[];
  education: Education[];
  projects: Project[];
  skills: Skill[];
  certifications: Certification[];
  languages: Language[];
  awards: Award[];
  volunteering: VolunteerExperience[];
  publications: Publication[];
  memberships: Membership[];
  licenses: License[];
  references: ProfessionalReference[];
  updatedAt: string;
}

interface PersonalInfo {
  fullName: string;
  headline?: string;
  email?: string;
  phone?: string;
  location?: string;
  website?: string;
  linkedin?: string;
  github?: string;
  professionalSummary?: string;
}

interface Language { id: string; name: string; proficiency?: string; certification?: string }
interface Award { id: string; name: string; issuer?: string; period?: string; summary: string; url?: string }
interface VolunteerExperience { id: string; role: string; organization: string; period?: string; summary: string }
interface Publication { id: string; title: string; venue?: string; period?: string; url?: string; summary: string }
interface Membership { id: string; organization: string; role?: string; period?: string }
interface License { id: string; name: string; issuer?: string; period?: string; credentialUrl?: string }
interface ProfessionalReference { id: string; name: string; role?: string; organization?: string; email?: string; phone?: string }
interface Interest { id: string; name: string }

interface ResumeDraft {
  id: string;
  profileId: string;
  title: string;
  targetRole?: string;
  summaryOverride?: string;
  includeSummary: boolean;
  sections: DraftSection[];
  templateId: string;
  templateVersion: string;
  styleOverrides: StyleOverrides;
  updatedAt: string;
}

interface DraftEntry {
  id: string;
  sourceId: string;
  included: boolean;
  order: number;
  textOverrides?: Record<string, string>;
}

interface ResumeDocument {
  schemaVersion: number;
  sections: SemanticSection[];
}

interface RenderDiagnostic {
  severity: 'info' | 'warning' | 'error';
  code: string;
  nodeId?: string;
  message: string;
}
```

CareerProfile 是尽可能完整的本地职业信息库。PersonalInfo 支持电话、地区、个人网站、LinkedIn、GitHub 等可选联系方式，并保存通用职业简介。语言、奖项、志愿经历、出版物/演讲、专业组织、执照和推荐人均为稳定 ID 的独立条目。推荐人资料属于敏感的第三方个人信息，默认不加入任何草稿；其他新增条目只在草稿引用且勾选后才会显示。Summary 默认使用档案中的通用简介，草稿可覆盖或关闭，不修改档案。

上述 TypeScript 是语言中立的接口示例。Rust 内核应定义等价模型并通过受控 DTO 与前端通信。具体字段与 enum 由实现阶段确定，必须提供 Schema、序列化测试和迁移策略。

### 4.3 不变量

1. 所有可引用条目具有稳定、唯一 ID；引用不存在的 sourceId 时必须报告错误。
2. 草稿可以覆盖文案但不能覆盖事实层数据；删除源条目时必须提示受影响草稿，不能产生无声悬挂引用。
3. 简历编译是纯函数：`compile(profile, draft) -> Result<ResumeDocument, CompileError>`。
4. 模板编译是纯函数：`applyTemplate(document, template, overrides) -> Result<VisualDocument, TemplateError>`。
5. 文件保存与导出通过应用层协调；采用临时文件 + 原子替换（在目标文件系统支持时），失败不能破坏原数据。
6. 导入备份先校验版本、结构、引用和冲突策略，成功后再提交；失败时本地现有数据保持不变。

## 5. 模块与依赖方向

```text
apps/desktop             # Tauri + React UI
crates/domain            # Profile、Draft、语义文档、验证
crates/resume-engine     # 事实 -> 草稿 -> 语义文档
crates/document-core     # 通用视觉节点、Token、约束
crates/template-engine   # 模板解析、绑定、编译、诊断
crates/storage-sqlite    # Repository 实现、迁移
crates/export-pdf        # PDF 后端（依 Spike 决策）
crates/export-html       # 本地 HTML+CSS bundle
crates/export-docx       # 可编辑 DOCX 后端
assets/templates         # 8 个内置的声明式模板
fixtures/                # 脱敏的样例数据、预期结果
```

依赖原则：`desktop -> application/services -> domain/engines -> ports`；基础设施实现依赖抽象，业务内核不依赖 Tauri、React、SQLite、网络或 PDF SDK。`template-engine` 不得依赖 `resume-engine` 或了解 Experience、Education 等业务实体。

### 5.1 应用服务接口

```ts
interface ResumeService {
  getProfile(): Promise<CareerProfile>;
  saveProfile(profile: CareerProfile): Promise<void>;
  listDrafts(): Promise<ResumeDraft[]>;
  saveDraft(draft: ResumeDraft): Promise<void>;
  compileDraft(draftId: string): Promise<ResumeDocument>;
  preview(draftId: string): Promise<PreviewResult>;
  exportPdf(draftId: string, path: string): Promise<RenderResult>;
  exportHtmlBundle(draftId: string, path: string): Promise<RenderResult>;
  exportDocx(draftId: string, path: string): Promise<RenderResult>;
  exportBackup(path: string): Promise<void>;
  importBackup(path: string, strategy: 'replace' | 'cancel'): Promise<void>;
}
```

错误通过明确的错误码和上下文返回给 UI；取消、权限不足、磁盘空间不足、数据损坏、模板不兼容、导出失败均是正常业务分支。

## 6. 模板系统 MVP

### 6.1 设计目标

模板是**受限、可序列化、可校验的声明式数据**，不是可执行脚本。定义以下层次：

- Design Tokens：颜色、字体、字号、间距、边框；内部长度统一采用 pt。
- Visual Primitives：`Text`、`Paragraph`、`Stack`、`Row`、`Divider`、`List`。
- Composition：按语义 Section 绑定内容，指定顺序与局部布局。
- Layout Hints：`keepWithNext`、`keepTogether`、可拆分提示。

模板库包含九种用途和视觉方向；保留一个明确推荐的 ATS 友好单栏模板。Two-Column / Modern 和 Two-Column / Left Rail 是可选视觉模板，不是默认方向。照片、图表和图标不作为简历内容的唯一载体。模板必须适配缺失或被用户隐藏的章节：编译器省略空章节；布局器不得留下空栏、空标题、孤立分隔线或仅用于填充空间的占位区。两栏模板在一侧无内容时退化为协调的单栏排版。

### 6.2 模板格式与安全

```json
{
  "schemaVersion": 1,
  "id": "classic",
  "version": "1.0.0",
  "tokens": {
    "fontFamily": "Liberation Sans",
    "bodySizePt": 10,
    "accentColor": "#23354D",
    "sectionGapPt": 10
  },
  "layout": {
    "pageSize": "A4",
    "marginPt": 36,
    "sectionLayout": "single-column"
  }
}
```

该 JSON 仅用于展示最小配置，正式模板 Schema 还应包含组件规则与绑定关系。所有模板字段必须有确定的默认值、版本及校验规则。禁止动态代码执行、任意外部 URL、远程字体、未经限制的文件路径访问。

### 6.3 后端能力与诊断

PDF、静态 HTML+CSS 和可编辑 DOCX 都是 MVP 导出目标。PDF 与 HTML 应尽可能共享 HTML 结构和 CSS；DOCX 使用独立渲染器映射共享的语义/视觉结构，不要求像素级分页一致。模板编译失败、字体不可用、内容无法映射或无法满足分页约束时返回可见诊断；不得静默丢失内容。

## 7. 用户交互与数据流

### 7.1 主要页面

1. **Profile**：结构化表单管理个人信息与经历；支持新增、编辑、删除。
2. **Resumes**：创建草稿、选择包含的条目、排序、编辑仅属于草稿的描述。
3. **Editor**：左侧内容与样式控制，右侧实时页面预览；小屏可切换面板。
4. **Settings / Backup**：本地导入导出、数据路径说明、隐私说明。

### 7.2 数据流

```text
Local SQLite
   -> CareerProfile + ResumeDraft
   -> Resume Compiler (pure)
   -> Semantic Document
   -> Template Compiler (pure)
   -> Visual Document
   -> Local Preview / PDF, HTML+CSS, or DOCX Renderer
   -> Local File
```

预览不得把个人数据发给外部站点；应用 CSP 禁止不必要的外部连接。输入保存采用明确的草稿/提交语义或可靠的自动保存策略，保证切换模板、关闭窗口不会丢失已确认的修改。

## 8. AI 自主开发工作协议

### 8.1 代理角色

AI 负责具体实现、测试、调试、文档更新与阶段交付；人类负责产品目标、重要边界与验收。AI 可以在既定模块内部自主选择算法和组织代码，但不得静默修改产品原则、跨模块契约或范围。

### 8.2 每个任务的执行流程

1. **读取权威文档**：本文件、当前阶段计划、相关模块公开接口。旧草案仅供参考，不得覆盖当前规范。
2. **界定变更**：明确输入、输出、涉及模块、错误分支、验收用例；如需要修改公共契约，先提出变更建议。
3. **实现最小闭环**：先打通真实数据路径，不以 Mock 或硬编码假装完成业务功能。
4. **自行验证**：运行格式化、静态检查、单元测试、集成测试和相关 UI 测试。
5. **交付报告**：说明完成项、测试命令及结果、未解决问题、接口变化、下一步建议。

### 8.3 AI 必须遵守

- 禁止为“尽快跑通”绕过 domain/engine 直接在 UI 中实现业务规则。
- 禁止在业务路径使用无解释的 `unwrap`、`expect`、`panic!`，或吞掉错误。
- 禁止以编译通过替代功能验收；禁止将未运行的测试写为通过。
- 禁止在未获批准时新增网络依赖、遥测、账号、外部 AI API。
- 禁止为了一个局部需求大规模重构其他模块；确需重构先说明原因、影响和迁移方案。
- 禁止以过时文档作为权威来源；冲突时报告冲突，不自行拼接多个不兼容方案。
- 依赖与许可必须记录；优先成熟、可离线分发的组件。

### 8.4 不确定性处理

- **模块内部细节**：AI 可自主决策并测试。
- **公共接口变化**：先记录提案、兼容性影响及替代方案。
- **产品范围/隐私/持久化语义变化**：必须暂停并请求人类决策。
- **工具链阻塞**：给出复现步骤和候选解决方案，不以删除测试或禁用校验作为解决办法。

### 8.5 单任务交付模板

```md
## Task
目标、范围与不做的内容

## Contract
输入、输出、错误、不变量

## Acceptance
- [ ] 正常路径
- [ ] 边界/失败路径
- [ ] 离线验证（如适用）

## Implementation Report
- 修改的模块与公开接口
- 执行的测试命令与结果
- 风险、限制与未完成事项
- 是否需要 ADR / 规范更新
```

## 9. 分阶段开发计划

### Phase 0 — 技术风险验证

- 初始化 workspace、格式化/静态检查/测试命令；
- 验证目标平台的 Tauri 本地文件访问、SQLite、PDF 导出；
- 用固定样例测试分页、Unicode、长文本、可选中文本、离线导出；
- 输出 `ADR-0001-pdf-pipeline.md`，明确选择与已知限制。

**验收门槛**：PDF 可行性得到实测证据；若不满足，先更换方案，不继续堆叠 UI。

### Phase 1 — 领域与存储

- CareerProfile / ResumeDraft 类型与校验；
- SQLite schema、迁移、Repository；
- 引用完整性、CRUD、持久化和重启测试；
- JSON 备份的版本化格式与导入原子性。

**验收门槛**：无需 UI 也能完整创建、保存、读取、备份与恢复档案和草稿。

### Phase 2 — Resume Compiler

- 选择、排序、隐藏、文案覆盖；
- `CareerProfile + ResumeDraft -> ResumeDocument`；
- 空数据、删除源条目、无效引用、过长文本等测试。

**验收门槛**：纯函数测试覆盖多份草稿与源数据隔离。

### Phase 3 — Template Compiler

- Token、视觉原语、模板 Schema；
- 模板 Schema、编译、诊断与快照测试；后续 Phase 6 扩充为九种内置设计方向；
- 模板校验、编译、诊断与快照测试。

**验收门槛**：同一文档可生成合法 VisualDocument；无内容丢失。

### Phase 4 — 桌面编辑与预览

- Profile/Resumes/Editor/Settings 页面；
- 编辑、草稿选择、模板切换、实时预览；
- 错误展示、空状态、保存反馈和键盘可用性。

**验收门槛**：非开发者可以不接触 JSON 完成主要操作。

### Phase 5 — PDF 与备份

- 完整 PDF 导出和分页回归；
- JSON 导入导出、损坏文件处理；
- 基础离线/隐私检查；
- README、使用说明、已知限制。

**验收门槛**：通过已实现 PDF 与备份相关的端到端场景。外部发布另由 Phase 9 处理。

### Phase 6 — 专业模板设计

- 基于同一份合成简历比较九种原创模板方向；
- 打磨字体层级、色彩、留白、章节节奏和分页；
- 至少一个 ATS 友好单栏模板；双栏模板作为可选视觉方向。

**验收门槛**：九种模板均保留完整内容且风格清晰；A4/Letter 无裁切或重叠。

### Phase 7 — 静态 HTML+CSS 导出

- 导出包含 HTML、CSS 和本地资源的离线 bundle；
- 不含脚本、CDN 或远程字体；提供 print 样式。

**验收门槛**：浏览器可离线打开、内容/链接完整、打印布局与预览接近。

### Phase 8 — DOCX 导出

- 从共享文档模型生成可编辑 DOCX；
- 映射章节、段落、列表、日期、链接和页面设置。

**验收门槛**：在 LibreOffice 和 Microsoft Word（可用时）验证内容顺序、可编辑性和合理分页。

### Phase 9 — 发布准备（延期）

- 对打包应用做离线端到端检查；确认应用标识、许可证/发布者信息；
- 决定签名、发布渠道和发行包支持范围。

## 10. 端到端验收场景

1. **首次启动**：断网启动，创建档案、添加两段工作经历、一项教育和技能，保存并重启，数据一致。
2. **多份简历**：同一 Profile 创建 A/B 草稿，A 隐藏一项经历、B 保留；修改 A 的文案，B 和 Profile 不变。
3. **模板切换**：同一草稿切换九种模板，语义内容与顺序不丢失，预览无明显溢出。
4. **长内容**：使用包含长 URL、长职位名、Unicode 和超过两页经历的 Fixture，PDF 无裁切、重叠、空白异常或丢失内容。
5. **PDF 导出**：PDF 可在系统阅读器打开，文字可选择/搜索，主要文字与草稿一致。
6. **HTML+CSS 导出**：解压静态 bundle，断网打开，内容/链接完整且打印布局正常。
7. **DOCX 导出**：在支持的 Word 处理器中打开、编辑、保存并重新打开，结构与内容保持完整。
8. **备份恢复**：导出 JSON，清空测试环境并导入，Profile、Draft、模板引用与文本覆盖一致。
9. **损坏备份**：导入非法 JSON、未知版本、重复 ID、悬挂引用，显示明确错误且不破坏现有数据。
10. **离线与隐私**：在阻断网络的环境中完成所有核心操作；检查运行时无非预期外连请求。
9. **失败处理**：模拟不可写路径、磁盘写入失败、模板缺失；应用不崩溃、不静默丢失数据。
10. **重新启动**：保存后关闭并重新启动，草稿选择、内容和配置恢复正确。

## 11. 测试与质量门槛

- Domain/Compiler：单元测试覆盖正常、空值、边界、无效引用和不变量。
- Storage：真实 SQLite 集成测试；迁移、事务、备份导入失败回滚。
- Template：固定 Fixture 的视觉树快照；内容节点数量与引用完整性检查。
- PDF：文本提取对照、页数、溢出检查和少量人工视觉基线；不要只靠截图像素比较。
- UI：关键用户流程的端到端自动化测试，至少覆盖创建、编辑、切换模板、导出和重启。
- CI：`cargo fmt --check`、`cargo clippy`、`cargo test`、前端 typecheck/lint/test/build；具体命令写入 README 并保持可一键运行。
- 安全：审计网络调用、外部资源、模板解析和文件路径；确认离线发行包包含所需资源。

不得设置没有证据的“100% 测试覆盖率”目标。优先覆盖不可逆数据损坏、引用错误、内容丢失和导出失败等高风险路径。

## 12. 文档治理：避免 AI 反复重构造成规范冲突

仓库内保持少量权威文件：

```text
MVP_SPEC.md               # 产品边界、架构原则、验收标准（本文件）
CURRENT_PLAN.md           # 当前阶段、正在执行任务、阻塞与下一步
ROADMAP.md                # 高层里程碑、依赖顺序与验收目标
CONTRACTS.md              # 当前生效的跨模块接口与数据不变量
ADRs/                     # 仅保存影响较大的已接受技术决策
README.md                 # 运行、测试、构建、发布命令
```

优先级：`MVP_SPEC.md` 的产品/隐私约束 > 已接受 ADR 的特定技术决策 > `CONTRACTS.md` 当前接口 > `CURRENT_PLAN.md` 任务安排 > 代码中的实现细节。若发现文件间冲突，暂停相关变更并提出单一修订方案；不要同时保留多个“最新设计”。

每个阶段结束必须更新 `CURRENT_PLAN.md`，记录已验证功能、尚未验证功能和下一阶段入口。废弃计划移出权威路径，不要求 AI 在每次开发时通读历史讨论。

## 13. 第一条交给 AI 的开发指令

> 请先阅读 `MVP_SPEC.md`，不要立即实现完整产品。首先完成 Phase 0：建立最小 Rust + Tauri 工作区，使用固定样例验证离线 PDF 导出的可行性，重点测试分页、Unicode、字体、可选中文本与预览一致性。提交可运行的最小示例、自动化验证命令和 `ADR-0001-pdf-pipeline.md`。不得引入任何需要上传用户数据的服务。遇到会影响架构或产品范围的选择，请先说明方案与风险，不要擅自扩展实现。完成后更新 `CURRENT_PLAN.md`，列出实测结果和 Phase 1 的进入条件。

## 14. MVP 后的演进方向（非当前任务）

- 模板更新：签名校验、版本固定、声明式模板沙箱、可回滚；更新时不得上传职业数据。
- 移动端：复用领域模型和模板定义，先支持阅读、编辑和 PDF 分享。
- 本地 AI：作为可选适配器，不改变事实层；明确禁止默认上传数据。
- 通用模板引擎：当第二种真实文档类型出现后，再评估从 workspace 独立发布。
