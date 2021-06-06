#language: pt
Funcionalidade: Consulta PDV

  Cenário: Verificar Requisição Consulta PDV pelo Número PDV
    Dado que realizo a requisição "Object Repository/ConsultaPDV"
    Quando retorna o status code 200
    Então vejo o retorno com as informações de desconto
      | codigoPromocao     | descontoValorTotal | nomePromocao       | codigo  | desconto |
      | [QA] 7 - ParaCada6 |               0.04 | Para Cada 6 Sai 10 | 2000000 |    50.76 |
      
