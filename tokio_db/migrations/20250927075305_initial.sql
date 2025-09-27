-- Add migration script here
create table if not exists messages
(
    id integer primary key not null,
    message text not null
);

insert into messages (id, message) values (1, "hello world 1");
insert into messages (id, message) values (2, "hello world 2");
insert into messages (id, message) values (3, "hello world 3");
