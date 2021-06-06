package apis

import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.util.KeywordUtil
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows

import internal.GlobalVariable

import groovy.json.JsonSlurper


public class ações {
	
	@Keyword
	def RealizarRequisição(caminhoObjeto){
		
		def requisição = WS.sendRequest(findTestObject(caminhoObjeto))
		
		def codretorno = requisição.getStatusCode()
		GlobalVariable.statusCodigoRetorno = codretorno
		KeywordUtil.markWarning("Status Code é igual a " + GlobalVariable.statusCodigoRetorno)
		
		def retorno = requisição.getResponseText()
		def xmlNode
		xmlNode = new XmlSlurper().parseText(retorno) 
		GlobalVariable.retornoRequisicao = xmlNode
		KeywordUtil.markWarning("Get do texto dentro do body do retorno da requisição " + GlobalVariable.retornoRequisicao)
		
	}
	
	@Keyword
	def VerificarRetornoStatusCode(text){
		
		if(text == GlobalVariable.statusCodigoRetorno){
			
			KeywordUtil.markPassed("Passou: Retornou o status code esperado como " + text)
					
		} else {
		
			KeywordUtil.markFailedAndStop("Falhou: Não retorno o status code esperado como " + text + ". Retornou o status code como " + GlobalVariable.statusCodigoRetorno)
		} 

	}
	
	
	@Keyword
	def VerificarCodigoPromoção(text){
		
		def var = GlobalVariable.retornoRequisicao.beneficioPromocao.codigoPromocao.text()
		
		if(text == var){
			
			KeywordUtil.markPassed("Passou: Retornou o codigoPromocao esperado como " + text)
			
					
		} else {
		
			KeywordUtil.markFailed("Falhou: Não retorno o codigoPromocao esperado como " + text + ". Retornou o codigoPromocao como " + var)
		} 

	}
	
	@Keyword
	def VerificarDescontoValorTotal(text){
		
		def var2 = ConverterInteiroParaString(text)
			
		def var = GlobalVariable.retornoRequisicao.beneficioPromocao.descontoValorTotal.text()
		
		if(var2 == var){
			
			KeywordUtil.markPassed("Passou: Retornou o descontoValorTotal esperado como " + var2)
			
					
		} else {
		
			KeywordUtil.markFailed("Falhou: Não retorno o descontoValorTotal esperado como " + var2 + ". Retornou o codigoPromocao como " + var)
		}

	}
	
	
	@Keyword
	def ConverterInteiroParaString(text){
		
		String var2 = text
		return var2
		
	}
	
	
	@Keyword
	def VerificarNomePromocao(text){
		
		def var = GlobalVariable.retornoRequisicao.beneficioPromocao.nomePromocao.text()
		
		if(text == var){
			
			KeywordUtil.markPassed("Passou: Retornou o nomePromocao esperado como " + text)
			
					
		} else {
		
			KeywordUtil.markFailed("Falhou: Não retorno o nomePromocao esperado como " + text + ". Retornou o nomePromocao como " + var)
		}

	}
	
	@Keyword
	def VerificarCódigoProduto(text){
		
		def var2 = ConverterInteiroParaString(text)
		
		def var = GlobalVariable.retornoRequisicao.beneficioPromocao.produtos.codigo.text()
		
		if(var2 == var){
			
			KeywordUtil.markPassed("Passou: Retornou o codigoProduto esperado como " + var2)
			
					
		} else {
		
			KeywordUtil.markFailed("Falhou: Não retorno o codigoProduto esperado como " + var2 + ". Retornou o codigoProduto como " + var)
		}

	}
	
	@Keyword
	def VerificarDesconto(text){
		
		def var2 = ConverterInteiroParaString(text)
		
		def var = GlobalVariable.retornoRequisicao.beneficioPromocao.produtos.desconto.text()
		
		if(var2 == var){
			
			KeywordUtil.markPassed("Passou: Retornou o desconto esperado como " + var2)
			
					
		} else {
		
			KeywordUtil.markFailed("Falhou: Não retorno o desconto esperado como " + var2 + ". Retornou o desconto como " + var)
		}

	}
	
	
	
	
}
