-- Your SQL goes here
CREATE TABLE channels (
    id             INTEGER  PRIMARY KEY
                          NOT NULL,
    title          VARCHAR  NOT NULL
                          DEFAULT (''),
    description    VARCHAR  NOT NULL
                          DEFAULT (''),
    feed_url       VARCHAR  NOT NULL
                          DEFAULT (''),
    link           VARCHAR  NOT NULL
                          DEFAULT (''),
    ttl            INTEGER  NOT NULL
                          DEFAULT (1000),
    favicon        VARCHAR  NOT NULL
                          DEFAULT (''),
    category       VARCHAR  NOT NULL
                          DEFAULT (''),
    tag            VARCHAR  NOT NULL
                          DEFAULT (''),
    create_date    DATETIME NOT NULL
                          DEFAULT (''),
    update_date    DATETIME NOT NULL
                          DEFAULT (''),
    last_sync_date DATETIME NOT NULL
                          DEFAULT (''),
    CONSTRAINT UQ_9a971c99f8dc817fc66aa304b7a UNIQUE (
        feed_url
    )
);

CREATE TABLE articles (
    id          INTEGER  PRIMARY KEY
                         NOT NULL,
    title       VARCHAR  NOT NULL
                         DEFAULT (''),
    link        VARCHAR  NOT NULL
                         DEFAULT (''),
    description VARCHAR  NOT NULL
                         DEFAULT (''),
    author      VARCHAR  NOT NULL
                         DEFAULT (''),
    content     VARCHAR  NOT NULL
                         DEFAULT (''),
    category    INTEGER  NOT NULL
                         DEFAULT (1000),
    comments    VARCHAR  NOT NULL
                         DEFAULT (''),
    pub_date     DATETIME NOT NULL
                         DEFAULT (''),
    create_date  DATETIME NOT NULL
                         DEFAULT (''),
    update_date  DATETIME NOT NULL
                         DEFAULT (''),
    has_read     INTEGER  NOT NULL
                         DEFAULT (0),
    is_like      INTEGER  NOT NULL
                         DEFAULT (0),
    channel_id   VARCHAR NOT NULL,
    CONSTRAINT UQ_061c2abbafa91e39bfaf4ba59a4 UNIQUE (
        link
    ),
    CONSTRAINT FK_c43fc32d50ee689b016923d06a4 FOREIGN KEY (
        channel_id
    )
    REFERENCES channel (id) ON DELETE NO ACTION
                            ON UPDATE NO ACTION
);

