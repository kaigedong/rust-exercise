```sh
# 首先在数据库中插入一些数据，参考db/db.sql

# 然后对数据库进行查询
cd db && cargo run
```

```console
Courses = [Course { id: 1, teacher_id: 1, name: "first course", time: Some(2022-01-17T05:40:00) }]
```

## 允许离线模式构建(要求本地已经安装 db，会将结果保存到一个文件中)

```sh
cargo install sqlx-cli
# enable offline features
# 确保修改.env文件，因为DATABASE_URL环境变量优先于sqlx-data.json
cargo sqlx prepare
```
