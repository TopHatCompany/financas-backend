-- Your SQL goes here
INSERT INTO transactions (currency,label,kind,transacted_date,amount,account,description,opts)
VALUES

('BRL','Transferência','aporte',to_date('30/05/2023','DD/MM/YYYY'),1204,'Caixinha celular','Aporte',''),

('BRL','Entrada','rendimento',to_date('27/06/2023','DD/MM/YYYY'),280.18,'Caixinha celular','rendimento',''),

('BRL','Transferência','resgate',to_date('27/06/2023','DD/MM/YYYY'),-9613.48,'Caixinha celular','Resgate','')

;

-- 2023-07-08
-- ('BRL','TODO','TODO',to_date('24/12/3333','DD/MM/YYYY'),-00.00,'aaaaaa','aaa','')
