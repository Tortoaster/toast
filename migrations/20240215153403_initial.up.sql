CREATE TABLE projects
(
    id           VARCHAR(128) PRIMARY KEY NOT NULL,
    name         VARCHAR(32)              NOT NULL,
    preview      VARCHAR(300)             NOT NULL,
    thumbnail_id UUID                     NOT NULL,
    project_url  VARCHAR(2000),
    date_posted  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT current_timestamp,
    deleted      BOOLEAN                  NOT NULL DEFAULT FALSE
);
