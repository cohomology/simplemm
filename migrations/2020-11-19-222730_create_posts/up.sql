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
  secret VARCHAR(16) NOT NULL,
  PRIMARY KEY(list_id, email),
  CONSTRAINT `lists`
    FOREIGN KEY (list_id) REFERENCES mailing_lists (id) ON DELETE CASCADE ON UPDATE RESTRICT
);

CREATE TABLE messages (
  id INTEGER NOT NULL PRIMARY KEY AUTO_INCREMENT,
  list_id INTEGER NOT NULL,
  email VARCHAR(50) NOT NULL,
  received TIMESTAMP NOT NULL,
  message LONGTEXT NOT NULL,
  send TIMESTAMP NULL DEFAULT NULL,
  CONSTRAINT `user`
    FOREIGN KEY (list_id, email) REFERENCES users (list_id, email) ON DELETE CASCADE ON UPDATE RESTRICT 
);
