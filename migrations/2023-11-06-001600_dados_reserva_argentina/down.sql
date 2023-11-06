-- This file should undo anything in `up.sql`
DELETE FROM transactions
       WHERE transacted_date BETWEEN '2023/05/27' AND '2023-10-31'
       AND account = 'Reserva argentina'
