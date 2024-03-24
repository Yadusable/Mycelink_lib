CREATE TABLE database_metadata
(
    schema_version INT NOT NULL
);

CREATE TABLE IF NOT EXISTS chat_ids
(
    id       BLOB(16) PRIMARY KEY,
    tenant   TEXT NOT NULL,
    protocol TEXT NOT NULL,
    FOREIGN KEY (tenant, protocol) REFERENCES protocol_config_per_tenant (tenant, protocol)
);

CREATE TABLE IF NOT EXISTS direct_chats
(
    id                BLOB(16),
    recipient_pub_key TEXT NOT NULL,
    FOREIGN KEY (id) REFERENCES chat_ids (id)
);

CREATE TABLE IF NOT EXISTS chat_messages
(
    chat_id         BLOB(16),
    message_id      BLOB(16) PRIMARY KEY,
    message_type    TEXT NOT NULL,
    message_content BLOB,
    timestamp       INTEGER,
    FOREIGN KEY (chat_id) REFERENCES chat_ids (id)
);

CREATE TABLE IF NOT EXISTS chat_message_reactions
(
    root_message_id     BLOB(16),
    reaction_message_id BLOB(16),
    FOREIGN KEY (root_message_id) REFERENCES chat_messages (message_id),
    FOREIGN KEY (reaction_message_id) REFERENCES chat_messages (message_id)
);

CREATE TABLE IF NOT EXISTS chat_message_threads
(
    root_message_id   BLOB(16),
    thread_message_id BLOB(16),
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
    config   TEXT NOT NULL,

    PRIMARY KEY (tenant, protocol),
    FOREIGN KEY (tenant) REFERENCES tenants (display_name)
);

CREATE TABLE IF NOT EXISTS contacts
(
    display_name       TEXT NOT NULL,
    tenant             TEXT NOT NULL,
    protocol           TEXT NOT NULL,
    connection_details TEXT NOT NULL,

    PRIMARY KEY (tenant, protocol, connection_details),
    FOREIGN KEY (tenant, protocol) REFERENCES protocol_config_per_tenant (tenant, protocol)

);