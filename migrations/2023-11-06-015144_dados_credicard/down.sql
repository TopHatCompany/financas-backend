-- This file should undo anything in `up.sql`
DELETE FROM transactions
       WHERE transacted_date BETWEEN '2023-01-26' AND '2023-04-28'
       AND account = 'Credicard card'
