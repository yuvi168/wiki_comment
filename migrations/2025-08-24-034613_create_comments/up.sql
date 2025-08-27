Create Table comments(
    id SERIAL PRIMARY Key,
    content TEXT NOT NULL,
    reply VARCHAR,
    upvotes INTEGER NOT NULL DEFAULT 0,
    got_replies INTEGER NOT NULL DEFAULT 0,
    userid VARCHAR NOT NULL,  
    wiki_entry VARCHAR NOT NULL
    --add field title: APPROPRIATE TYPE,
    --add field time: Time STAMP TYPE(I guess)
);
