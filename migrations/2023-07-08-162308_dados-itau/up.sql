-- Your SQL goes here
INSERT INTO transactions (currency,label,kind,transacted_date,amount,account,description,opts)
VALUES

('BRL','Sandro','entrada',to_date('25/05/2023','DD/MM/YYYY'),-780,'Itau','pix',''),

('BRL','Saida','compra',to_date('27/05/2023','DD/MM/YYYY'),-213.84,'Itau','pix','netshoes'),
('BRL','Transferência','saida',to_date('28/05/2023','DD/MM/YYYY'),-566.16,'Itau','pix','Nubank'),

('BRL','Entrada','salário',to_date('30/05/2023','DD/MM/YYYY'),12031.30,'Itau','Salario',''),
('BRL','Transferência','saida',to_date('30/05/2023','DD/MM/YYYY'),-12031.30,'Itau','Salario','Nubank'),


('BRL','Sandro','entrada',to_date('07/06/2023','DD/MM/YYYY'),1000,'Itau','pix','aluguel'),
('BRL','Sandro','entrada',to_date('07/06/2023','DD/MM/YYYY'),1000,'Itau','pix','aluguel'),

('BRL','Sandro','entrada',to_date('08/06/2023','DD/MM/YYYY'),500,'Itau','pix','aluguel'),
('BRL','Sandro','entrada',to_date('09/06/2023','DD/MM/YYYY'),500,'Itau','pix','aluguel'),

('BRL','Transferência','entrada',to_date('09/06/2023','DD/MM/YYYY'),1820,'Itau','pix','Nubank'),
('BRL','Transferência','saida',to_date('09/06/2023','DD/MM/YYYY'),-1000,'Itau','pix','Nubank'),

('BRL','Saida','pagamento',to_date('12/06/2023','DD/MM/YYYY'),-3803.32,'Itau','aluguel',''),

('BRL','Transferência','saida',to_date('01/07/2023','DD/MM/YYYY'),-16.68,'Itau','pix','Nubank'),
('BRL','Transferência','entrada',to_date('03/07/2023','DD/MM/YYYY'),155.23,'Itau','aluguel','Nubank'),
('BRL','Saida','pagamento',to_date('03/07/2023','DD/MM/YYYY'),-155.23,'Itau','licenciamento','imposto'),


('BRL','Entrada','reembolso',to_date('04/07/2023','DD/MM/YYYY'),0.01,'Itau','credito cartao','cartao luiza'),
('BRL','Transfência','entrada',to_date('04/07/2023','DD/MM/YYYY'),-1820,'Itau','pix','Nubank'),

('BRL','Sandro','entrada',to_date('06/07/2023','DD/MM/YYYY'),500,'Itau','pix','aluguel'),
('BRL','Sandro','entrada',to_date('06/07/2023','DD/MM/YYYY'),500,'Itau','pix','aluguel'),
('BRL','Sandro','entrada',to_date('07/07/2023','DD/MM/YYYY'),1000,'Itau','pix','aluguel'),
('BRL','Sandro','entrada',to_date('07/07/2023','DD/MM/YYYY'),500,'Itau','pix','aluguel'),

('BRL','Saida','pagamento',to_date('10/07/2023','DD/MM/YYYY'),-3798.81,'Itau','aluguel','')
;

-- 2023-07-08
-- ('BRL','TODO','TODO',to_date('24/12/3333','DD/MM/YYYY'),-00.00,'aaaaaa','aaa','')
