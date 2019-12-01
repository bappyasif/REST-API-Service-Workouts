package anotherPackage
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory
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


class UpdatingBookContentsRequest {
	/**
	 * The step definitions below match with Katalon sample Gherkin steps
	 */
	@Given("(\\d+) And (.*) Book Obeject Properties Are Available For Update Request")
	def bookDetails(int value, String name) {
		println value
		println name
	}

	@When("All Object Properties Are (\\d+) (.*) (.*) (.*) (.*) Ready To Use For Making This Update API Request")
	def updatingBookContentsResponse(int value, String title, String description, String excerpt, String date) {
		println value
		println title
		println description
		println excerpt
		println date
		
		ResponseObject responseObject =  WS.sendRequest(findTestObject('REST Example/Books Requests/Update Specefic Book Request'
			, [('book_id') : value
			, ('book_title') : title
			, ('book_description') : description
			, ('book_excerpt') : excerpt
			, ('book_published') : date]))
		
		String statusCode = responseObject.getStatusCode()
		
		verifyStatusCodeResponse(statusCode)

	}

	@Then("I Verify REST API Update Request Status Code (.*)")
	def verifyStatusCodeResponse(String status) {
		println status
	}
	
	@Then("I Also Checkout Some Custom Keywords Usage Examples")
	def customKeywords() {
		def customKeywords = new samplePackage.AnotherCustomKeywordsExample()
		TestObject testObject = findTestObject('REST Example/Books Requests/Get Specefic Book Request')
		//customKeywords.verifyStatusCode(testObject, 0)
		customKeywords.bookContains(testObject)
		customKeywords.bookSpeceficContentExtraction(testObject)
	}
}