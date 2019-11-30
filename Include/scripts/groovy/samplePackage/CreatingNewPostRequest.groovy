package samplePackage
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory
import com.kms.katalon.core.main.CustomKeywordDelegatingMetaClass
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testcase.TestCaseFactory
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testdata.TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords

import internal.GlobalVariable

import MobileBuiltInKeywords as Mobile
import WSBuiltInKeywords as WS
import WebUiBuiltInKeywords as WebUI

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

import cucumber.api.java.en.And
import cucumber.api.java.en.Given
import cucumber.api.java.en.Then
import cucumber.api.java.en.When


class CreatingNewPostRequest {
	/**
	 * The step definitions below match with Katalon sample Gherkin steps
	 */
	@Given("(\\d+) And (.*) Book Obeject Properties Are Available For Example")
	def checkingBookIDAndTitle (int id, String name) {
//		println id
//		println name
		
		if(id != null) {
			println id
		} else {
			KeywordUtil.markError("Non Empty ID Is Not Allowed")
		}
		
		if (name == null) {
			KeywordUtil.markError("Non Empty Title Is Not Allowed")
		} else {
			println name
		}		
	}

	@When("All Object Properties Are (\\d+) (.*) (.*) (.*) (.*) Ready To Use For Making This POST API Request")
	def makingAPIRequestReady(int value, String title, String description, String excerpt, String date) {
		println value
		println title
		println description
		println excerpt
		println date
		
		ResponseObject responseStatus = WS.sendRequest(findTestObject('REST Example/Books Requests/Post New Book Request', [('book_id') : value, ('book_title') : title, ('book_description') : description, 
			('book_excerpt') : excerpt, ('books_published') : date]))
		
		String statusCode = responseStatus.getStatusCode()
		
		// Was Trying Custom Keywords. Checkout verifyingCode Function For Demonstration.
		//TestObject testObject = new TestObject()
		//RequestObject requestObject = (RequestObject)testObject
		//responseStatus = WSBuiltInKeywords.sendRequest(requestObject)
		
		def customKeyword =  new SampleCustomKeywordsExample()
		customKeyword.successfulRequestStatusCode(findTestObject('REST Example/Books Requests/Get Specefic Book Request', [('book_id') : value]))
		
		verifyingCode(statusCode)
	}

	@Then("I Verify REST API Post Request Status Code (.*)")
	def verifyingCode(String status) {
		
		println status
		
		//CustomKeywordDelegatingMetaClass.
		//CustomKeywords.'samplePackage.SampleCustomKeywordsExample.successfulRequestStatusCode'(response)
		
//		TestObject request
//		RequestObject requestObject = (RequestObject) request
//		ResponseObject response = WSBuiltInKeywords.sendRequest(requestObject)
//		
//		def paramTypes = new SampleCustomKeywordsExample(response)
//		paramTypes.successfulRequestStatusCode() 
		
		
		// Success Code Extraction For Specific Book Request Object
		def customKeyword =  new SampleCustomKeywordsExample()
//		ResponseObject responseStatus =  customKeyword.successfulRequestStatusCode(findTestObject('REST Example/Books Requests/Get Specefic Book Request', [('book_id') : 2]))
//		//println responseStatus.getStatusCode()
//		println responseStatus
		ResponseObject responseStatus =  customKeyword.verifyStatusCode(findTestObject('REST Example/Books Requests/Get Specefic Book Request', [('book_id') : 2]), 200)
		//println responseStatus.getStatusCode()
//		String codeReturned = responseStatus.getStatusCode()
//		println ("Code Retyurned : "+codeReturned)
	}

}