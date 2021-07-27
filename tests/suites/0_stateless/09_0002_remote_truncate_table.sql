DROP DATABASE IF EXISTS db1;
CREATE DATABASE db1;
USE db1;

CREATE TABLE IF NOT EXISTS t(a varchar, b varchar);
INSERT INTO t(a,b) VALUES('1', 'v1'),('2','v2');
SELECT * FROM t;
TRUNCATE TABLE t;
SELECT * FROM t;

DROP TABLE t;
TRUNCATE TABLE t; -- {ErrorCode 25}

DROP DATABASE db1;
TRUNCATE TABLE db1.t; -- {ErrorCode 3}