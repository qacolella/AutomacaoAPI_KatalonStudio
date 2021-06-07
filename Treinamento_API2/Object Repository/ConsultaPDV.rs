<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>API de consulta de PDV por promoção</description>
   <name>ConsultaPDV</name>
   <tag></tag>
   <elementGuidId>f6ab3c67-5471-4c15-a8d2-155ffdb4d2f3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;codigoCupomPromocional\&quot;: ${codigoCupomPromocional},\n    \&quot;formaPagamento\&quot;: ${formaPagamento},\n    \&quot;codigoPromocional\&quot;: ${codigoPromocional},\n    \&quot;modalidade\&quot;: ${modalidade},\n    \&quot;identificadorVenda\&quot;: {\n        \&quot;codVenda\&quot;: ${codVenda},\n        \&quot;codFilial\&quot;: ${codFilial},\n        \&quot;codCanalVenda\&quot;: \&quot;PDV\&quot;,\n        \&quot;codExternoCanalVenda\&quot;: ${codExternoCanalVenda}\n    },\n    \&quot;produtos\&quot;: [\n        {\n            \&quot;posicaoItem\&quot;: ${posicaoItem},\n            \&quot;codProduto\&quot;: ${codProduto},\n            \&quot;codEmbalagem\&quot;: ${codEmbalagem},\n            \&quot;qtd\&quot;: ${qtd},\n            \&quot;descontoManual\&quot;: ${descontoManual},\n            \&quot;valorUnitario\&quot;: ${valorUnitario}\n        }] \n      }         &quot;,
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
   <variables>
      <defaultValue>GlobalVariable.codigoCupomPromocional</defaultValue>
      <description></description>
      <id>24ca528a-ef89-4ae3-8f46-04f9d61e625a</id>
      <masked>false</masked>
      <name>codigoCupomPromocional</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.formaPagamento</defaultValue>
      <description></description>
      <id>61a4f063-fcaa-4f16-af92-a45099a9f853</id>
      <masked>false</masked>
      <name>formaPagamento</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.codigoPromocional</defaultValue>
      <description></description>
      <id>ccbc4ee6-db46-44b2-a66e-295283e9a4f5</id>
      <masked>false</masked>
      <name>codigoPromocional</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.modalidade</defaultValue>
      <description></description>
      <id>13157af2-6f0e-44b6-839f-dab39a9cfbb6</id>
      <masked>false</masked>
      <name>modalidade</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.codVenda</defaultValue>
      <description></description>
      <id>39e93f8d-f9f5-4651-8eee-e6bfb71a5a0f</id>
      <masked>false</masked>
      <name>codVenda</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.codFilial</defaultValue>
      <description></description>
      <id>4f3832c6-0c15-4b0a-aac9-0bf9be602f98</id>
      <masked>false</masked>
      <name>codFilial</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.codCanalVenda</defaultValue>
      <description></description>
      <id>6a76b2e1-5dce-4b13-a896-2468ee300b62</id>
      <masked>false</masked>
      <name>codCanalVenda</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.codExternoCanalVenda</defaultValue>
      <description></description>
      <id>334c57d0-566e-4b36-92ef-e420fadb79aa</id>
      <masked>false</masked>
      <name>codExternoCanalVenda</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.posicaoItem</defaultValue>
      <description></description>
      <id>d1f337c1-b705-4f97-8570-e08d5f8af41d</id>
      <masked>false</masked>
      <name>posicaoItem</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.codProduto</defaultValue>
      <description></description>
      <id>0e7028c9-419b-4095-b703-93eed9ce65ac</id>
      <masked>false</masked>
      <name>codProduto</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.codEmbalagem</defaultValue>
      <description></description>
      <id>cf02da14-35cf-46d2-a32a-2a8749c0a67f</id>
      <masked>false</masked>
      <name>codEmbalagem</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.qtd</defaultValue>
      <description></description>
      <id>95c71446-3a07-48fd-a510-6a9926f8ddef</id>
      <masked>false</masked>
      <name>qtd</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.descontoManual</defaultValue>
      <description></description>
      <id>13481931-7956-45ee-91d9-53cf92e81d6b</id>
      <masked>false</masked>
      <name>descontoManual</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.valorUnitario</defaultValue>
      <description></description>
      <id>856daa5a-22bf-4be1-b64f-3cd1c11688ec</id>
      <masked>false</masked>
      <name>valorUnitario</name>
   </variables>
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
