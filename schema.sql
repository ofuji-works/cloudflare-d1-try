-- DDL
CREATE TABLE test_table
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT ,
    post_id    BIGINT,
    short_text VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    sample_id  BIGINT NOT NULL DEFAULT 0 NOT NULL
);
CREATE INDEX index_test_table_on_post_id ON test_table (post_id);
CREATE INDEX index_test_table_on_sample_id ON test_table (sample_id);

-- DML
WITH RECURSIVE
    temp(x) AS (VALUES(1) UNION ALL SELECT x+1 FROM temp WHERE x<1000000)
INSERT INTO test_table (post_id, short_text, sample_id)
SELECT CAST(RANDOM()*1000000 AS BIGINT), SUBSTR(RANDOMBLOB(16), 1, 32), CAST(RANDOM()*1000000 AS BIGINT)
FROM temp;

DELETE FROM test_table;

SELECT * FROM test_table;
