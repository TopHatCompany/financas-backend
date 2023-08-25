-- Your SQL goes here
INSERT INTO transactions (currency,label,kind,transacted_date,amount,account,description,opts)
VALUES
('BRL','Entrada','desconto',to_date('22/04/2023','DD/MM/YYYY'),-4.46,'Carrefour card','desconto cartao carrefour',''),
('BRL','Saida','compra',to_date('19/05/2023','DD/MM/YYYY'),190.01,'Carrefour card','PSA posto santo a','abastecimento'),
('BRL','Entrada','pagamento',to_date('30/05/2023','DD/MM/YYYY'),-198.30,'Carrefour card','pagamento fatura via pix',''),
('BRL','Saida','compra',to_date('08/06/2023','DD/MM/YYYY'),174.62,'Carrefour card','PSA posto santo a','abastecimento'),

('BRL','Entrada','desconto',to_date('19/05/2023','DD/MM/YYYY'),-3.77,'Carrefour card','desconto cartao carrefour',''),
('BRL','Saida','compra',to_date('25/06/2023','DD/MM/YYYY'),209.61,'Carrefour card','PSA posto santo a','abastecimento'),
('BRL','Entrada','pagamento',to_date('30/06/2023','DD/MM/YYYY'),-187.37,'Carrefour card','pagamento fatura via pix','')

;

-- 2023-07-08
-- ('BRL','TODO','TODO',to_date('24/12/3333','DD/MM/YYYY'),-00.00,'aaaaaa','aaa','')
