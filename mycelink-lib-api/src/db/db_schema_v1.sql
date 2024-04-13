CREATE TABLE database_metadata
(
    schema_version INT NOT NULL
);

CREATE TABLE protocols
(
    name TEXT PRIMARY KEY
);


INSERT INTO protocols (name)
VALUES ('Mycelink');

CREATE TABLE IF NOT EXISTS chat_ids
(
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    display_name    TEXT NOT NULL,
    tenant          TEXT NOT NULL,
    protocol        TEXT NOT NULL,
    protocol_config BLOB NOT NULL,
    FOREIGN KEY (tenant, protocol) REFERENCES protocol_config_per_tenant (tenant, protocol),
    FOREIGN KEY (protocol) REFERENCES protocols (name)
);

CREATE TABLE IF NOT EXISTS chat_messages
(
    chat_id               INTEGER NOT NULL,
    message_id            INTEGER PRIMARY KEY AUTOINCREMENT,
    contact_id            INTEGER NOT NULL,
    protocol_message_meta BLOB    NOT NULL,
    message_content       BLOB    NOT NULL,
    timestamp             INTEGER NOT NULL,
    tenant                TEXT    NOT NULL,
    FOREIGN KEY (tenant, chat_id) REFERENCES chat_ids (tenant, id),
    FOREIGN KEY (chat_id) REFERENCES chat_ids (id),
    FOREIGN KEY (contact_id) REFERENCES contacts (id)
);

CREATE TABLE IF NOT EXISTS chat_message_reactions
(
    root_message_id     INTEGER PRIMARY KEY,
    reaction_message_id INTEGER,
    FOREIGN KEY (root_message_id) REFERENCES chat_messages (message_id),
    FOREIGN KEY (reaction_message_id) REFERENCES chat_messages (message_id)
);

CREATE TABLE IF NOT EXISTS chat_message_threads
(
    root_message_id   INTEGER PRIMARY KEY,
    thread_message_id INTEGER,
    FOREIGN KEY (root_message_id) REFERENCES chat_messages (message_id),
    FOREIGN KEY (thread_message_id) REFERENCES chat_messages (message_id)
);

CREATE TABLE IF NOT EXISTS tenants
(
    display_name TEXT PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS protocol_config_per_tenant
(
    tenant   TEXT NOT NULL,
    protocol TEXT NOT NULL,
    config   BLOB NOT NULL,

    PRIMARY KEY (tenant, protocol),
    FOREIGN KEY (tenant) REFERENCES tenants (display_name),
    FOREIGN KEY (protocol) REFERENCES protocols (name)
);

CREATE TABLE IF NOT EXISTS contacts
(
    id                      INTEGER PRIMARY KEY AUTOINCREMENT,
    display_name            TEXT NOT NULL,
    alternative_name        TEXT,
    tenant                  TEXT NOT NULL,
    profile_picture         BLOB,
    low_res_profile_picture BLOB,
    protocol                TEXT NOT NULL,
    connection_details      BLOB NOT NULL,

    FOREIGN KEY (tenant, protocol) REFERENCES protocol_config_per_tenant (tenant, protocol)
);
