-- Add migration script here

-- -- 创建用户角色表
-- CREATE TABLE IF NOT EXISTS user_roles (
--     id INTEGER PRIMARY KEY AUTOINCREMENT, -- 用户角色ID
--     name TEXT NOT NULL, -- 用户角色名
--     description TEXT NOT NULL, -- 用户角色描述
--     permissions TEXT NOT NULL, -- 权限
--     is_active INTEGER NOT NULL DEFAULT 1, -- 0: 禁用, 1: 启用
--     created_at DATETIME DEFAULT CURRENT_TIMESTAMP -- 创建时间
-- );

-- -- 创建权限表
-- CREATE TABLE IF NOT EXISTS permissions (
--     id INTEGER PRIMARY KEY AUTOINCREMENT, -- 权限ID
--     name TEXT NOT NULL, -- 权限名
--     description TEXT NOT NULL, -- 权限描述
--     is_active INTEGER NOT NULL DEFAULT 1, -- 0: 禁用, 1: 启用
--     created_at DATETIME DEFAULT CURRENT_TIMESTAMP -- 创建时间
-- );

-- 创建用户表
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY NOT NULL, -- 用户ID
    username TEXT NOT NULL UNIQUE, -- 用户名
    email TEXT NOT NULL UNIQUE, -- 邮箱
    password TEXT NOT NULL, -- 密码
    salt TEXT NOT NULL, -- 盐
    -- role_id INTEGER NOT NULL REFERENCES user_roles(id), -- 角色ID
    role TEXT NOT NULL CHECK(role IN ('admin', 'user', 'guest')) DEFAULT 'user', -- 'admin'、'user'、'guest' 角色
    avatar TEXT, -- 头像
    gender INTEGER NOT NULL DEFAULT 2, -- 0: 女, 1: 男, 2: 未知
    is_active BOOLEAN NOT NULL DEFAULT 1, -- 0: 禁用, 1: 启用
    is_online BOOLEAN NOT NULL DEFAULT 0, -- 0: 离线, 1: 在线
    last_ip TEXT, -- 最后登录IP
    last_login DATETIME, -- 最后登录时间
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP -- 创建时间
);

-- 创建验证码表
CREATE TABLE IF NOT EXISTS verification_codes (
    id INTEGER PRIMARY KEY AUTOINCREMENT, -- 验证码ID
    email TEXT NOT NULL UNIQUE, -- 邮箱
    code TEXT NOT NULL, -- 验证码
    expires_at DATETIME NOT NULL, -- 过期时间
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP -- 创建时间
);

-- 创建程序表
CREATE TABLE IF NOT EXISTS programs (
    id TEXT PRIMARY KEY NOT NULL, -- 程序ID
    user_id TEXT NOT NULL REFERENCES users(id), -- 用户ID
    name TEXT NOT NULL, -- 程序名
    description TEXT, -- 程序描述
    source_code TEXT NOT NULL, -- 程序内容
    compiled_code TEXT, -- 完成代码
    status TEXT NOT NULL, -- 状态
    metadata TEXT, -- 元数据
    is_active BOOLEAN NOT NULL DEFAULT 1, -- 0: 禁用, 1: 启用
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP, -- 创建时间
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP -- 更新时间
);

-- 创建文档表
CREATE TABLE IF NOT EXISTS documents (
    id TEXT PRIMARY KEY NOT NULL, -- 文档ID
    title TEXT NOT NULL, -- 文档标题
    content TEXT NOT NULL, -- 文档内容
    user_id TEXT NOT NULL REFERENCES users(id), -- 用户ID
    doc_type TEXT NOT NULL, -- 'markdown', 'html', 'json', 'text', 'code'
    is_active BOOLEAN NOT NULL DEFAULT 1, -- 0: 禁用, 1: 启用
    metadata TEXT, -- 元数据
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP, -- 创建时间
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP -- 更新时间
);

