-- This file should undo anything in `up.sql`
DELETE FROM transactions
       WHERE transacted_date BETWEEN '2023-01-26' AND '2023-05-02'
       AND account = 'Itau card'
