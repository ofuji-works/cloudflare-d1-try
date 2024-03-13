DROP TABLE IF EXISTS test_table;

CREATE TABLE test_table
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT ,
    post_id    INT,
    short_text VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    sample_id  INT NOT NULL DEFAULT 0 NOT NULL
);
CREATE INDEX index_test_table_on_post_id ON test_table (post_id);
CREATE INDEX index_test_table_on_sample_id ON test_table (sample_id);

