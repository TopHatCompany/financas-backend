-- Your SQL goes here

INSERT INTO transactions (currency,label,kind,transacted_date,amount,account,description,opts)
VALUES

('BRL','Sandro','entrada',to_date('30/05/2023','DD/MM/YYYY'),1000,'Reserva de emergencia','entrada','Nubank'),
('BRL','Transferência','aporte',to_date('30/05/2023','DD/MM/YYYY'),1204,'Reserva de emergencia','aporte','Nubank'),
('BRL','Sandro','entrada',to_date('10/06/2023','DD/MM/YYYY'),1000,'Reserva de emergencia','entrada','Nubank'),
('BRL','Transferência','saida',to_date('15/06/2023','DD/MM/YYYY'),-1000,'Reserva de emergencia','resgate','Nubank'),

('BRL','Transferência','saida',to_date('26/06/2023','DD/MM/YYYY'),-7899,'Reserva de emergencia','resgate','celular'),
('BRL','Transferência','entrada',to_date('27/06/2023','DD/MM/YYYY'),4582.3,'Reserva de emergencia','entrada','Nubank'),

('BRL','Transferência','entrada',to_date('27/06/2023','DD/MM/YYYY'),4726.81,'Reserva de emergencia','entrada','Nubank'),
('BRL','Transferência','aporte',to_date('30/05/2023','DD/MM/YYYY'),1210,'Reserva de emergencia','aporte','Nubank'),

('BRL','Transferência','aporte',to_date('30/05/2023','DD/MM/YYYY'),304.37,'Reserva de emergencia','aporte','Nubank')
;
--
-- TODO
-- NUBANK card
-- NUBANK accoutn
