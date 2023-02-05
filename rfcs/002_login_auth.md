- Feature Name: auth login
- Start Date: 2021-02-06

# Summary

[summary]: #summary

- 用户登录

# Motivation

[motivation]: #motivation

web 端, 实用的登录. 应该包括以下几个方面:

1. 账号密码登录
2. 手机号登录
3. 邮箱登录
4. 登录日志

# Guide-level explanation

[guide-level-explanation]: #guide-level-explanation

- 用户登录支持账号密码登录, 手机号登录, 邮箱登录
- 登录日志
- 用户密码加密传输&存储 -- 什么加密方式?
- 前端输入框的屏蔽 Sql 注入
- 登录失败次数限制 - 5 次
- 一个用户同时只能在一个地方(一台电脑)登录
- 认证方式 JWT
- 考虑 redis 缓存,用户信息.
- [api 风格](https://restfulapi.cn/)

# Reference-level explanation

[reference-level-explanation]: #reference-level-explanation

## API

### 登录接口(/api/v1/login1)

第一阶段只支持账号密码登录

```proto3

enum LoginType {
  Account = 1;
  Phone = 2;
  Email = 3;
}

enum LoginError {
  // 验证码错误
  CaptchaError = 1;
  // 用户名或者密码错误
  UsernameOrPasswordError = 2;
  UserDisabled = 3; // 用户被禁用

  // 登录失败次数超过限制
  LoginFailedLimit = 4;
  // 账户被锁定
  UserLocked = 5;

}

message LoginRequest {
  string username = 1;
  string password = 2;
  string captcha = 3;
}

message LoginResponse {
  string token = 1;
}
```

### 退出接口(/api/v1/logout)

```proto3
message LogoutResponse {
  bool success = 1;
}
```

## 数据库 schema

N/A

# Drawbacks

[drawbacks]: #drawbacks

没有支持,第三方授权登录
一般企业系统后台管理系统都不支持第三方授权登录, 但是对于一些 SaaS 产品, 有必要支持第三方授权登录

# Rationale and alternatives

[rationale-and-alternatives]: #rationale-and-alternatives

cookie 与 jwt 的区别
不用 cookie, 用 jwt 的好处是什么?

jwt 优点:
可以方便的扩展为单点登录系统

# Prior art

[prior-art]: #prior-art

N/A

# Unresolved questions

[unresolved-questions]: #unresolved-questions

第三方登录

# Future possibilities

[future-possibilities]: #future-possibilities

登录功能独立成一个服务, 形成公司内部的单点登录系统

或者方便的与公司内部的单点登录系统对接, 怎么对接

```

```
