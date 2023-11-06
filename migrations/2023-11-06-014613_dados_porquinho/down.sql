-- This file should undo anything in `up.sql`
DELETE FROM transactions
       WHERE transacted_date BETWEEN '2023-03-27' AND '2023-04-18'
       AND account = 'Porquinho'
