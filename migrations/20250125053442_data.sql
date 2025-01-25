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
    username TEXT NOT NULL, -- 用户名
    email TEXT NOT NULL UNIQUE, -- 邮箱
    password TEXT NOT NULL, -- 密码
    salt TEXT NOT NULL, -- 盐
    -- role_id INTEGER NOT NULL REFERENCES user_roles(id), -- 角色ID
    role TEXT NOT NULL DEFAULT 'user', -- 'admin' or 'user' 角色
    avatar TEXT DEFAULT '', -- 头像
    gender INTEGER NOT NULL DEFAULT 2, -- 0: 女, 1: 男, 2: 未知
    is_active INTEGER NOT NULL DEFAULT 1, -- 0: 禁用, 1: 启用
    is_online INTEGER NOT NULL DEFAULT 0, -- 0: 离线, 1: 在线
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
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
    content TEXT NOT NULL, -- 程序内容
    status TEXT NOT NULL, -- 状态
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP, -- 创建时间
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP -- 更新时间
);

-- 创建文档表
CREATE TABLE IF NOT EXISTS documents (
    id TEXT PRIMARY KEY NOT NULL, -- 文档ID
    title TEXT NOT NULL, -- 文档标题
    content TEXT NOT NULL, -- 文档内容
    owner_id TEXT NOT NULL, -- 用户ID
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP, -- 创建时间
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP -- 更新时间
);

-- 创建文档权限表
CREATE TABLE IF NOT EXISTS document_permissions (
    id TEXT PRIMARY KEY NOT NULL, -- 文档权限ID
    document_id TEXT NOT NULL, -- 文档ID
    user_id TEXT NOT NULL, -- 用户ID
    parameters TEXT, -- 参数
    permission_type TEXT NOT NULL, -- 'read', 'write', 'admin'
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP, -- 创建时间
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP -- 更新时间
);

-- 创建任务表
CREATE TABLE IF NOT EXISTS tasks (
    id TEXT PRIMARY KEY NOT NULL, -- 任务ID
    name TEXT NOT NULL, -- 任务名
    cron_expression TEXT NOT NULL, -- 定时表达式
    task_type TEXT NOT NULL, -- 'document_sync' or 'other'
    parameters TEXT, -- 参数
    is_active INTEGER NOT NULL DEFAULT 1, -- 0: 禁用, 1: 启用
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP, -- 创建时间
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP -- 更新时间
);

-- 创建定时任务表
CREATE TABLE IF NOT EXISTS scheduled_tasks (
    id TEXT PRIMARY KEY NOT NULL, -- 定时任务ID
    name TEXT NOT NULL, -- 任务名
    cron_expression TEXT NOT NULL, -- 定时表达式
    task_type TEXT NOT NULL, -- 任务类型
    parameters TEXT, -- 参数
    is_active INTEGER NOT NULL DEFAULT 1, -- 0: 禁用, 1: 启用
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP, -- 创建时间
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP -- 更新时间
);

-- 创建日志表
CREATE TABLE IF NOT EXISTS logs (
    id TEXT PRIMARY KEY NOT NULL, -- 日志ID
    message TEXT NOT NULL, -- 日志消息
    level TEXT NOT NULL, -- INFO, WARN, ERROR
    metadata TEXT, -- 元数据
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP -- 创建时间
);