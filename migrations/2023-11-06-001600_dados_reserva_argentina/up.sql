-- Your SQL goes here

INSERT INTO transactions (currency,label,kind,transacted_date,amount,account,description,opts)
VALUES
('BRL','Transferência','aporte','2023-05-28',200,'Reserva argentina','aporte','Nubank'),
('BRL','Transferência','aporte','2023-06-01',200,'Reserva argentina','aporte','Nubank'),
('BRL','Transferência','aporte','2023-07-01',200,'Reserva argentina','aporte','Nubank'),
('BRL','Transferência','aporte','2023-08-30',200,'Reserva argentina','aporte','Nubank'),
('BRL','Transferência','aporte','2023-09-01',200,'Reserva argentina','aporte','Nubank'),
('BRL','Transferência','resgate','2023-09-18',-100,'Reserva argentina','resgate','Nubank'),
('BRL','Transferência','aporte','2023-09-20',200,'Reserva argentina','aporte','Nubank'),
('BRL','Transferência','aporte','2023-09-28',100,'Reserva argentina','aporte','Nubank'),
('BRL','Transferência','aporte','2023-10-30',200,'Reserva argentina','aporte','Nubank')

;
-- TODO
