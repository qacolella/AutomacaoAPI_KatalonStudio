<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>API de consulta de PDV por promoção</description>
   <name>ConsultaPDV_Fixo</name>
   <tag></tag>
   <elementGuidId>333bdf08-d8c5-46d0-ae6a-81bbbc33fe7b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;codigoCupomPromocional\&quot;: null,\n    \&quot;formaPagamento\&quot;: null,\n    \&quot;codigoPromocional\&quot;: null,\n    \&quot;modalidade\&quot;: \&quot;1\&quot;,\n    \&quot;identificadorVenda\&quot;: {\n        \&quot;codVenda\&quot;: \&quot;3687\&quot;,\n        \&quot;codFilial\&quot;: \&quot;81716587\&quot;,\n        \&quot;codCanalVenda\&quot;: \&quot;PDV\&quot;,\n        \&quot;codExternoCanalVenda\&quot;: \&quot;1\&quot;\n    },\n    \&quot;produtos\&quot;: [\n        {\n            \&quot;posicaoItem\&quot;: 1,\n            \&quot;codProduto\&quot;: 2000000,\n            \&quot;codEmbalagem\&quot;: 1,\n            \&quot;qtd\&quot;: \&quot;15\&quot;,\n            \&quot;descontoManual\&quot;: 0,\n            \&quot;valorUnitario\&quot;: \&quot;5.90\&quot;\n        }] \n      }         &quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <katalonVersion>7.8.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://localhost:8181/motor2/api/v1/venda/calcularPromocao</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
