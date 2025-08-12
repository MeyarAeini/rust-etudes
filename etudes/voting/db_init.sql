create table options(id integer primary key autoincrement, name text not null, description text null);
create table users(id integer primary key autoincrement, name text not null);
create table votes(
user_id Integer not null,
option_id integer not null,
ordinal integer not null,
foreign key (user_id) references users(id),
foreign key (option_id) references options(id),
primary key (user_id,option_id)
);


insert into options (name,description) values ('option2','option2 description');
insert into options (name,description) values ('option3','option3 description');
insert into options (name,description) values ('option4','option4 description');
insert into options (name,description) values ('option1','option1 description');

