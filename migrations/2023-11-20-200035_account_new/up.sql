-- Your SQL goes here
do
$$
declare r record;
begin
  for r in select users.id from users
  loop
    INSERT INTO users_accounts (user_id, identification, kind, initial_amount)
    VALUES
    (r.id, 'itau', 'savings', 0.0),
    (r.id, 'nubank', 'savings', 0.0),
    (r.id, 'nubank card', 'credit card', 0.0),
    (r.id, 'itau card', 'credit card', 0.0);
  end loop;
end
$$;
