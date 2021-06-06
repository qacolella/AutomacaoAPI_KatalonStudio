import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testcase.TestCaseFactory
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testdata.TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import internal.GlobalVariable

import org.openqa.selenium.WebElement
import org.openqa.selenium.WebDriver
import org.openqa.selenium.By

import com.kms.katalon.core.mobile.keyword.internal.MobileDriverFactory
import com.kms.katalon.core.webui.driver.DriverFactory

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.TestObjectProperty

import com.kms.katalon.core.mobile.helper.MobileElementCommonHelper
import com.kms.katalon.core.util.KeywordUtil

import com.kms.katalon.core.webui.exception.WebElementNotFoundException

import cucumber.api.java.pt.E
import cucumber.api.java.pt.Dado
import cucumber.api.java.pt.Então
import cucumber.api.java.pt.Quando

import io.cucumber.datatable.DataTable

class ConsultaPDV {

	@Dado("que realizo a requisição {string}")
	public void que_realizo_a_requisição_Consulta_PDV(String string) {

		CustomKeywords.'apis.ações.RealizarRequisição'(string)
		
	}
	
	@Quando("retorna o status code {int}")
	public void retorna_o_status_code(Integer int1) {

		CustomKeywords.'apis.ações.VerificarRetornoStatusCode'(int1)
		
	}
	
	@Então("vejo o retorno com as informações de desconto")
	public void vejo_o_retorno_com_as_informações_de_desconto(DataTable dataTable) {

		List<Map<String, String>> userList =  dataTable.asMaps(String.class, String.class)
		
		CustomKeywords.'apis.ações.VerificarCodigoPromoção'(userList.get(0).get("codigoPromocao"))
		CustomKeywords.'apis.ações.VerificarDescontoValorTotal'(userList.get(0).get("descontoValorTotal"))
		CustomKeywords.'apis.ações.VerificarCódigoProduto'(userList.get(0).get("codigo"))
		CustomKeywords.'apis.ações.VerificarDesconto'(userList.get(0).get("desconto"))
		CustomKeywords.'apis.ações.VerificarNomePromocao'(userList.get(0).get("nomePromocao"))
		
		
	}
	
	
}