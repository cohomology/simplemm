CREATE TABLE mailing_lists (
  id INTEGER NOT NULL PRIMARY KEY AUTO_INCREMENT, 
  title VARCHAR(100) NOT NULL, 
  email VARCHAR(50) NOT NULL, 
  enabled BOOLEAN NOT NULL DEFAULT true
);

CREATE TABLE users (
  list_id INTEGER NOT NULL,
  email VARCHAR(50) NOT NULL,
  password VARCHAR(50) NOT NULL,
  enabled BOOLEAN NOT NULL DEFAULT false,
  PRIMARY KEY(list_id, email),
  CONSTRAINT `user_to_lists`
    FOREIGN KEY (list_id) REFERENCES mailing_lists (id) ON DELETE CASCADE ON UPDATE RESTRICT
);

CREATE TABLE inbound_messages (
  id INTEGER NOT NULL PRIMARY KEY AUTO_INCREMENT,
  list_id INTEGER NOT NULL,
  email VARCHAR(50) NOT NULL,
  received TIMESTAMP NOT NULL,
  message LONGTEXT NOT NULL,
  CONSTRAINT `inbound_messages_to_user`
    FOREIGN KEY (list_id, email) REFERENCES users (list_id, email) ON DELETE CASCADE ON UPDATE RESTRICT 
);

CREATE TABLE outbound_messages (
  id INTEGER NOT NULL PRIMARY KEY AUTO_INCREMENT,
  inbound_id INTEGER NOT NULL,
  send TIMESTAMP NOT NULL,
  message LONGTEXT NOT NULL,
  CONSTRAINT `outbound_to_inbound_messages`
    FOREIGN KEY (inbound_id) REFERENCES inbound_messages (id) ON DELETE CASCADE ON UPDATE RESTRICT 
); 

CREATE TABLE secrets (
  id INTEGER NOT NULL PRIMARY KEY AUTO_INCREMENT,
  list_id INTEGER NOT NULL,
  email VARCHAR(50) NOT NULL, 
  secret VARCHAR(16) NOT NULL,
  valid_to TIMESTAMP NOT NULL,
  CONSTRAINT `secrets_to_user`
    FOREIGN KEY (list_id,email) REFERENCES users (list_id,email) ON DELETE CASCADE ON UPDATE RESTRICT  
);
