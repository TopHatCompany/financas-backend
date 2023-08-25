-- Your SQL goes here
INSERT INTO transactions (currency,label,kind,transacted_date,amount,account,description,opts)
VALUES
('BRL','Sandro','entrada',to_date('09/05/2023','DD/MM/YYYY'),600.00,'Itau','pix',''),
('BRL','Sandro','entrada',to_date('09/05/2023','DD/MM/YYYY'),500.00,'Itau','pix',''),
('BRL','Sandro','entrada',to_date('09/05/2023','DD/MM/YYYY'),400.00,'Itau','pix',''),
('BRL','Sandro','entrada',to_date('10/05/2023','DD/MM/YYYY'),340.00,'Itau','pix',''),
('BRL','Sandro','entrada',to_date('10/05/2023','DD/MM/YYYY'),1450.00,'Itau','pix',''),
('BRL','Transferência','saida',to_date('10/05/2023','DD/MM/YYYY'),600.00,'Itau','pix','Nubank'),
('BRL','Sandro','entrada',to_date('10/05/2023','DD/MM/YYYY'),600.00,'Nubank','pix',''),
('BRL','Transferência','saida',to_date('10/05/2023','DD/MM/YYYY'),-600.00,'Nubank','pix','Itau'),
('BRL','Saida','pagamento',to_date('10/05/2023','DD/MM/YYYY'),-3804.44,'Itau','aluguel',''),
('BRL','Sandro','entrada',to_date('16/05/2023','DD/MM/YYYY'),300,'Itau','pix',''),
('BRL','Transferência','saida',to_date('17/05/2023','DD/MM/YYYY'),-385.56,'Itau','pix','Nubank'),
('BRL','Transferência','entrada',to_date('18/05/2023','DD/MM/YYYY'),497.50,'Itau','pix','Nubank'),
('BRL','Saida','pix',to_date('28/04/2023','DD/MM/YYYY'),-200,'Nubank','Danilo',''),
('BRL','Transferência','saida',to_date('02/05/2023','DD/MM/YYYY'),-3000,'Nubank','boleto','Amortização'),
('BRL','Emily','entrada',to_date('04/05/2023','DD/MM/YYYY'),71,'Nubank','pix',''),
('BRL','Saida','pagamento',to_date('06/05/2023','DD/MM/YYYY'),-17.6,'Nubank','pix','Top'),
('BRL','Saida','transferência',to_date('07/05/2023','DD/MM/YYYY'),-125,'Nubank','Isabella','pix'),
('BRL','Entrada','transferência',to_date('07/05/2023','DD/MM/YYYY'),125,'Nubank','Isabella','pix'),
('BRL','Entrada','transaferência',to_date('15/05/2023','DD/MM/YYYY'),32,'Nubank','Isabella','pix'),
('BRL','Saida','pagamento',to_date('15/05/2023','DD/MM/YYYY'),-20,'Nubank','recarga','claro sp'),
('BRL','Transferência','pagamento',to_date('15/05/2023','DD/MM/YYYY'),-32,'Nubank','pagamento','Nubank card'),
('BRL','Transferência','pagamento',to_date('15/05/2023','DD/MM/YYYY'),32,'Nubank card','pagamento','Nubank'),
('BRL','Transferência','resgate',to_date('15/05/2023','DD/MM/YYYY'),-2000,'Reserva de emergencia','pagamento','Nubank'),
('BRL','Transferência','resgate',to_date('15/05/2023','DD/MM/YYYY'),2000,'Nubank','pagamento','Reserva de emergencia'),
('BRL','Sandro','saida',to_date('15/05/2023','DD/MM/YYYY'),-2000,'Nubank','pix',''),
('BRL','Sandro','entrada',to_date('16/05/2023','DD/MM/YYYY'),700,'Nubank','pix','pagamento'),
('BRL','Renata','saida',to_date('17/05/2023','DD/MM/YYYY'),-50,'Nubank','pix','Selma'),
('BRL','Transferência','entrada',to_date('17/05/2023','DD/MM/YYYY'),385.56,'Nubank','pix',''),
('BRL','Transferência','aporte',to_date('17/05/2023','DD/MM/YYYY'),2000,'Reserva de emergencia','pagamento','Nubank'),
('BRL','Transferência','aporte',to_date('17/05/2023','DD/MM/YYYY'),-2000,'Nubank','pagamento','Reserva de emergencia'),
('BRL','Transferência','saida',to_date('18/05/2023','DD/MM/YYYY'),-497.50,'Nubank','pix','Itau'),

('BRL','Saida','compra',to_date('20/05/2023','DD/MM/YYYY'),-12,'Nubank','porao',''),
('BRL','Entrada','transferência',to_date('20/05/2023','DD/MM/YYYY'),17,'Nubank','Isabella','pix'),
('BRL','Saida','reembolso',to_date('20/05/2023','DD/MM/YYYY'),-17,'Nubank','Isabella','pix'),
('BRL','Emily','saida',to_date('20/05/2023','DD/MM/YYYY'),-42,'Nubank','saida',''),

('BRL','Renata','entrada',to_date('22/05/2023','DD/MM/YYYY'),300,'Nubank','pix','danilo'),
('BRL','Renata','saida',to_date('22/05/2023','DD/MM/YYYY'),-180,'Nubank','pix','Sandro'),
('BRL','Renata','saida',to_date('22/05/2023','DD/MM/YYYY'),-120,'Nubank','pix','Selma'),


('BRL','Emily','saida',to_date('24/05/2023','DD/MM/YYYY'),-49.73,'Nubank','pix','')

;

-- ('BRL','Transferência','pagamento',to_date('27/04/2023','DD/MM/YYYY'),-1031.24,'Nubank','fatura','Carrefour card'),
-- ('BRL','Emily','saida',to_date('24/05/2023','DD/MM/YYYY'),-49.73,'Nubank','pix','')
-- TODO
-- CAIXINHA CELULAR
-- RESERVA DE EMERGENCIA
-- PORQUINHO
-- CREDICARD
-- NUBANK CARD
--
