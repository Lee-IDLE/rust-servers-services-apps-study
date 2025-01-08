/* 테이블이 이미 존재한다면 테이블을 삭제한다 */
drop table if exists ezy_course_c7 cascade;
drop table if exists ezy_tutor_c7;

/* app 사용자가 존재하면 삭제하고 재생성한다 */
--drop user if exists truuser;
--create user truuser with password 'trupwd';

/* 테이블을 생성한다 */
create table ezy_tutor_c7 (
    tutor_id serial primary key,
    tutor_name varchar(200) not null,
    tutor_pic_url varchar(200) not null,
    tutor_profile varchar(2000) not null
);

create table ezy_course_c7
(
    course_id serial primary key,
    tutor_id INT not null,
    course_name varchar(140) not null,
    course_description varchar(2000),
    course_format varchar(30),
    course_structure varchar(200),
    course_duration varchar(30),
    course_price INT,
    course_language varchar(30),
    course_level varchar(30),
    posted_time TIMESTAMP default now(),
    /* ezy_course_c6의 tutor_id 컬럼을 ezy_tutor_c6의 tutor_id 컬럼의 외부키로 지정한다 */
    CONSTRAINT fk_tutor
        FOREIGN KEY(tutor_id)
        REFERENCES ezy_tutor_c7(tutor_id)
    ON DELETE cascade
);

/* 데이터베이스 사용자에게 새롭게 생성한 테이블의 접근 권한을 부여한다. */
grant all privileges on table ezy_tutor_c7 to truuser;
grant all privileges on table ezy_course_c7 to truuser;
grant all privileges on all sequences in schema public to truuser;

/* 테스트를 위한 시드 데이터를 로드한다 */
insert into ezy_tutor_c7(tutor_id, tutor_name, tutor_pic_url, tutor_profile)
values (1, 'Merlene', 'http://s3.amazone.aws.com/pic1',
'Merlene is an experienced finance professional');

insert into ezy_tutor_c7(tutor_id, tutor_name, tutor_pic_url, tutor_profile)
values (2, 'Frank', 'http://s3.amazon.aws.com/pic2',
'Frank is an expert nuclear engineer');

insert into ezy_course_c7(course_id, tutor_id, course_name, course_level, posted_time)
values (1, 1, 'First course', 'Beginner', '2021-04-12 05:40:00');

insert into ezy_course_c7(course_id, tutor_id, course_name, course_level, posted_time)
values (2, 1, 'Second course', 'ebook', '2021-04-12 05:45:00');