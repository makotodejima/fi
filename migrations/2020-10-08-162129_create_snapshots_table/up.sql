-- Your SQL goes here
CREATE TABLE snapshots(
   id SERIAL PRIMARY KEY,
   account_id VARCHAR NOT NULL,
   date date NOT NULL,
   amount INT NOT NULL,
   CONSTRAINT fk_account
      FOREIGN KEY(account_id) 
	  REFERENCES accounts(id)
);
