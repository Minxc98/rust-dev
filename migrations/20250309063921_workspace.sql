--CREATE TABLE IF NOT EXISTS users (
--    id SERIAL PRIMARY KEY,
--    username VARCHAR(50) NOT NULL UNIQUE,
--    password VARCHAR(255) NOT NULL,
--    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
--);
--更改 password 为 password_hash
ALTER TABLE users RENAME COLUMN password TO password_hash;
--添加email字段
ALTER TABLE users ADD COLUMN email VARCHAR(255) NOT NULL UNIQUE;
--添加workspace
create table if not exists workspaces (
    id serial primary key,
    name varchar(50) not null unique,
    owner_id integer not null,
    created_at timestamp default current_timestamp
);
--insert super user
begin;
INSERT into users (id, username,email, password_hash) VALUES (0,'admin','admin@qq.com' ,'');
insert into workspaces (id,name,owner_id) values (0,'admin',0);
commit;

