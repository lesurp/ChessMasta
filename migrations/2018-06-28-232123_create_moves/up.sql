-- Your SQL goes here
create table moves
(
    id integer primary key not null,
    parent int,
    turn int not null,
    name_ varchar(5) not null,
    special_name varchar(16),
    line_description varchar(512),
    foreign key(parent) references moves(id)
)