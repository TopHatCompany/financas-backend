-- This file should undo anything in `up.sql`
DELETE FROM transactions
       WHERE transacted_date BETWEEN '2023/01/29' AND '2023-06-30'
       AND account = 'Caixinha celular'
