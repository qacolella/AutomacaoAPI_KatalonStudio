#language: pt
Funcionalidade: Consulta PDV

  Cenário: Verificar Requisição Consulta PDV pelo Número PDV
    Dado que informo no cabeçalho da requisição os dados
      | codigoCupomPromocional | formaPagamento | codigoPromocional | modalidade | codVenda | codFilial | codCanalVenda | codExternoCanalVenda |
      | null                   | null           | null              |          1 |     3687 |  81716587 | PDV           |                    1 |
    E informo os dados do produto
      | posicaoItem | codProduto | codEmbalagem | qtd | descontoManual | valorUnitario |
      |           1 |    2000000 |            1 |  15 |              0 |          5.90 |
    E que realizo a requisição "Object Repository/ConsultaPDV"
    Quando retorna o status code 200
    Então vejo o retorno com as informações de desconto
      | codigoPromocao     | descontoValorTotal | nomePromocao       | codigo  | desconto |
      | [QA] 7 - ParaCada6 |               0.04 | Para Cada 6 Sai 10 | 2000000 |    50.76 |
 
      
    Cenário: Verificar Requisição Consulta PDV com número de PDV Inválido
    Dado que informo no cabeçalho da requisição os dados
      | codigoCupomPromocional | formaPagamento | codigoPromocional | modalidade | codVenda | codFilial | codCanalVenda | codExternoCanalVenda |
      | null                   | null           | null              |          null |     3687 |  81716587 | PDV           |                    1 |
    E informo os dados do produto
      | posicaoItem | codProduto | codEmbalagem | qtd | descontoManual | valorUnitario |
      |           1 |    10 |            1 |  15 |              0 |          5.90 |
    E que realizo a requisição "Object Repository/ConsultaPDV"
    Quando retorna o status code 200
    Então vejo o retorno com as informações de desconto
      | codigoPromocao     | descontoValorTotal | nomePromocao       | codigo  | desconto |
      | [QA] 7 - ParaCada6 |               0.04 | Para Cada 6 Sai 10 | 2000000 |    50.76 |    