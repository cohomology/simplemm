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
  CONSTRAINT `user_to_list`
    FOREIGN KEY (list_id) REFERENCES mailing_lists (id) ON DELETE CASCADE ON UPDATE RESTRICT
);

CREATE TABLE subscriptions (
  uuid BINARY(16) NOT NULL PRIMARY KEY,
  list_id INTEGER NOT NULL,
  email VARCHAR(50) NOT NULL,
  timestamp TIMESTAMP NOT NULL,
  request TEXT NOT NULL,
  CONSTRAINT `subscription_to_list`
    FOREIGN KEY (list_id) REFERENCES mailing_lists (id) ON DELETE CASCADE ON UPDATE RESTRICT,
  INDEX (list_id)
);
