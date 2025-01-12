drop table if exists ezyweb_user;

create table ezyweb_user
(
    username varchar(20) primary key,
    tutor_id INT,
    user_password CHAR(100) not null
);