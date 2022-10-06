#!/bin/bash

cd `dirname $0`
url=$DATABASE_URL

proto="$(echo $url | sed -e 's,^\(.*://\).*,\1,g')"
url="$(echo ${url/$proto/})"
user_pass="$(echo $url | cut -d@ -f1)"
host_port="$(echo ${url/$user_pass@/} | cut -d/ -f1)"
db_name="$(echo ${url/$user_pass@/} | cut -d/ -f2)"

user="$(echo ${user_pass} | cut -d: -f1)"
pass="$(echo ${user_pass/$user:/})"

host="$(echo ${host_port} | cut -d: -f1)"
port="$(echo ${host_port/$host:/})"

MYSQL_PWD=$pass mysqldef --user=$user --host=$host --port=$port --file=./schema/1_init.sql $db_name