-- 创建文档权限表
CREATE TABLE IF NOT EXISTS document_permissions (
    id TEXT PRIMARY KEY NOT NULL, -- 文档权限ID
    document_id TEXT NOT NULL REFERENCES documents(id), -- 文档ID
    user_id TEXT NOT NULL REFERENCES users(id), -- 用户ID
    parameters TEXT, -- 参数
    permission_type TEXT NOT NULL, -- 'read', 'write', 'admin'
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP, -- 创建时间
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP -- 更新时间
);

-- 创建任务表
CREATE TABLE IF NOT EXISTS tasks (
    id TEXT PRIMARY KEY NOT NULL, -- 任务ID
    name VARCHAR(255) NOT NULL, -- 任务名
    description TEXT, -- 任务描述
    task_type VARCHAR(255) NOT NULL, -- 任务类型
    cron_expression VARCHAR(100) NOT NULL, -- 定时表达式
    one_time BOOLEAN NOT NULL DEFAULT 0, -- 0: 循环, 1: 一次性
    priority TEXT NOT NULL CHECK(priority IN ('low', 'medium', 'high', 'critical')) DEFAULT 'medium', -- 优先级
    timeout_seconds INTEGER, -- 超时时间
    max_retries INTEGER NOT NULL DEFAULT 0, -- 最大重试次数
    retry_delay_seconds INTEGER NOT NULL DEFAULT 60, -- 重试延迟时间
    parameters TEXT, -- 参数
    status TEXT NOT NULL CHECK(status IN ('pending', 'scheduled', 'running', 'completed', 'failed', 'paused', 'canceled')) DEFAULT 'pending', -- 状态
    is_active BOOLEAN NOT NULL DEFAULT 1, -- 0: 禁用, 1: 启用
    created_by TEXT NOT NULL REFERENCES users(id), -- 创建者ID
    next_run_at DATETIME, -- 下次运行时间
    last_run_at DATETIME -- 上次运行时间
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP, -- 创建时间
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP -- 更新时间
);

-- 创建任务依赖表
CREATE TABLE IF NOT EXISTS task_dependencies (
    id TEXT PRIMARY KEY NOT NULL, -- 任务依赖ID
    dependent_task_id TEXT NOT NULL REFERENCES tasks(id), -- 依赖任务ID
    prerequisite_task_id TEXT NOT NULL REFERENCES tasks(id), -- 前置任务ID
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP, -- 创建时间
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP -- 更新时间
);

-- 创建任务执行表
CREATE TABLE IF NOT EXISTS task_executions (
    id TEXT PRIMARY KEY NOT NULL, -- 任务执行ID
    task_id TEXT NOT NULL REFERENCES tasks(id), -- 任务ID
    status TEXT NOT NULL CHECK(status IN ('pending', 'scheduled','running', 'completed', 'failed', 'paused', 'canceled')) DEFAULT 'pending', -- 状态
    started_at DATETIME, -- 开始时间
    completed_at DATETIME, -- 完成时间
    duration_ms INTEGER, -- 持续时间
    error_message TEXT, -- 错误消息
    node_id VARCHAR(255), -- 节点ID
    attempt_number INTEGER NOT NULL DEFAULT 1, -- 尝试次数
    parameters TEXT, -- 参数
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP, -- 创建时间
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP -- 更新时间
);

-- 创建任务日志表
CREATE TABLE IF NOT EXISTS task_logs (
    id TEXT PRIMARY KEY NOT NULL, -- 任务日志ID
    task_id TEXT NOT NULL REFERENCES tasks(id), -- 任务ID
    user_id TEXT NOT NULL REFERENCES users(id), -- 用户ID
    action VARCHAR(50) NOT NULL, -- 操作
    details TEXT, -- 详情
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP -- 创建时间
);
-- 创建日志表
CREATE TABLE IF NOT EXISTS logs (
    id TEXT PRIMARY KEY NOT NULL, -- 日志ID
    level TEXT NOT NULL CHECK(level IN ('debug', 'info', 'warning', 'error', 'critical', 'event')) DEFAULT 'info', -- 日志级别
    message TEXT NOT NULL, -- 日志消息
    metadata TEXT, -- 元数据
    source TEXT, -- 来源
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP -- 创建时间
);