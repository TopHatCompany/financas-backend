-- This file should undo anything in `up.sql`
DELETE FROM transactions
       WHERE transacted_date BETWEEN '2023/05/29' AND '2023-11-02'
       AND account = 'Reserva de emergência'
